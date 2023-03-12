use crate::{
    library::{
        app::App,
        client::TangleTunesClient,
        database::Database,
        protocol::{RequestChunksDecoder, SendChunksEncoder},
    },
    util::SongId,
};
use ethers_providers::{Http, PendingTransaction, StreamExt};
use eyre::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered, SinkExt};
use std::{collections::VecDeque, net::SocketAddr, sync::Mutex};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
};
use tokio_util::codec::{FramedRead, FramedWrite};

static EXIT_SIGNAL: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);

pub async fn run(app: App) -> eyre::Result<()> {
    // Initialize our client and tcp-server
    let client = app.initialize_client(&app.database).await?;
    let listener = TcpListener::bind(("127.0.0.1", app.port())).await?;

    // Start our exit handler which will message our main thread upon exit-signal
    let mut exit_signal = {
        let (tx, rx) = oneshot::channel();
        *EXIT_SIGNAL.lock().unwrap() = Some(tx);
        ctrlc::set_handler(|| {
            let _ = EXIT_SIGNAL.lock().unwrap().take().unwrap().send(());
        })?;
        rx
    };

    // And register for distribution of all songs
    let registered_songs = register_for_songs(&app, client).await?;

    // Now we can run the actual server that accepts connections
    let result = async {
        loop {
            // Concurrently do:
            tokio::select! {
                biased;

                // If we receive an exit signal, we can break immeadeately
                _ = (&mut exit_signal) => {
                    break Ok::<_, eyre::Report>(())
                }

                // If there is an incoming tcp-connection, handle it appropriately
                connection = listener.accept() => {
                    let (mut stream, addr) = connection?;
                    tokio::task::spawn(async move {
                        if let Err(e) =
                            dbg!(handle_new_listener(&mut stream, addr, client, app.database).await)
                        {
                            warn!("Handler exited with an error: {:?}", e);
                        }
                    });
                }
            }
        }
    }
    .await;

    // Now clean up and deregister for the songs
    let deregister_result = deregister_for_songs(registered_songs, client).await;

    if let Err(e) = deregister_result {
        result.wrap_err(e)
    } else {
        result
    }
}

async fn handle_new_listener(
    stream: &mut TcpStream,
    _addr: SocketAddr,
    client: &'static TangleTunesClient,
    database: Database,
) -> eyre::Result<()> {
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
                    let _receipt = dbg!(pending_tx.await?.unwrap());
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
                let decoded_call = client.decode_get_chunks_tx_rlp(&tx_rlp)?;
                if decoded_call.distributor != client.wallet_address() {
                    bail!(
                        "Distributor address is not my address!: {}, {}",
                        decoded_call.distributor,
                        client.wallet_address()
                    )
                }
                (
                    decoded_call.song_id.into(),
                    decoded_call.index.as_u128().try_into()?,
                    decoded_call.amount.as_u128().try_into()?,
                )
            };

            // And push the request and pending transaction to the lists.
            open_requests.push_back((song_id, from, amount));
            pending_txs_stage_1.push_back(Box::pin(async move {
                let pending_tx = dbg!(client.send_raw_tx(tx_rlp.freeze().into()).await)?;
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
            let (chunks, _fee) = database
                .get_chunks(song_id, start_chunk as u32, chunk_amount as u32)
                .await?;

            write_stream
                .send((start_chunk as u32, &chunks.into()))
                .await?;
        }
    }
}

pub async fn register_for_songs(
    app: &App,
    client: &'static TangleTunesClient,
) -> eyre::Result<Vec<SongId>> {
    let mut tx_hashes = FuturesUnordered::new();
    for (song_id, distributing) in app.database.get_songs_metadata().await? {
        let _result = {
            if distributing {
                println!("Registering song {song_id} on the smart-contract..");
                let tx_hash = *client.distribute2(song_id.clone(), app.fee)?.send().await?;
                tx_hashes.push(async move { (client.get_receipt(tx_hash).await, song_id) })
            }
            Ok::<_, eyre::Report>(())
        };
    }
    let mut registered_songs = Vec::new();
    while let Some((receipt, song_id)) = tx_hashes.next().await {
        if let Ok(receipt) = receipt {
            if receipt.status.unwrap() == 1.into() {
                println!("Succesfully registered song {song_id}!");
                registered_songs.push(song_id);
                continue;
            }
        }
        println!("ERROR: Could not register song {song_id}")
    }
    Ok(registered_songs)
}

pub async fn deregister_for_songs(
    registered_songs: Vec<SongId>,
    client: &'static TangleTunesClient,
) -> eyre::Result<()> {
    let mut tx_hashes = FuturesUnordered::new();
    for song_id in registered_songs {
        let _result = {
            println!("Deregistering song {song_id} on the smart-contract..");
            let tx_hash = *client.undistribute(song_id.clone())?.send().await?;
            tx_hashes.push(async move { (client.get_receipt(tx_hash).await, song_id) });
            Ok::<_, eyre::Report>(())
        };
    }
    while let Some((receipt, song_id)) = tx_hashes.next().await {
        if let Ok(receipt) = receipt {
            if receipt.status.unwrap() == 1.into() {
                println!("Succesfully deregistered song {song_id}!");
                continue;
            }
        }
        println!("ERROR: Could not deregister song {song_id}");
    }
    Ok(())
}
