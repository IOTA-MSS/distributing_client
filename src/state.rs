use crate::iota::{wallet_from_key, IotaMssContract};
use ethers::{
    prelude::{k256::SecretKey, rand::rngs::ThreadRng},
    signers::LocalWallet,
    signers::Signer,
};
use ethers_providers::{Http, Provider};
use eyre::eyre;
use once_cell::sync::OnceCell;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

static STATE: OnceCell<DistributionState> = OnceCell::new();

#[derive(Debug)]
pub struct DistributionState {
    pub music_folder: PathBuf,
    pub port: u16,
    pub wallet: LocalWallet,
    pub contract: IotaMssContract<Provider<Http>>,
}

impl DistributionState {
    /// Initializes the global application state
    pub fn init(
        music_folder: &str,
        port: u16,
        key: String,
        provider_url: &str,
    ) -> eyre::Result<()> {
        let wallet = wallet_from_key(key)?;

        let state = Self {
            music_folder: Path::new(music_folder).into(),
            port,
            contract: IotaMssContract::new(
                wallet.address(),
                Arc::new(Provider::try_from(provider_url)?),
            ),
            wallet,
        };

        STATE
            .set(state)
            .or(Err(eyre!("Can't set global state twice")))?;

        Ok(())
    }
}

/// Get the global state
pub fn state() -> &'static DistributionState {
    STATE.get().unwrap()
}
