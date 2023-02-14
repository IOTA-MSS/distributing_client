use ethers::{
    prelude::{abigen, k256::SecretKey, rand::rngs::ThreadRng},
    signers::LocalWallet,
};



pub fn wallet_from_key(key: String) -> eyre::Result<LocalWallet> {
    let key = hex::decode(key)?;
    let key = SecretKey::from_be_bytes(&key)?;
    Ok(LocalWallet::from(key))
}

pub fn new_wallet() -> (String, LocalWallet) {
    let wallet = LocalWallet::new(&mut ThreadRng::default());
    let key = hex::encode(wallet.signer().to_bytes());
    (key, wallet)
}
