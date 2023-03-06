use crate::library::client::TangleTunesClient;
use crate::library::crypto::{self, Wallet};
use crate::library::database::Database;
use eyre::Context;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub path_to_config: String,
    pub port: u16,
    pub contract_address: String,
    pub node_url: String,
    pub database_path: String,
}

impl Config {
    pub fn from_file(path: &str) -> eyre::Result<Self> {
        let config = std::fs::read_to_string(&path)
            .wrap_err(format!("Config does not exist at path {:?}", path))?;

        let config: TomlConfig = toml::from_str(&config)
            .wrap_err(format!("Could not parse config file at path {:?}", path))?;

        Ok(Self {
            path_to_config: path.to_string(),
            port: config.port,
            contract_address: config.contract_address,
            node_url: config.node_url,
            database_path: config.database_path,
        })
    }

    pub fn get_dir(&self) -> PathBuf {
        let mut config_path = PathBuf::from(&self.path_to_config);
        config_path.pop();
        config_path
    }

    pub async fn initialize_database(&self) -> eyre::Result<Database> {
        let mut path = self.get_dir();
        path.push(&self.database_path);
        Database::initialize(path).await
    }

    pub async fn initialize_client(&self, wallet: &Wallet) -> eyre::Result<TangleTunesClient> {
        Ok(TangleTunesClient::init(wallet, &self.node_url, &self.contract_address).await?)
    }

    pub async fn decrypt_wallet(
        &self,
        db: &Database,
        password: Option<&String>,
    ) -> eyre::Result<Wallet> {
        if let Some((key, encrypted)) = db.get_key().await? {
            let key = match (encrypted, password) {
                (true, Some(password)) => Ok(crypto::decrypt_private_key(&key, &password)?),
                (false, None) => Ok(key),
                (true, None) => Err(eyre!("Wallet is encrypted, please give a password.")),
                (false, Some(_)) => Err(eyre!("Wallet is not encrypted, no password needed.")),
            }?;
            Ok(Wallet::from_private_key(&key)?)
        } else {
            Err(eyre!("No wallet coupled."))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TomlConfig {
    pub port: u16,
    pub contract_address: String,
    pub node_url: String,
    pub database_path: String,
}

#[derive(clap::Parser, Debug, Clone)]
#[command(
    name = "TangleTunes distribution client",
    author = "The TangleTunes foundation",
    version = "0.0.1-beta.0",
    about = "A distribution client for TangleTunes"
)]
pub struct Arguments {
    /// The path to the configuration file
    #[arg(short, long, default_value = "./TangleTunes.toml")]
    pub config_path: String,

    /// Then optional password for an encrypted private key
    #[arg(short, long)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    ExportAddress,
    Listen,
    Distribute,
    AddSongs {
        /// The song-ids to add
        #[arg(required = true)]
        song_ids: Vec<String>,
    },
    RemoveSongs {
        /// The song-ids to remove
        #[arg(required = true)]
        song_ids: Vec<String>,
    },
    /// Set a new key for the wallet encrypted by password
    ImportWallet {
        /// Required flag for creating a plaintext wallet
        #[arg(short = 'P', long)]
        plaintext: bool,
        /// The private key to import
        #[arg(short, long)]
        key: String,
        /// The password used to encrypt the private key
        #[arg(short, long, required_unless_present("plaintext"))]
        password: Option<String>,
    },
    GenerateWallet {
        /// Required flag for creating a plaintext wallet
        #[arg(short = 'P', long)]
        plaintext: bool,
        /// The password used to encrypt the private key
        #[arg(short, long, required_unless_present("plaintext"))]
        password: Option<String>,
    },
}
