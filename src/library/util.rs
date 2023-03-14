use ethers::{
    prelude::ContractError,
    types::{TransactionReceipt, H256},
    utils::hex::{FromHex, ToHex},
};
use ethers_providers::{Http, Middleware, PendingTransaction};
use futures::{future::BoxFuture, stream::FuturesUnordered, Future, FutureExt, Stream};
use std::{
    collections::VecDeque,
    error::Error,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Sleep;

use super::client::{TTCall, TTMiddleWare, TangleTunesClient};

#[derive(Clone)]
pub struct SongId([u8; 32]);

impl TryFrom<Vec<u8>> for SongId {
    type Error = eyre::Report;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into().map_err(|value| {
            eyre!("Couldn't convert vec to song-id: {value:?}")
        })?))
    }
}

impl From<[u8; 32]> for SongId {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl Into<[u8; 32]> for SongId {
    fn into(self) -> [u8; 32] {
        self.0
    }
}

impl Debug for SongId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SongId").field(&self.to_hex()).finish()
    }
}

impl Display for SongId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Deref for SongId {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SongId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SongId {
    pub fn try_from_hex(hex: impl AsRef<str>) -> eyre::Result<Self> {
        Ok(Self(try_from_hex_prefix(hex)?))
    }

    pub fn to_hex(&self) -> String {
        to_hex_prefix(&self.0)
    }
}

pub fn to_hex_prefix(bytes: impl AsRef<[u8]>) -> String {
    format!("0x{}", ToHex::encode_hex::<String>(&bytes))
}

pub fn try_from_hex_prefix<T>(hex: impl AsRef<str>) -> eyre::Result<T>
where
    T: FromHex,
    T::Error: Error + Send + Sync + 'static,
{
    match hex.as_ref().strip_prefix("0x") {
        Some(hex) => Ok(FromHex::from_hex(hex)?),
        None => Ok(FromHex::from_hex(hex.as_ref())?),
    }
}

//------------------------------------------------------------------------------------------------
//  TransactionReceipt
//------------------------------------------------------------------------------------------------

pub trait TransactionReceiptExt: Sized {
    fn status_is_ok(self, msg: &str) -> eyre::Result<TransactionReceipt>;
}

impl TransactionReceiptExt for Option<TransactionReceipt> {
    fn status_is_ok(self, msg: &str) -> eyre::Result<TransactionReceipt> {
        match self {
            Some(receipt) => receipt.status_is_ok(msg),
            None => Err(eyre!("No receipt on transaction. {msg}")),
        }
    }
}
impl TransactionReceiptExt for TransactionReceipt {
    fn status_is_ok(self, msg: &str) -> eyre::Result<Self> {
        let status = self
            .status
            .ok_or_else(|| eyre!("Transaction without status: {self:?}"))?;

        if status != 1.into() {
            Err(eyre!("Transaction failed (status = 0): {msg}"))
        } else {
            Ok(self)
        }
    }
}

//------------------------------------------------------------------------------------------------
//  PendingTx
//------------------------------------------------------------------------------------------------

pub trait PendingTransactionExt: Sized {
    fn with_client<'b>(self, client: &'b TangleTunesClient) -> PendingTransaction<'b, Http>;
}

impl<'a> PendingTransactionExt for PendingTransaction<'a, Http> {
    fn with_client<'b>(self, client: &'b TangleTunesClient) -> PendingTransaction<'b, Http> {
        client.pending_tx(self.tx_hash(), 1)
    }
}

// pub struct TransactionPool {
//     client: &'static TangleTunesClient,
//     broadcasting: VecDeque<BroadcastTx>,
//     delay: Option<Pin<Box<Sleep>>>,
//     pending: FuturesUnordered<PendingTransaction<'static, Http>>,
// }

// struct BroadcastTx {
//     retries: u32,
//     call: TTCall<()>,
//     future: BoxFuture<'static, eyre::Result<H256>>,
// }

// impl Unpin for TransactionPool {}

// impl Stream for TransactionPool {
//     type Item = eyre::Result<TransactionReceipt>;

//     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         let this = &mut *self;

//         let poll_broadcast = match &mut this.delay {
//             Some(delay) => {
//                 if let Poll::Ready(()) = delay.poll_unpin(cx) {
//                     this.delay.take().unwrap();
//                     true
//                 } else {
//                     false
//                 }
//             }
//             None => true,
//         };

//         if poll_broadcast {
//             if let Some(tx) = this.broadcasting.front_mut() {
//                 if let Poll::Ready(result) = tx.future.poll_unpin(cx) {
//                     match result {
//                         Ok(hash) => {
//                             this.broadcasting.pop_front();
//                             let pending_tx = this.client.pending_tx(hash, 1);
//                             this.pending.push(pending_tx);
//                         }
//                         Err(e) => {
//                             if tx.retries == 0 {
//                                 return Poll::Ready(Some(Err(e.into())));
//                             }
//                             tx.retries -= 1;
//                             let call = tx.call.clone();
//                             let new_future = Box::pin(async {
//                                 this.client
//                                     .abi_client
//                                     .client_ref()
//                                     .send_transaction(tx.call.tx, None);
//                                 let future = call.send();
//                                 Ok(future.await?.tx_hash())
//                             });
//                             tx.future = new_future;
//                             todo!()
//                         }
//                     }
//                 }
//             }
//         }

//         for tx in &mut this.pending {
//             if let Poll::Ready(res) = tx.poll_unpin(cx) {
//                 match res {
//                     Ok(_) => todo!(),
//                     Err(_) => todo!(),
//                 }
//             }
//         }
//         todo!()
//     }
// }
