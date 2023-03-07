use std::{collections::VecDeque, net::SocketAddr};

use ethers::{
    types::{Transaction, U64},
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::StreamExt;
use futures::{future::BoxFuture, stream::FuturesUnordered};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::warn;

use crate::library::{client::TangleTunesClient, config::Config, database::Database};

pub async fn run(cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let wallet = cfg.decrypt_wallet(&database).await?;
    let client = &*Box::leak(Box::new(cfg.initialize_client(&wallet).await?));

    let listener = TcpListener::bind(("127.0.0.1", cfg.file.port))
        .await
        .unwrap();

    while let Ok((mut stream, addr)) = listener.accept().await {
        let _ = tokio::task::spawn(async move {
            if let Err(e) =
                handle_incoming_tcp_connection(&mut stream, addr, database, client).await
            {
                warn!("Handler exited with an error: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_incoming_tcp_connection(
    stream: &mut TcpStream,
    _addr: SocketAddr,
    database: Database,
    client: &'static TangleTunesClient,
) -> eyre::Result<()> {
    static DEBT_LIMIT: u32 = 20;

    // The amount of credit available for this client.
    let mut credit: i32 = DEBT_LIMIT as i32;
    // The requests from this client that haven't beel fulfilled yet.
    let mut open_requests: VecDeque<(String, i32, i32)> = VecDeque::new();
    // The transactions that will resolve with the amount of credit gained from them.
    let mut pending_confirmations =
        FuturesUnordered::<BoxFuture<'static, eyre::Result<i32>>>::new();
    let mut tcp_buffer = Vec::with_capacity(2048);

    'outer: loop {
        // If there are pending transactions, we have to select both streams.
        // Otherwise, we just read from the stream.
        tcp_buffer.clear();
        let tcp_msg_received = if pending_confirmations.is_empty() {
            Some(stream.read(&mut tcp_buffer).await)
        } else {
            tokio::select! {
                biased;

                new_credit = pending_confirmations.next() => {
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
            let raw_tx = Transaction::decode(&Rlp::new(&tcp_buffer))?;

            // And attempt to decode it as a get_chunks call
            let decoded_tx = client.decode_get_chunks_call(&raw_tx.input)?;

            // It is only valid if it is our own address!
            if decoded_tx.distributor != client.address() {
                bail!(
                    "Distributor address is not my address!: {}, {}",
                    decoded_tx.distributor,
                    client.address()
                )
            }

            let song_id = hex::encode(decoded_tx.song);
            let from = decoded_tx.index.as_u128().try_into()?;
            let amount = decoded_tx.index.as_u128().try_into()?;

            // And push the request and pending transaction to the lists.
            open_requests.push_back((song_id, from, amount));
            pending_confirmations.push(Box::pin(async move {
                let Some(receipt) = client.send_raw_tx(&raw_tx).await? else {
                    bail!("Didn't receive a receipt")
                };
                let Some(status) = &receipt.status else {
                    bail!("EIP-658 not activated")
                };
                if *status != U64::from(1) {
                    bail!("Transaction-status == 0 --> failure")
                };

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
                .get_chunks(song_id, from_now as u32, amount_now as u32)
                .await?;

            // And send them to the client
            stream.write_i32(from_now).await?; // todo: use buffered writer
            stream.write_i32(amount_now).await?;
            stream.write(&chunks).await?;
            stream.flush().await?;
        }
    }

    Ok(())
}
