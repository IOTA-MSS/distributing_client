use std::{error::Error, fmt::Display, sync::atomic::AtomicU64};

use async_trait::async_trait;
use ethers::{
    prelude::{signer::SignerMiddlewareError, SignerMiddleware},
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, BlockId},
};
use ethers_providers::{Http, Middleware, MiddlewareError, Provider};

#[derive(Debug)]
pub struct TangleTunesMiddleware {
    inner: SignerMiddleware<Provider<Http>, LocalWallet>,
    nonce: Option<AtomicU64>,
}

impl TangleTunesMiddleware {
    pub async fn new(provider: Provider<Http>, wallet: LocalWallet) -> Self {
        Self {
            nonce: None,
            inner: SignerMiddleware::new(provider, wallet),
        }
    }
}

#[async_trait]
impl Middleware for TangleTunesMiddleware {
    type Error = TangleTunesMiddlewareError;
    type Provider = Http;
    type Inner = SignerMiddleware<Provider<Http>, LocalWallet>;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    async fn fill_transaction(
        &self,
        tx: &mut TypedTransaction,
        block: Option<BlockId>,
    ) -> Result<(), Self::Error> {
        self.inner()
            .fill_transaction(tx, block)
            .await
            .map_err(MiddlewareError::from_err)?;

        // tx.set_nonce(nonce);
        todo!()
    }
}

impl Display for TangleTunesMiddlewareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for TangleTunesMiddlewareError {}

#[derive(Debug)]
pub struct TangleTunesMiddlewareError(eyre::Report);

impl MiddlewareError for TangleTunesMiddlewareError {
    type Inner = SignerMiddlewareError<Provider<Http>, LocalWallet>;

    fn from_err(e: Self::Inner) -> Self {
        todo!()
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        todo!()
    }
}
