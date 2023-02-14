use std::{str::FromStr, sync::Arc};

use ethers::{
    prelude::{abigen, k256::ecdsa::SigningKey, SignerMiddleware},
    signers::{LocalWallet, Wallet},
    types::Address,
};
use ethers_providers::{Http, Provider};

static CONTRACT_ADDRESS: &'static str = "TODO";

// Generates a custom type-safe contract
abigen! {
    IotaMssContract,
    "https://raw.githubusercontent.com/DanielMelero/IOTA-MSS/a2a7fe394601b672fd89fed4e3089b732aea5eaa/contract/Platform.abi";
}

pub type SignedIotaMssContract = IotaMssContract<SignerMiddleware<Provider<Http>, LocalWallet>>;

impl SignedIotaMssContract {
    pub fn new_signed(raw_client: Provider<Http>, wallet: Wallet<SigningKey>) -> Self {
        IotaMssContract::new(
            Address::from_str(CONTRACT_ADDRESS).unwrap(),
            Arc::new(SignerMiddleware::new(raw_client.clone(), wallet)),
        )
    }
}
