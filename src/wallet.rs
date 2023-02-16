use ethers::{
    prelude::{k256::SecretKey, rand::rngs::ThreadRng},
    signers::LocalWallet,
};

pub fn from_hex_key(key: &str) -> eyre::Result<LocalWallet> {
    let key = SecretKey::from_be_bytes(&hex::decode(key)?)?;
    let wallet = LocalWallet::from(key);
    Ok(wallet)
}

pub fn new_rand() -> LocalWallet {
    LocalWallet::new(&mut ThreadRng::default())
}

pub fn to_hex_key(wallet: &LocalWallet) -> String {
    hex::encode(wallet.signer().to_bytes())
}
