use super::client::{TTCall, TangleTunesClient};
use color_eyre::Report;
use ethers::{
    abi::Detokenize,
    types::TransactionReceipt,
    utils::hex::{FromHex, ToHex},
};
use ethers_providers::{Http, PendingTransaction};
use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
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

impl FromStr for SongId {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_hex(s)
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
            Err(eyre!(
                "Transaction failed: status = 0, tx-hash = {:?}, {msg}.",
                self.transaction_hash
            ))
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
        client.create_pending_tx(self.tx_hash(), 1)
    }
}

//------------------------------------------------------------------------------------------------
//  CallExt
//------------------------------------------------------------------------------------------------

pub trait TTCallExt<T>: Sized {
    fn set_defaults(self) -> Self;
}

impl<T: Detokenize> TTCallExt<T> for TTCall<T> {
    fn set_defaults(self) -> Self {
        self.legacy().gas(1_000_000).gas_price(1)
    }
}
