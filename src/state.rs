use crate::{
    contract::{IotaMssContract, SignedIotaMssContract},
    music_folder::MusicFolder,
    wallet::wallet_from_key,
};
use ethers::signers::LocalWallet;
use ethers_providers::{Http, Provider};
use eyre::eyre;
use once_cell::sync::OnceCell;

static STATE: OnceCell<State> = OnceCell::new();

#[derive(Debug)]
pub struct State {
    /// The folder from which music is served
    pub music_folder: MusicFolder,
    /// The port on which to serve
    pub port: u16,
    /// The wallet used for the iota ledger
    pub wallet: LocalWallet,
    /// The type-checked smart contract which signs transactions with the local wallet
    pub contract: SignedIotaMssContract,
    /// The raw client which can be used to send raw transactions.
    pub raw_client: Provider<Http>,
}

impl State {
    /// Initializes the global application state
    pub fn init(music_folder: &str, port: u16, key: String, node_url: &str) -> eyre::Result<()> {
        let raw_client = Provider::try_from(node_url)?;
        let wallet = wallet_from_key(key)?;
        let music_folder = MusicFolder::new(music_folder);
        let contract = IotaMssContract::new_signed(raw_client.clone(), wallet.clone());

        let state = Self {
            music_folder,
            port,
            raw_client,
            wallet,
            contract,
        };

        STATE.set(state).expect("Can't set global state twice");

        Ok(())
    }

    /// Get the global state
    pub fn get() -> &'static State {
        STATE.get().unwrap()
    }

    pub fn folder() -> &'static MusicFolder {
        &Self::get().music_folder
    }

    pub fn contract() -> &'static SignedIotaMssContract {
        &Self::get().contract
    }
}
