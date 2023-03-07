use crate::library::client::TangleTunesClient;
use crate::library::crypto::{self, Wallet};
use crate::library::database::Database;
use clap::Parser;
use eyre::Context;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub file: ConfigFile,
    pub args: Args,
}

impl Config {
    pub fn from_args() -> eyre::Result<Self> {
        let args = Args::parse();
        let config = std::fs::read_to_string(&args.config_path)
            .wrap_err(format!("Config does not exist at path {:?}", args))?;

        let config: ConfigFile = toml::from_str(&config)
            .wrap_err(format!("Could not parse config file at path {:?}", args))?;

        Ok(Self { file: config, args })
    }

    pub fn get_dir(&self) -> PathBuf {
        let mut config_path = PathBuf::from(&self.args.config_path);
        config_path.pop();
        config_path
    }

    pub async fn initialize_database(&self) -> eyre::Result<Database> {
        let mut path = self.get_dir();
        path.push(&self.file.database_path);
        Database::initialize(path).await
    }

    pub async fn initialize_client(&self, wallet: &Wallet) -> eyre::Result<TangleTunesClient> {
        Ok(
            TangleTunesClient::initialize(wallet, &self.file.node_url, &self.file.contract_address)
                .await?,
        )
    }

    pub async fn decrypt_wallet(&self, db: &Database) -> eyre::Result<Wallet> {
        if let Some((key, encrypted)) = db.get_key().await? {
            let key = match (encrypted, &self.args.password) {
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
pub struct ConfigFile {
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
pub struct Args {
    /// The path to the configuration file
    #[arg(short, long, default_value = "./TangleTunes.toml", global = true)]
    pub config_path: String,

    /// Then optional password for an encrypted private key
    #[arg(short, long, global = true)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    /// Generate a new wallet
    GenerateWallet {
        /// Required flag for creating a plaintext wallet
        #[arg(short = 'P', long)]
        plaintext: bool,
        /// The password used to encrypt the private key
        #[arg(short, long, required_unless_present("plaintext"))]
        password: Option<String>,
    },

    /// Import an existing wallet
    ImportWallet {
        /// The private key to import
        #[arg(short, long)]
        key: String,
        /// Required flag for creating a plaintext wallet
        #[arg(short = 'P', long)]
        plaintext: bool,
        /// The password used to encrypt the private key
        #[arg(short, long, required_unless_present("plaintext"))]
        password: Option<String>,
    },

    /// Export the IOTA address
    ExportAddress,

    /// Export the private key
    ExportPrivateKey {
        /// Required flag for plaintext export
        #[arg(short = 'P', long, required = true)]
        plaintext: bool,
    },

    /// Start distributing songs
    Run,

    /// Download songs from another distributor
    Download {
        /// The song-ids
        ids: Vec<String>,
        /// Whether to distribute this song
        #[arg(long, short, default_value_t = true)]
        distribute: bool,
    },

    /// Add songs from the file-system
    AddFromPath {
        /// The path where the song is stored as "{song_id}.mp3"
        paths: Vec<String>,
        /// Whether to distribute this song
        #[arg(long, short, default_value_t = true)]
        distribute: bool,
    },

    Remove {
        /// The song-ids
        song_ids: Vec<String>,
    },

    StopDistribution {
        /// The song-ids
        song_ids: Vec<String>,
    },

    StartDistribution {
        /// The song-ids
        song_ids: Vec<String>,
    },

    Listen {
        #[arg(long, short)]
        port: u16
    },
}