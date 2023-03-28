use ethers::types::U256;

use crate::library::{
    client::TangleTunesClient,
    crypto::{self, Wallet},
    database::Database,
};
use std::{fmt::Debug, net::SocketAddr, path::PathBuf};

use super::client::WEI_PER_IOTA;

const DEFAULT_MAX_PRICE: u128 = WEI_PER_IOTA * 1_000_000; // 1 million iota

#[derive(Debug)]
pub struct AppData {
    pub password: Option<String>,
    pub contract_address: String,
    pub node_url: String,
    pub database_path: PathBuf,
    pub chain_id: u16,
    pub fee: U256,
    pub database: Database,
    pub client: TangleTunesClient,
    pub server_address: SocketAddr,
    pub bind_address: SocketAddr,
    pub max_price_wei: U256,
}

impl AppData {
    /// Updates the internal song-list with data from the smart contract.
    pub async fn update_song_list(&self) -> eyre::Result<()> {
        let index = self.database.get_next_song_index().await?;
        let new_ids = self.client.get_song_ids_from_index(index).await?;
        self.database.add_to_song_index(&new_ids).await?;
        Ok(())
    }

    pub async fn reset_song_list(&self) -> eyre::Result<()> {
        self.database.clear_song_index().await?;
        self.update_song_list().await?;
        Ok(())
    }
}

pub struct AppDataBuilder {
    pub contract_address: String,
    pub node_url: String,
    pub database_path: String,
    pub chain_id: u16,
    pub fee: u32,
    pub password: Option<String>,
    pub server_address: SocketAddr,
    pub bind_address: SocketAddr,
    pub max_price_iota: Option<u64>,
}

impl AppDataBuilder {
    pub async fn build(self) -> eyre::Result<&'static AppData> {
        Self::_build(self, false).await
    }

    async fn _build(self, in_memory: bool) -> eyre::Result<&'static AppData> {
        let database = if in_memory {
            Database::initialize_in_memory().await?
        } else {
            Database::initialize(&self.database_path).await?
        };

        let wallet = {
            if let Some((key, encrypted)) = database.get_key().await? {
                let key = match (encrypted, &self.password) {
                    (true, Some(password)) => Ok(crypto::decrypt_private_key(&key, password)?),
                    (false, None) => Ok(key),
                    (true, None) => Err(eyre!("Wallet is encrypted, please give a password.")),
                    (false, Some(_)) => Err(eyre!("Wallet is not encrypted, no password needed.")),
                }?;
                Ok(Wallet::from_private_key(&key, self.chain_id)?)
            } else {
                Err(eyre!("No private key found. Import or generate one!"))
            }
        }?;

        let client =
            TangleTunesClient::initialize(wallet, &self.node_url, &self.contract_address).await?;

        let max_price_wei = match self.max_price_iota {
            Some(max_price) => ((max_price as u128) * WEI_PER_IOTA).into(),
            None => DEFAULT_MAX_PRICE.into(),
        };

        let app = AppData {
            password: self.password,
            contract_address: self.contract_address,
            node_url: self.node_url,
            database,
            database_path: PathBuf::from(self.database_path),
            chain_id: self.chain_id,
            fee: self.fee.into(),
            client,
            server_address: self.server_address,
            bind_address: self.bind_address,
            max_price_wei,
        };

        Ok(Box::leak(Box::new(app)))
    }
}

#[cfg(test)]
pub mod test {
    use super::{AppData, AppDataBuilder};
    use crate::config::ConfigFile;
    use eyre::Context;

    impl AppData {
        /// Overrides:
        /// - database_path to ":memory:" (in memory database)
        /// - ip_address to "127.0.0.1"
        pub async fn init_for_test(
            port: Option<u16>,
            in_memory: bool,
        ) -> eyre::Result<&'static AppData> {
            let mut builder = ConfigFile::from_path("TangleTunes.toml")
                .wrap_err("Cannot run tests without config file at ./TangleTunes.toml")?
                .parse_to_app_builder(None, "TangleTunes.toml")?;

            if let Some(port) = port {
                builder.bind_address = format!("127.0.0.1:{port}").parse()?;
            }

            AppDataBuilder::_build(builder, in_memory).await
        }
    }
}
