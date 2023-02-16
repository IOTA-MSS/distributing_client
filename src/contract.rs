use std::{str::FromStr, sync::Arc};

use ethers::{
    prelude::{abigen, k256::ecdsa::SigningKey, SignerMiddleware},
    signers::{LocalWallet, Wallet},
    types::Address,
};
use ethers_providers::{Http, Provider};

// Generates a custom type-safe contract
abigen! {
    IotaMssContract,
    "https://raw.githubusercontent.com/DanielMelero/IOTA-MSS/a2a7fe394601b672fd89fed4e3089b732aea5eaa/contract/Platform.abi";
}

pub type SignedIotaMssContract = IotaMssContract<SignerMiddleware<Provider<Http>, LocalWallet>>;


pub fn new_signed(
    raw_client: Provider<Http>,
    wallet: Wallet<SigningKey>,
    contract_address: &str,
) -> SignedIotaMssContract {
    IotaMssContract::new(
        Address::from_str(contract_address).unwrap(),
        Arc::new(SignerMiddleware::new(raw_client.clone(), wallet)),
    )
}

pub fn new_raw(node_url: &str) -> eyre::Result<Provider<Http>> {
    Ok(Provider::try_from(node_url)?)
}
