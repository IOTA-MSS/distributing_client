use crate::{
    arguments::Demo,
    command::distribute::{
        background_tasks::{auto_distribute, exit_listener},
        distribution::{distribute_songs_in_database, undistribute_songs_in_database},
    },
    library::{
        abi::GetChunksCall,
        app::App,
        tcp::{RequestChunksDecoder, SendChunksEncoder},
        transaction_pool::TransactionPool,
        util::{SongId, TransactionReceiptExt},
    },
};
use ethers::types::Bytes;
use ethers_providers::StreamExt;
use eyre::Context;
use futures::SinkExt;
use std::{collections::VecDeque, convert::Infallible, net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{FramedRead, FramedWrite};

mod background_tasks;
mod distribution;

/// How many chunks in debt the listener is allowed
const DEBT_LIMIT: u32 = 10;
/// The amount of songs distributed at once
const DISTR_SIZE: usize = 5;
/// The amount of attempts at distribution of songs
const DISTR_ATTEMPTS: usize = 1;

pub async fn distribute(app: &'static App, demo: Option<Demo>) -> eyre::Result<()> {
    let mut exit_listener = exit_listener()?;

    // Bind on the port
    println!("Binding on address {}..", app.bind_address);
    let listener = TcpListener::bind(app.bind_address).await?;
    println!("Binding successful!\n");

    // Register our server address on the smart contract
    println!(
        "Registering address {} on smart contract..",
        app.server_address
    );
    app.client
        .edit_server_info_call(app.server_address.to_string())
        .send()
        .await?
        .await?;
    println!("Registration of address succesful!\n");

    // And distribute all songs in the database
    if let Err(e) = distribute_songs_in_database(app, DISTR_ATTEMPTS, DISTR_SIZE).await {
        return match undistribute_songs_in_database(app, DISTR_ATTEMPTS, DISTR_SIZE).await {
            Ok(()) => Err(e),
            Err(e2) => Err(e.wrap_err(e2)),
        };
    }

    // Spawn our automatic distributor
    let mut auto_distributor = tokio::task::spawn(self::auto_distribute(app, demo));

    let result = tokio::select! {
        // The ctrl-c exit-handler
        _ = &mut exit_listener => {
            auto_distributor.abort();
            let _ = auto_distributor.await;
            Ok(())
        }

        // The auto-distribute task
        res = &mut auto_distributor => {
            Err(res.unwrap_err().into())
        }

        // The main process that handles incoming connections
        res = accept_tcp_connections(listener, app) => {
            auto_distributor.abort();
            let _ = auto_distributor.await;
            match res {
                Err(e) => Err(e),
                Ok(_) => unreachable!()
            }
        }
    };

    // And for graceful shutdown we undistribute all songs
    match (undistribute_songs_in_database(app, 3, 5).await, result) {
        (Ok(_), Ok(_)) => Ok(()),
        (Ok(_), Err(e)) => Err(e),
        (Err(e), Ok(_)) => Err(e),
        (Err(e1), Err(e2)) => Err(e1.wrap_err(e2)),
    }
}

/// Accept incoming tcp-connections and spawn processes to handle them.
pub async fn accept_tcp_connections(
    listener: TcpListener,
    app: &'static App,
) -> eyre::Result<Infallible> {
    println!("Accepting connections on {}", app.bind_address);
    loop {
        let (mut stream, addr) = listener.accept().await?;
        tokio::task::spawn(async move {
            match handle_new_connection(&mut stream, addr, app).await {
                Ok(()) => (),
                Err(e) => eprintln!("Handler {addr} exited with error {e:#}."),
            }
        });
    }
}

/// Handle a new tcp-connection.
async fn handle_new_connection(
    stream: &mut TcpStream,
    addr: SocketAddr,
    app: &'static App,
) -> eyre::Result<()> {
    println!("Accepted connetion from {addr}");

    // The amount of credit in chunks
    let mut credit: u32 = DEBT_LIMIT;
    // Queue of client chunk-requests
    let mut open_requests: VecDeque<GetChunksCall> = VecDeque::new();
    // The transactions that resolves to the amount of credit.
    let mut transaction_pool = TransactionPool::new(&app.client, Duration::from_millis(100), 7);

    // Construct the tcp-stream
    let stream = stream.split();
    let mut tcp_reader = FramedRead::new(stream.0, RequestChunksDecoder::new());
    let mut tcp_writer = FramedWrite::new(stream.1, SendChunksEncoder);

    // An repeatedly wait for messages to arrive over tcp or for a transaction to complete from the pool.
    'outer: loop {
        let tcp_msg = tokio::select! {
            Some(result) = transaction_pool.next() => {
                let (receipt, new_credit) = result?;
                receipt.status_is_ok("").wrap_err(format!("From request-chunks transaction of {addr}"))?;
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

        // If the event was a tcp-message, then we have to verify it.
        if let Some(tcp_msg) = tcp_msg {
            let tx = Bytes(
                tcp_msg
                    .wrap_err(format!("Custom tcp-protocol not folowed by {addr}"))?
                    .freeze(),
            );

            // Decode the chunk-request paramaters
            let params = app.client.decode_get_chunks_params(&tx)?;
            if params.distributor != app.client.wallet_address() {
                bail!(
                    "Distributor address is not my address!: {}, {}",
                    params.distributor,
                    app.client.wallet_address()
                )
            }

            println!(
                "Received get-chunks from {} for song {} with index {} and amount {}",
                addr,
                SongId::from(params.song),
                params.index,
                params.amount
            );

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

                credit -= amount;
                params.amount -= amount.into();
                params.index += amount.into();

                (amount, index)
            };

            // Get the chunks from the database and send over tcp
            let chunks = app
                .database
                .get_chunks(&params.song.into(), index, amount)
                .await?;
            println!("Sending {amount} chunks starting at {index} to {addr}.");
            tcp_writer.send((index, &chunks.into())).await?;
        }
    }
}
