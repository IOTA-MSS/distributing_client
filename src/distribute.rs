use crate::library::{
    client::TangleTunesClient,
    app::App,
    database::Database,
    protocol::{RequestChunksDecoder, SendChunksEncoder},
};
use ethers_providers::{StreamExt, PendingTransaction, Http};
use futures::{future::BoxFuture, stream::FuturesUnordered, SinkExt};
use std::{collections::VecDeque, net::SocketAddr,};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{FramedRead, FramedWrite};

pub async fn run(cfg: App) -> eyre::Result<()> {
    let database = cfg.database()?;
    let client = cfg.initialize_client(&database).await?;

    let listener = TcpListener::bind(("127.0.0.1", cfg.port()))
        .await
        .unwrap();

    while let Ok((mut stream, addr)) = listener.accept().await {
        let _ = tokio::task::spawn(async move {
            if let Err(e) =
                dbg!(handle_incoming_tcp_connection(&mut stream, addr, database, client).await)
            {
                warn!("Handler exited with an error: {:?}", e);
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
    static DEBT_LIMIT: u32 = 3;
    let (read_stream, write_stream) = stream.split();
    let mut read_stream = FramedRead::new(read_stream, RequestChunksDecoder::new());
    let mut write_stream = FramedWrite::new(write_stream, SendChunksEncoder);

    // The amount of credit available for this client.
    let mut credit: i32 = DEBT_LIMIT as i32;
    // The requests from this client that haven't beel fulfilled yet.
    let mut open_requests: VecDeque<(String, i32, i32)> = VecDeque::new();
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
                    hex::encode(decoded_call.song),
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
