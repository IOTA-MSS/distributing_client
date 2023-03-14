use ethers::{
    prelude::{k256::SecretKey, rand::rngs::ThreadRng},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::Context;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::library::util::{to_hex_prefix, try_from_hex_prefix};

/// A wrapper around the `LocalWallet`.
#[derive(Clone, Debug)]
pub struct Wallet {
    wallet: LocalWallet,
}

impl Wallet {
    /// Create the wallet from the hex-encoded secret key.
    pub fn from_private_key(secret: &str, chain_id: u16) -> eyre::Result<Self> {
        let key = SecretKey::from_be_bytes(&try_from_hex_prefix::<Vec<u8>>(secret)?)?;
        Ok(Self {
            wallet: LocalWallet::from(key).with_chain_id(chain_id),
        })
    }

    /// Generate a new wallet with a random secret key.
    pub fn generate(chain_id: u16) -> Self {
        Self {
            wallet: LocalWallet::new(&mut ThreadRng::default()).with_chain_id(chain_id),
        }
    }

    /// Get the inner `LocalWallet`.
    pub fn local_wallet(&self) -> &LocalWallet {
        &self.wallet
    }

    /// Get the hex-encoded private key of this wallet.
    pub fn private_key(&self) -> String {
        to_hex_prefix(self.wallet.signer().to_bytes())
    }

    /// Get the address of this wallet.
    pub fn address(&self) -> Address {
        self.wallet.address()
    }
}

/// Encrypt the hex-encoded secret key with the password.
pub fn encrypt_private_key(secret_key: &str, password: &str) -> String {
    new_magic_crypt!(password, 256).encrypt_bytes_to_base64(secret_key)
}

/// Encrypt the encrypted secret key with the password to a hex-encoded secret key.
pub fn decrypt_private_key(encrypted_key: &str, password: &str) -> eyre::Result<String> {
    Ok(new_magic_crypt!(password, 256)
        .decrypt_base64_to_string(encrypted_key)
        .wrap_err("Incorrect password")?)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test;

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
        let wallet1 = Wallet::generate(test::CHAIN_ID);
        let wallet2 = Wallet::from_private_key(&wallet1.private_key(), test::CHAIN_ID).unwrap();

        assert_eq!(wallet1.address(), wallet2.address());
        assert_eq!(wallet1.private_key(), wallet2.private_key());
    }
}
