mod background;
mod register;

use crate::{
    distribute::{
        background::{auto_distribute_added_songs, auto_download_new_songs, exit_listener},
        register::{deregister_for_songs, register_for_songs},
    },
    library::{
        abi::GetChunksCall,
        app::AppData,
        tcp::{RequestChunksDecoder, SendChunksEncoder},
        transaction_pool::TransactionPool,
        util::TransactionReceiptExt,
    },
};
use ethers::types::Bytes;
use ethers_providers::StreamExt;
use futures::SinkExt;
use std::{collections::VecDeque, convert::Infallible, net::SocketAddr, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
    time::sleep,
};
use tokio_util::codec::{FramedRead, FramedWrite};

const DEBT_LIMIT: u32 = 10;

pub async fn run_distribute(app: &'static AppData) -> eyre::Result<()> {
    let mut exit_listener = exit_listener()?;

    // Bind the address
    println!("Binding on address {}..", app.bind_address);
    let listener = TcpListener::bind(app.bind_address).await?;
    println!("Binding successful!\n");

    // Register our address
    println!(
        "Registering address {} on smart contract..",
        app.server_address
    );
    app.client
        .edit_server_info_call(app.server_address.to_string())
        .send()
        .await?;
    sleep(Duration::from_secs(1)).await;
    println!("Registration succesful!\n");

    // And register for the songs
    println!("Registering for all songs in database..");
    let result = match register_for_songs(&app).await {
        Ok(()) => {
            // Spawn background tasks
            let mut auto_distribute_task = spawn(auto_distribute_added_songs(app));
            let mut auto_download_task = spawn(auto_download_new_songs(app));

            tokio::select! {
                // The ctrl-c exit-handler
                _ = &mut exit_listener => {
                    Ok(())
                }

                // The auto-distribute task
                res = &mut auto_distribute_task => {
                    match res {
                        Ok(Err(e)) => Err(e),
                        Err(e) => Err(e.into()),
                        Ok(Ok(_)) => unreachable!()
                    }
                }

                // The auto-distribute task
                res = &mut auto_download_task => {
                    match res {
                        Ok(Err(e)) => Err(e),
                        Err(e) => Err(e.into()),
                        Ok(Ok(_)) => unreachable!()
                    }
                }

                // The main process that handles incoming connections
                res = accept_tcp_connections(listener, &app) => {
                    match res {
                        Err(e) => Err(e),
                        Ok(_) => unreachable!()
                    }
                }
            }
        }
        Err(error) => Err(error),
    };

    match (deregister_for_songs(app).await, result) {
        (Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Err(e)) => Err(e),
        (Err(e), Ok(_)) => Err(e),
        (Err(e1), Err(e2)) => Err(e1.wrap_err(e2)),
    }
}

pub async fn accept_tcp_connections(
    listener: TcpListener,
    app: &'static AppData,
) -> eyre::Result<Infallible> {
    println!("Accepting connections on {}", app.bind_address);
    loop {
        let (mut stream, addr) = listener.accept().await?;
        tokio::task::spawn(async move {
            match handle_new_listener(&mut stream, addr, app).await {
                Ok(()) => (),
                Err(e) => println!("Handler of {addr} exited with error {e}."),
            }
        });
    }
}

async fn handle_new_listener(
    stream: &mut TcpStream,
    addr: SocketAddr,
    app: &'static AppData,
) -> eyre::Result<()> {
    println!("Accepted connetion from {addr}");
    let stream = stream.split();

    let mut tcp_reader = FramedRead::new(stream.0, RequestChunksDecoder::new());
    let mut tcp_writer = FramedWrite::new(stream.1, SendChunksEncoder);
    // The amount of credit in chunks
    let mut credit: u32 = DEBT_LIMIT as u32;
    // Queue of client chunk-requests
    let mut open_requests: VecDeque<GetChunksCall> = VecDeque::new();
    // The transactions that resolves to the amount of credit.
    let mut transaction_pool = TransactionPool::new(&app.client, Duration::from_millis(100), 5);

    'outer: loop {
        let tcp_msg = tokio::select! {
            Some(result) = transaction_pool.next() => {
                let (receipt, new_credit) = result?;
                receipt.status_is_ok("")?;
                credit = credit.checked_add(new_credit).unwrap();
                None
            }

            res = tcp_reader.next() => {
                match res {
                    Some(msg) => {
                        Some(msg)
                    },
                    None => break 'outer Ok(())
                }
            }
        };

        if let Some(tcp_msg) = tcp_msg {
            let tx = match tcp_msg {
                Ok(tx) => Bytes(tx.freeze()),
                Err(e) => bail!("Incorrect tcp-protocol: {e}"),
            };

            let params = app.client.decode_get_chunks_params(&tx)?;
            if params.distributor != app.client.wallet_address() {
                bail!(
                    "Distributor address is not my address!: {}, {}",
                    params.distributor,
                    app.client.wallet_address()
                )
            }

            // And push the request and pending transaction to the lists.
            transaction_pool.push_raw_tx(tx, params.amount.as_u128().try_into()?);
            open_requests.push_back(params);
        };

        // Now we can send chunks until the credit runs out.
        'credit: while credit > 0 {
            // Check the next open request
            let Some(params) = open_requests.front_mut() else {
                continue 'outer;
            };

            // If it is empty, then we choose the next.
            if params.amount == 0.into() {
                open_requests.pop_front().unwrap();
                continue 'credit;
            }

            // Update the current open request, and select how much to stream right now.
            let (amount, index) = {
                let amount = Ord::min(credit, params.amount.as_u32());
                let index = params.index.as_u32();

                //
                credit -= amount;
                params.amount -= amount.into();
                params.index += amount.into();

                (amount, index)
            };

            // Get the chunks from the database
            let chunks = app
                .database
                .get_chunks(&params.song.into(), index, amount)
                .await?;

            println!("Sending {amount} chunks starting at {index} over TCP.");
            tcp_writer.send((index as u32, &chunks.into())).await?;
        }
    }
}