use crate::{
    contract::{self, IotaMssContract, SignedIotaMssContract},
    music_folder::MusicFolder,
    wallet,
};
use ethers::signers::LocalWallet;
use ethers_providers::{Http, Provider};
use once_cell::sync::OnceCell;

static STATE: OnceCell<DistributionState> = OnceCell::new();

#[derive(Debug)]
pub struct DistributionState {
    /// The folder from which music is served
    pub folder: MusicFolder,
    /// The port on which to serve
    pub port: u16,
    /// The wallet used for the iota ledger
    pub wallet: LocalWallet,
    /// The type-checked smart contract which signs transactions with the local wallet
    pub contract: SignedIotaMssContract,
    /// The raw client which can be used to send raw transactions.
    pub raw_client: Provider<Http>,
}

impl DistributionState {
    /// Initializes the global application state
    pub fn new(
        folder: &str,
        port: u16,
        key: &str,
        node_url: &str,
        contract_address: &str,
    ) -> eyre::Result<Self> {
        let raw_client = contract::new_raw(node_url)?;
        let wallet = wallet::from_hex_key(key)?;
        let folder = MusicFolder::new(folder);
        let contract = contract::new_signed(raw_client.clone(), wallet.clone(), contract_address);

        Ok(Self {
            folder,
            port,
            raw_client,
            wallet,
            contract,
        })
    }

    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }
}
