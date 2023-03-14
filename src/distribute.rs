use crate::library::{
    app::AppData,
    tcp::{RequestChunksDecoder, SendChunksEncoder},
    util::SongId,
    util::TransactionReceiptExt,
};
use color_eyre::Report;
use ethers_providers::{Http, PendingTransaction, StreamExt};
use eyre::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered, SinkExt};
use std::{collections::VecDeque, net::SocketAddr, sync::Mutex, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot, time::sleep,
};
use tokio_util::codec::{FramedRead, FramedWrite};

pub async fn run_distribute(app: &'static AppData) -> eyre::Result<()> {
    // Start our exit handler which will message our main thread upon exit-signal
    let mut exit_signal = get_exit_signal()?;

    // Initialize some things
    let address = SocketAddr::new(app.ip_address, app.port);
    let listener = TcpListener::bind(address).await?;

    // Register our address on the server
    println!("Registering server address on smart contract: {address}");
    app.client
        .edit_server_info(address.to_string())
        .send()
        .await?;
    sleep(Duration::from_secs(1)).await;



    println!("Registering songs");
    tokio::select! {
        _ = &mut exit_signal => {
            return deregister_for_songs(&app).await;
        }

        res = register_for_songs(&app) => { res }
    }?;

    println!("Accepting connections on {address}");
    match accept_connections(listener, &mut exit_signal, &app).await {
        Ok(()) => deregister_for_songs(app).await,
        Err(e) => deregister_for_songs(app).await.wrap_err(e),
    }
}

async fn accept_connections(
    listener: TcpListener,
    exit_signal: &mut oneshot::Receiver<()>,
    app: &'static AppData,
) -> eyre::Result<()> {
    tokio::select! {
        _ = exit_signal => {
            Ok(())
        }

        res = async {
            loop {
                // If there is an incoming tcp-connection, handle it appropriately
                let (mut stream, addr) = listener.accept().await?;
                tokio::task::spawn(async move {
                    if let Err(e) = dbg!(handle_new_listener(&mut stream, addr, app).await) {
                        warn!("Handler exited with an error: {:?}", e);
                    }
                });
            }
        } => { res }
    }
}

async fn handle_new_listener(
    stream: &mut TcpStream,
    addr: SocketAddr,
    app: &'static AppData,
) -> eyre::Result<()> {
    println!("Accepted connetion from {addr}");
    static DEBT_LIMIT: u32 = 3;
    let (read_stream, write_stream) = stream.split();
    let mut read_stream = FramedRead::new(read_stream, RequestChunksDecoder::new());
    let mut write_stream = FramedWrite::new(write_stream, SendChunksEncoder);

    // The amount of credit available for this client.
    let mut credit: i32 = DEBT_LIMIT as i32;
    // The requests from this client that haven't beel fulfilled yet.
    let mut open_requests: VecDeque<(SongId, i32, i32)> = VecDeque::new();

    // The transactions that will resolve with the amount of credit gained from them.
    let mut pending_txs_stage_1 =
        VecDeque::<BoxFuture<'static, eyre::Result<(PendingTransaction<Http>, i32)>>>::new();
    let mut pending_txs_stage_2 = FuturesUnordered::<BoxFuture<'static, eyre::Result<i32>>>::new();

    'outer: loop {
        let tcp_msg = tokio::select! {
            Some(pending_tx) = async {
                if let Some(pending_tx) = &mut pending_txs_stage_1.front_mut() {
                    Some(pending_tx.await)
                } else {
                    None
                }
            } => {
                let (pending_tx, amount) = pending_tx?;
                pending_txs_stage_1.pop_front().unwrap();
                pending_txs_stage_2.push(Box::pin(async move {
                    dbg!(pending_tx.await)?.unwrap();
                    Ok(amount as i32)
                }));
                None
            }

            Some(new_credit) = pending_txs_stage_2.next() => {
                credit = credit.checked_add(new_credit?).unwrap();
                None
            }

            res = read_stream.next() => {
                match res {
                    Some(msg) => {
                        Some(msg)
                    },
                    None => break 'outer Ok(())
                }
            }
        };

        if let Some(tcp_msg) = tcp_msg {
            let Ok(tx_rlp) = tcp_msg else { todo!("{tcp_msg:?}")};

            // Decode the parameters we care about
            let (song_id, from, amount) = {
                let decoded_call = app.client.decode_get_chunks_tx_rlp(&tx_rlp)?;
                if decoded_call.distributor != app.client.wallet_address() {
                    bail!(
                        "Distributor address is not my address!: {}, {}",
                        decoded_call.distributor,
                        app.client.wallet_address()
                    )
                }
                (
                    decoded_call.song.into(),
                    decoded_call.index.as_u128().try_into()?,
                    decoded_call.amount.as_u128().try_into()?,
                )
            };

            // And push the request and pending transaction to the lists.
            open_requests.push_back((song_id, from, amount));
            pending_txs_stage_1.push_back(Box::pin(async move {
                println!("Sending raw transaction to the smart-contract...");
                let pending_tx = dbg!(app.client.send_raw_tx(tx_rlp.freeze().into()).await)?;
                sleep(Duration::from_secs(1)).await;
                Ok((pending_tx, amount))
            }));
        };

        // Now we can send chunks until the credit runs out.
        'inner: while dbg!(credit) > 0 {
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
            let (chunk_amount, start_chunk) = {
                let amount_now = credit.min(*amount);
                let from_now = from.clone();

                credit -= amount_now;
                *amount -= amount_now;
                *from += amount_now;

                dbg!((amount_now, from_now))
            };

            // Get the chunks from the database
            let (chunks, _fee) = app
                .database
                .get_chunks(song_id, start_chunk as u32, chunk_amount as u32)
                .await?;

            println!("Sending {chunk_amount} chunks starting at {start_chunk} over TCP.");

            write_stream
                .send((start_chunk as u32, &chunks.into()))
                .await?;
        }
    }
}

/// Registers for distribution of all songs in the database.
/// If an error occurs, all songs are automatically deregistered.
pub async fn register_for_songs(app: &AppData) -> eyre::Result<()> {
    let mut pending_txs = FuturesUnordered::new();

    // Send all transactions until complete or an error is encountered
    let sending_txs_result = {
        for (song_id, _) in app
            .database
            .get_songs_info()
            .await?
            .into_iter()
            .filter(|(_, distributing)| !distributing)
        {
            println!(
                "Registering for distribution of song {song_id} with nonce {}...",
                app.client.nonce().await?
            );

            let tx_hash =
                dbg!(app.client.distribute(song_id.clone(), app.fee).send().await)?.tx_hash();
            sleep(Duration::from_secs(1)).await;

            pending_txs.push(async move {
                let receipt = app
                    .client
                    .pending_tx(tx_hash, 1)
                    .await?
                    .status_is_ok(&format!("Could not register song with id {song_id}"))?;
                app.database.set_distribution(&song_id, true).await?;
                Ok::<_, Report>((receipt, song_id))
            })
        }

        Ok::<_, eyre::Report>(())
    };

    // Wait for all confirmations to be received.
    // If an error occurs we just log it and continue.
    while let Some(result) = pending_txs.next().await {
        match result {
            Ok((_, song_id)) => println!("Registered song {song_id}"),
            Err(e) => println!("ERROR: {e}"),
        }
    }

    // If sending of all registrations was successful return that.
    // If an error occured there, we will deregister for all registered songs.
    match sending_txs_result {
        Ok(_) => Ok(()),
        Err(e) => match deregister_for_songs(app).await {
            Ok(_) => Err(e),
            Err(e2) => Err(e.wrap_err(e2)),
        },
    }
}

/// Deregister for distribution of the given songs.
pub async fn deregister_for_songs(app: &AppData) -> eyre::Result<()> {
    let mut pending_transactions = FuturesUnordered::new();

    for (song_id, _) in app
        .database
        .get_songs_info()
        .await?
        .into_iter()
        .filter(|(_, registered)| *registered)
    {
        let result = {
            println!("Deregistering song {song_id} on the smart-contract..");
            let tx_hash = app
                .client
                .undistribute(song_id.clone())
                .send()
                .await?
                .tx_hash();
            sleep(Duration::from_secs(1)).await;

            pending_transactions.push(async move {
                let receipt = app
                    .client
                    .pending_tx(tx_hash, 1)
                    .await?
                    .unwrap()
                    .status_is_ok(&format!("Could not deregister song with id {song_id}"))?;
                app.database.set_distribution(&song_id, false).await?;
                Ok::<_, eyre::Report>((receipt, song_id))
            });
            Ok::<_, eyre::Report>(())
        };
        if let Err(e) = result {
            println!("ERROR: Problem deregistering song: {e}");
        };
    }

    while let Some(result) = pending_transactions.next().await {
        match result {
            Ok((_, song_id)) => println!("Deregistered song {song_id}"),
            Err(e) => println!("ERROR: {e}"),
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::library::{app::AppData, util::SongId, util::TransactionReceiptExt};

    #[ignore]
    #[tokio::test]
    async fn test_distribute() -> eyre::Result<()> {
        // let app =  App::initialize_from_cfg_path("./TangleTunes.toml".to_string(), None).await?;
        let app: &'static AppData = todo!();
        dbg!(app
            .client
            .distribute(
                SongId::try_from_hex(
                    "0x8b3d8bfd0c161381ce232660cd0b2262109b27be18989870406b5d0b986e60f9"
                )?,
                1
            )
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok("")?);
        Ok(())
    }
}

pub fn get_exit_signal() -> eyre::Result<oneshot::Receiver<()>> {
    static EXIT_SIGNAL: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);

    let (tx, rx) = oneshot::channel();
    *EXIT_SIGNAL.lock().unwrap() = Some(tx);
    ctrlc::set_handler(|| {
        let _ = EXIT_SIGNAL.lock().unwrap().take().unwrap().send(());
    })?;
    Ok(rx)
}
