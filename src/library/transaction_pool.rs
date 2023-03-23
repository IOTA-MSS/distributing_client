use std::{collections::VecDeque, time::Duration};

use ethers::types::{Bytes, TransactionReceipt};
use ethers_providers::{Http, PendingTransaction, StreamExt};
use eyre::Context;
use futures::{future::BoxFuture, stream::FuturesUnordered};
use tokio::time::sleep;

use super::client::TangleTunesClient;

/// Multiple pending transactions that will be executed after another. The initial sending of the
/// transaction happens in order, while awaiting the transactions happens concurrently.
///
/// By setting the timeout and attempts, the initial sending will be retried that amount of times.
///
/// If any transaction fails for whatever reason
#[allow(clippy::type_complexity)]
pub struct TransactionPool<T> {
    client: &'static TangleTunesClient,
    stage1: VecDeque<BoxFuture<'static, eyre::Result<(PendingTransaction<'static, Http>, T)>>>,
    stage2: FuturesUnordered<BoxFuture<'static, eyre::Result<(TransactionReceipt, T)>>>,
    timeout: Duration,
    attempts: u32,
}

impl<T: std::fmt::Debug + Send + 'static> TransactionPool<T> {
    pub fn new(client: &'static TangleTunesClient, timeout: Duration, attempts: u32) -> Self {
        Self {
            client,
            stage1: VecDeque::new(),
            stage2: FuturesUnordered::new(),
            timeout,
            attempts,
        }
    }

    pub async fn next(&mut self) -> Option<eyre::Result<(TransactionReceipt, T)>> {
        loop {
            tokio::select! {
                biased;

                Some(res) = async {
                    match self.stage1.front_mut() {
                        Some(fut) => Some(fut.await),
                        None => None
                    }
                } => {
                    self.stage1.pop_front().unwrap();
                    match res {
                        Ok((pending_tx, val)) => {
                            self.stage2.push(Box::pin(async move {
                                Ok((pending_tx.await?.unwrap(), val))
                            }));
                            continue;
                        },
                        Err(e) => break Some(Err(e))
                    }
                }

                Some(res) = self.stage2.next() => {
                    break Some(res);
                }

                else => {
                    break None;
                }
            }
        }
    }

    pub fn push_raw_tx(&mut self, tx: Bytes, val: T) {
        let client = self.client;
        let attempts = self.attempts;
        let timeout = self.timeout;

        self.stage1.push_back(Box::pin(async move {
            let mut result: Option<eyre::Result<PendingTransaction<'static, Http>>> = None;

            for attempt in 0..attempts {
                match client.send_raw_tx(tx.clone()).await {
                    Ok(pending_tx) => {
                        result = Some(Ok(pending_tx));
                        break;
                    }
                    Err(new_err) => match result {
                        Some(Err(err)) => result = Some(Err(err)),
                        None => result = Some(Err(new_err)),
                        Some(Ok(_)) => unreachable!(),
                    },
                }
                sleep(timeout * 2_u32.saturating_pow(attempt)).await;
            }

            let pending_tx = result
                .expect("attempts > 0")
                .wrap_err(format!("Sending the transaction failed {attempts} times"))?;
            Ok((pending_tx, val))
        }));
    }
}
