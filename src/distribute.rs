use crate::library::{
    app::AppData,
    client::TangleTunesClient,
    tcp::{RequestChunksDecoder, SendChunksEncoder},
    util::SongId,
    util::TransactionReceiptExt,
};
use color_eyre::Report;
use ethers::{
    types::{transaction::eip2718::TypedTransaction, Bytes},
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::{Http, PendingTransaction, StreamExt};
use eyre::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered, Future, FutureExt, SinkExt, Stream};
use std::{
    collections::VecDeque,
    fmt::Debug,
    net::SocketAddr,
    pin::Pin,
    sync::Mutex,
    task::{ready, Poll},
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::oneshot,
    time::{sleep, Sleep},
};
use tokio_util::codec::{FramedRead, FramedWrite};

pub async fn run_distribute(app: &'static AppData) -> eyre::Result<()> {
    // Start our exit handler which will message our main thread upon exit-signal
    let mut exit_signal = get_exit_signal()?;

    // Initialize some things
    let listener = TcpListener::bind(app.bind_address).await?;

    // Register our address on the server
    println!("Registering server address on smart contract: {}", app.server_address);
    app.client
        .edit_server_info(app.server_address.to_string())
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

    println!("Accepting connections on {}", app.bind_address);
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
    const DEBT_LIMIT: u32 = 10;

    let (read_stream, write_stream) = stream.split();
    let mut read_stream = FramedRead::new(read_stream, RequestChunksDecoder::new());
    let mut write_stream = FramedWrite::new(write_stream, SendChunksEncoder);

    // The amount of credit available for this client.
    let mut credit: i32 = DEBT_LIMIT as i32;
    // The requests from this client that haven't beel fulfilled yet.
    let mut open_requests: VecDeque<(SongId, i32, i32)> = VecDeque::new();

    // The transactions that will resolve with the amount of credit gained from them.
    let mut pending_transactions = VecDeque::<BoxFuture<'static, eyre::Result<i32>>>::new();
    // let mut pending_transactions = PendingTxPool::new(&app.client);

    'outer: loop {
        let tcp_msg = tokio::select! {
            Some(pending_tx) = async {
                if let Some(pending_tx) = &mut pending_transactions.front_mut() {
                    Some(pending_tx.await)
                } else {
                    None
                }
            } => {
                    pending_transactions.pop_front().unwrap();
                credit = credit.checked_add(pending_tx?).unwrap();
                None
            }

            // Some(new_credit) = pending_transactions.next() => {
            //     credit.checked_add(new_credit?).unwrap();
            //     None
            // }

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
            let tx_rlp = Bytes(tx_rlp.freeze());

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
            // pending_transactions.push_transaction(tx_rlp, amount);
            pending_transactions.push_back(Box::pin(async move {
                let tx = TypedTransaction::decode(&Rlp::new(&tx_rlp))?;
                println!("Sending transaction with nonce {:?}", tx.nonce());
                app.client
                    .send_raw_tx(tx_rlp.clone())
                    .await?
                    .await?
                    .status_is_ok("")?;
                Ok(amount)
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

                (amount_now, from_now)
            };

            // Get the chunks from the database
            let chunks = app
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
        for song_id in app.database.get_song_ids().await? {
            println!(
                "Registering for distribution of song {song_id} with nonce {}...",
                app.client.nonce().await?
            );

            let tx_hash = app
                .client
                .distribute(song_id.clone(), app.fee)
                .send()
                .await?
                .tx_hash();
            sleep(Duration::from_secs(1)).await;

            pending_txs.push(async move {
                let receipt = app
                    .client
                    .pending_tx(tx_hash, 1)
                    .await?
                    .status_is_ok(&format!("Could not register song with id {song_id}"))?;
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

    for song_id in app.database.get_song_ids().await? {
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

//------------------------------------------------------------------------------------------------
//  PendingTxPool
//------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PendingTxPool {
    client: &'static TangleTunesClient,
    transactions: Vec<PendingTx>,
}

impl PendingTxPool {
    pub fn new(client: &'static TangleTunesClient) -> Self {
        Self {
            client,
            transactions: Vec::new(),
        }
    }

    pub fn push_transaction(&mut self, tx: Bytes, credit: i32) {
        self.transactions
            .push(PendingTx::new(&self.client, tx, credit));
    }
}

impl Unpin for PendingTxPool {}

impl Stream for PendingTxPool {
    type Item = eyre::Result<i32>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut result = None;

        println!("-- Polling tx-pool --");

        // First poll the transaction that is stuck at stage 1.
        if let Some((i, tx)) = self
            .transactions
            .iter_mut()
            .enumerate()
            .find(|(_, tx)| tx.is_at_stage_1())
        {
            println!("Polling transaction {i} at stage 1");
            result = Some(tx.poll_unpin(cx)?.map(|credit| (i, credit)));
        }

        // If this did not set the result
        if result.is_none() {
            'inner: for (i, tx) in self.transactions.iter_mut().enumerate() {
                if let Poll::Ready(credit) = tx.poll_unpin(cx)? {
                    result = Some(Poll::Ready((i, credit)));
                    break 'inner;
                }
            }
        }

        println!("-- Done polling tx-pool: {result:#?} --");

        match result {
            // Tx is ready, remove it and return that
            Some(Poll::Ready((i, credit))) => {
                self.transactions.remove(i);
                Poll::Ready(Some(Ok(credit)))
            }
            Some(Poll::Pending) => Poll::Pending,
            // No transaction is ready at the moment
            None => match self.transactions.is_empty() {
                true => Poll::Pending,
                false => Poll::Ready(None),
            },
        }
    }
}

struct PendingTx {
    client: &'static TangleTunesClient,
    bytes: Bytes,
    credit: i32,
    timeout_len: Duration,
    retries_left: usize,
    timeout: Option<Pin<Box<Sleep>>>,
    stage1: Option<
        Pin<
            Box<
                dyn Future<Output = eyre::Result<PendingTransaction<'static, Http>>>
                    + Send
                    + 'static,
            >,
        >,
    >,
    stage2: Option<PendingTransaction<'static, Http>>,
}

impl Debug for PendingTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PendingTx")
            .field("client", &self.client)
            .field("bytes", &self.bytes)
            .field("credit", &self.credit)
            .field("timeout_len", &self.timeout_len)
            .field("retries_left", &self.retries_left)
            .field("timeout", &self.timeout)
            .field("stage1", &self.stage1.as_ref().map(|_| &".."))
            .field("stage2", &self.stage2)
            .finish()
    }
}

impl PendingTx {
    pub fn is_at_stage_1(&self) -> bool {
        self.stage2.is_none()
    }

    pub fn new(client: &'static TangleTunesClient, bytes: Bytes, credit: i32) -> Self {
        Self {
            client,
            bytes,
            credit,
            timeout_len: Duration::from_millis(100),
            retries_left: 8,
            timeout: None,
            stage1: None,
            stage2: None,
        }
    }

    fn try_set_next_timeout(&mut self) -> eyre::Result<()> {
        if self.retries_left == 0 {
            bail!("Max retries reached for tx")
        }
        if self.timeout.is_some() {
            panic!("Timeout must be none before setting the next timeout")
        }
        self.retries_left -= 1;
        self.timeout = Some(Box::pin(sleep(self.timeout_len)));
        self.timeout_len *= 2;
        Ok(())
    }
}

impl Unpin for PendingTx {}

impl Future for PendingTx {
    type Output = eyre::Result<i32>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        println!("PendingTx is being polled now");
        loop {
            // Check if a timeout is set and wait for that first
            if let Some(timeout) = &mut self.timeout {
                ready!(timeout.poll_unpin(cx));
                self.timeout = None;
                self.timeout_len *= 2;
            }

            // Check if the tx is already at stage 2
            if let Some(pending_tx) = &mut self.stage2 {
                println!("POLLING STAGE 2: {pending_tx:?}");
                ready!(dbg!(pending_tx.poll_unpin(cx)))
                    .wrap_err("Transaction failed on second stage")?
                    .status_is_ok("Transaction failed on second stage")?;
                return Poll::Ready(Ok(self.credit));
            };

            match &mut self.stage1 {
                // If the tx is already set, poll it
                Some(future) => match dbg!(ready!(future.poll_unpin(cx))) {
                    // Continue to stage 2
                    Ok(pending_tx) => self.stage2 = Some(pending_tx),
                    // Retry the transaction
                    Err(e) => {
                        println!("Retrying transaction with error: {e}");
                        self.stage1 = None;
                        self.try_set_next_timeout()?;
                    }
                },
                // If it is not then we send a new transaction
                None => {
                    println!("Creating stage_1 future now!");
                    let future = self.client.send_raw_tx(self.bytes.clone());
                    self.stage1 = Some(Box::pin(future));
                }
            }
        }
    }
}
