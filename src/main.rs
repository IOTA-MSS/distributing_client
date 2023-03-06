use crate::library::crypto::Wallet;
use clap::Parser;
use config::{Arguments, Command, Config};
use ethers::{
    types::{Transaction, U64},
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::StreamExt;
use futures::{future::BoxFuture, stream::FuturesUnordered};
use library::{client::TangleTunesClient, crypto, database::Database};
use std::{collections::VecDeque, io::stdin, net::SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime::Runtime,
};
use tracing::warn;

mod config;
mod library;
mod listener;
mod state;
#[macro_use]
extern crate eyre;

fn main() -> eyre::Result<()> {
    Runtime::new().unwrap().block_on(async move {
        color_eyre::install().unwrap();
        _main(Arguments::parse()).await
    })
}

async fn _main(args: Arguments) -> eyre::Result<()> {
    let config = Config::from_file(&args.config_path)?;
    let password = args.password;

    match args.command {
        Command::GenerateWallet {
            plaintext: _,
            password,
        } => {
            let key = Wallet::generate().private_key();
            let db = config.initialize_database().await?;
            set_key_with_confirmation(&db, key, password).await?;
        }
        Command::ImportWallet {
            key,
            password,
            plaintext: _,
        } => {
            let database = config.initialize_database().await?;
            set_key_with_confirmation(&database, key, password).await?;
        }
        Command::ExportAddress => {
            let database = config.initialize_database().await?;
            let wallet = config.decrypt_wallet(&database, password.as_ref()).await?;
            println!("Your address: {:?}", wallet.address());
        }
        Command::RemoveSongs { song_ids } => {
            let db = config.initialize_database().await?;
            for id in &song_ids {
                if db.remove_song(id).await? {
                    println!("Succesfully removed song {id:?}!");
                } else {
                    println!("Song with id {id:?} does not exist");
                }
            }
        }
        Command::AddSongs { song_ids: _ } => {
            todo!()
        }
        Command::Distribute => {
            color_eyre::install()?;
            tracing_subscriber::fmt::init();
            let database = config.initialize_database().await?;
            let wallet = config.decrypt_wallet(&database, password.as_ref()).await?;
            let client = &*Box::leak(Box::new(config.initialize_client(&wallet).await?));

            let listener = TcpListener::bind(("127.0.0.1", config.port)).await.unwrap();

            while let Ok((mut stream, addr)) = listener.accept().await {
                let _ = tokio::task::spawn(async move {
                    if let Err(e) =
                        handle_incoming_tcp_connection(&mut stream, addr, database, client).await
                    {
                        warn!("Handler exited with an error: {}", e);
                    }
                });
            }
        }
        _ => todo!(),
    };

    Ok(())
}

fn ask_confirmation(msg: &str) -> eyre::Result<bool> {
    println!("{msg} [y/N]");
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    if line.starts_with("y") || line.starts_with("Y") {
        println!("Ok!");
        Ok(true)
    } else {
        println!("Canceling...");
        Ok(false)
    }
}

async fn set_key_with_confirmation(
    db: &Database,
    key: String,
    password: Option<String>,
) -> eyre::Result<()> {
    if db.get_key().await?.is_some()
        && !ask_confirmation(
            "Are your sure? Setting a new key will DELETE the key currently in use.",
        )?
    {
        return Ok(());
    }
    let (key, encrypted) = match password {
        Some(password) => (crate::crypto::encrypt_private_key(&key, &password), true),
        None => (key, false),
    };
    db.set_key(&key, encrypted).await?;
    Ok(())
}

async fn handle_incoming_tcp_connection(
    stream: &mut TcpStream,
    _addr: SocketAddr,
    database: Database,
    client: &'static TangleTunesClient,
) -> eyre::Result<()> {
    static DEBT_LIMIT: i32 = 20;
    let mut credit: i32 = DEBT_LIMIT;
    let mut open_requests: VecDeque<(String, i32, i32)> = VecDeque::new();
    let mut pending_transactions = FuturesUnordered::<BoxFuture<'static, eyre::Result<i32>>>::new();
    let mut tcp_buffer = Vec::with_capacity(2048);

    'outer: loop {
        // If there are pending transactions, we have to select both streams.
        // Otherwise, we just read from the stream.
        tcp_buffer.clear();
        let tcp_msg_received = if pending_transactions.is_empty() {
            Some(stream.read(&mut tcp_buffer).await)
        } else {
            tokio::select! {
                biased;

                new_credit = pending_transactions.next() => {
                    credit = credit.checked_add(new_credit.unwrap()?).unwrap();
                    None
                }

                res = stream.read(&mut tcp_buffer) => {
                    Some(res)
                }
            }
        };

        // If we have received a TCP-message, we should handle it
        if let Some(result) = tcp_msg_received {
            // If the TCP-connection is closed, we can stop normally
            if result.is_err() {
                break 'outer;
            }

            // Otherwise, we decode the transaction from the buffer
            let transaction = Transaction::decode(&Rlp::new(&tcp_buffer))?;
            let (song_id, from, amount) = client.decode_chunk_tx_input(&transaction.input)?;

            // And push the request and pending transaction to the lists.
            open_requests.push_back((song_id, from as i32, amount as i32));
            pending_transactions.push(Box::pin(async move {
                let receipt = client.send_raw_tx(&transaction).await?;

                let Some(receipt) = receipt else {
                    Err(eyre!("Didn't receive a receipt"))?
                };
                let Some(status) = &receipt.status else {
                    Err(eyre!("EIP-658 not activated"))?
                };
                if *status != U64::from(1) {
                    Err(eyre!("Transaction status = 0, failure"))?
                };
                // tokio::time::sleep(Duration::from_millis(500)).await;

                Ok(amount as i32)
            }));
        };

        // Now we can send chunks until the credit runs out.
        'inner: while credit > 0 {
            // Check the next open request
            let Some((song_id, from, amount)) = open_requests.front_mut() else {
                continue 'outer;
            };
            // If it is empty, then we choose the next.
            if *amount == 0 {
                open_requests.pop_front().unwrap();
                continue 'inner;
            }

            // Update the current open request, and select how much to stream right now.
            let (amount_now, from_now) = {
                let amount_now = credit.min(*amount);
                let from_now = from.clone();
                credit -= amount_now;
                *amount -= amount_now;
                *from += amount_now;
                (amount_now, from_now)
            };

            // Get the chunks from the database
            let (chunks, _fee) = database
                .read_chunks(song_id, from_now as u32, amount_now as u32)
                .await?;

            // And send them to the client
            stream.write_i32(from_now).await?;
            stream.write_i32(amount_now).await?;
            stream.write(&chunks).await?;
            stream.flush().await?;
        }
    }

    Ok(())
}
