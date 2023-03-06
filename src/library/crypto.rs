use ethers::{
    prelude::{k256::SecretKey, rand::rngs::ThreadRng},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::Context;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

static CHAIN_ID_IOTA: &'static str =
    "tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s";
static CHAIN_ID_ETH: u16 = 9999;

#[derive(Clone, Debug)]
pub struct Wallet {
    wallet: LocalWallet,
}

impl Wallet {
    pub fn from_private_key(secret: &str) -> eyre::Result<Self> {
        let key = SecretKey::from_be_bytes(&hex::decode(secret)?)?;
        Ok(Self {
            wallet: LocalWallet::from(key).with_chain_id(CHAIN_ID_ETH),
        })
    }

    pub fn generate() -> Self {
        Self {
            wallet: LocalWallet::new(&mut ThreadRng::default()).with_chain_id(CHAIN_ID_ETH),
        }
    }

    pub fn inner(&self) -> &LocalWallet {
        &self.wallet
    }

    pub fn private_key(&self) -> String {
        hex::encode(self.wallet.signer().to_bytes())
    }

    pub fn address(&self) -> Address {
        self.wallet.address()
    }
}

pub fn encrypt_private_key(secret_key: &str, password: &str) -> String {
    new_magic_crypt!(password, 256).encrypt_bytes_to_base64(secret_key)
}

pub fn decrypt_private_key(encrypted_key: &str, password: &str) -> eyre::Result<String> {
    Ok(new_magic_crypt!(password, 256)
        .decrypt_base64_to_string(encrypted_key)
        .wrap_err("Incorrect password")?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encrypt_decrypt_correct() {
        let password = "myPassword";
        let key = "29227eb1efdb4057589c05c4dd01683c0634fccb3ff2a0635ed4f449eac40b1b";

        let encrypted_key = encrypt_private_key(key, password);
        let original_key = decrypt_private_key(&encrypted_key, password).unwrap();

        assert_eq!(key, original_key);
    }

    #[test]
    fn encrypt_decrypt_incorrect() {
        let password = "myPassword";
        let key = "29227eb1efdb4057589c05c4dd01683c0634fccb3ff2a0635ed4f449eac40b1b";

        let encrypted_key = encrypt_private_key(key, password);
        assert!(decrypt_private_key(&encrypted_key, "incorrectPassword").is_err());
    }

    #[test]
    fn generating_and_importing() {
        let wallet1 = Wallet::generate();
        let wallet2 = Wallet::from_private_key(&wallet1.private_key()).unwrap();

        assert_eq!(wallet1.address(), wallet2.address());
        assert_eq!(wallet1.private_key(), wallet2.private_key());
    }
}
