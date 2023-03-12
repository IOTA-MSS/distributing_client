use std::collections::VecDeque;

use ethers_providers::StreamExt;
use futures::{future::BoxFuture, stream::FuturesUnordered, Future};

pub struct PendingTransactionPool<T, R> {
    stage_1: VecDeque<BoxFuture<'static, eyre::Result<T>>>,
    stage_2: FuturesUnordered<BoxFuture<'static, eyre::Result<R>>>,
    converter: Box<dyn Fn(T) -> BoxFuture<'static, eyre::Result<R>> + Send>,
}

impl<T: Send + 'static, R: 'static> PendingTransactionPool<T, R> {
    pub fn new<F2>(converter: impl Fn(T) -> F2 + Send + 'static) -> Self
    where
        F2: Future<Output = eyre::Result<R>> + Send + 'static,
    {
        Self {
            stage_1: VecDeque::new(),
            stage_2: FuturesUnordered::new(),
            converter: Box::new(move |t| Box::pin(converter(t))),
        }
    }

    pub fn push_stage_1(
        &mut self,
        stage_1: impl Future<Output = eyre::Result<T>> + Send + 'static,
    ) {
        self.stage_1.push_back(Box::pin(stage_1));
    }

    pub async fn next(&mut self) -> Option<eyre::Result<R>> {
        loop {
            tokio::select! {
                Some(pending_tx) = async {
                    if let Some(pending_tx) = self.stage_1.front_mut() {
                        Some(pending_tx.await)
                    } else {
                        None
                    }
                } => {
                    self.stage_1.pop_front().unwrap();
                    match pending_tx {
                        Err(e) => {
                            break Some(Err(e))
                        },
                        Ok(pending_tx) => {
                            let stage_2 = (self.converter)(pending_tx);
                            self.stage_2.push(Box::pin(stage_2));
                        }
                    }
                }

                Some(result) = self.stage_2.next() => {
                    break Some(result)
                }

                else => {
                    break None
                }

            }
        }
    }
}
