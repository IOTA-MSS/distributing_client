use crate::lib::{
    client::TangleTunesClient,
    crypto::{self, Wallet},
    database::Database,
};
use eyre::Context;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};

#[derive(Debug)]
pub struct App {
    pub password: Option<String>,
    pub port: u16,
    pub contract_address: String,
    pub node_url: String,
    pub database_path: PathBuf,
    pub chain_id: u16,
    pub database: Database,
    pub fee: u32,
}

impl App {
    pub fn chain_id(&self) -> u16 {
        self.chain_id
    }

    pub fn node_url(&self) -> &str {
        &self.node_url
    }

    pub fn contract_address(&self) -> &str {
        &self.contract_address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn from_config_file(
        config_path: String,
        password: Option<String>,
    ) -> eyre::Result<Self> {
        let file = std::fs::read_to_string(&config_path)
            .wrap_err(format!("Config does not exist at path {:?}", config_path))?;
        let cfg_file = toml::from_str::<ConfigFile>(&file).wrap_err(format!(
            "Could not parse config file at path {:?}",
            config_path
        ))?;

        let database_path = {
            let mut database_path = PathBuf::from(&config_path);
            database_path.pop();
            database_path.push(&cfg_file.database_path);
            database_path
        };

        let cfg = Self {
            password: password,
            port: cfg_file.port,
            contract_address: cfg_file.contract_address,
            node_url: cfg_file.node_url,
            database: Database::initialize(&database_path).await?,
            database_path,
            chain_id: cfg_file.chain_id,
            fee: cfg_file.fee,
        };

        Ok(cfg)
    }

    pub async fn initialize_client(
        &self,
        database: &Database,
    ) -> eyre::Result<&'static TangleTunesClient> {
        let wallet = self.decrypt_wallet(&database).await?;
        let client =
            TangleTunesClient::initialize(wallet, self.node_url(), self.contract_address()).await?;
        Ok(&*Box::leak(Box::new(client)))
    }

    pub async fn decrypt_wallet(&self, db: &Database) -> eyre::Result<Wallet> {
        if let Some((key, encrypted)) = db.get_key().await? {
            let key = match (encrypted, &self.password) {
                (true, Some(password)) => Ok(crypto::decrypt_private_key(&key, &password)?),
                (false, None) => Ok(key),
                (true, None) => Err(eyre!("Wallet is encrypted, please give a password.")),
                (false, Some(_)) => Err(eyre!("Wallet is not encrypted, no password needed.")),
            }?;
            Ok(Wallet::from_private_key(&key, self.chain_id())?)
        } else {
            Err(eyre!("No private key found. Import or generate one!"))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    port: u16,
    contract_address: String,
    node_url: String,
    database_path: String,
    chain_id: u16,
    fee: u32,
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
    pub config: String,

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

    DownloadLocal {
        /// The local port of the distributor
        #[arg(long, short)]
        distributor_port: u16,

        /// The id of the song to listen to
        #[arg(long, short)]
        song_id: String,

        /// The index to start listening at
        #[arg(long, short)]
        index: usize,

        /// The amount of chunks from index
        #[arg(long, short = 'C')]
        chunks: usize,

        /// The file-name to output into
        #[arg(long, short)]
        file: String,
    },

    CreateAccount {
        #[arg(long, short)]
        name: String,

        #[arg(long, short)]
        description: Option<String>,
    },

    DeleteAccount,

    Deposit {
        #[arg(long, short)]
        amount: u64,
    },

    Withdraw {
        #[arg(long, short)]
        amount: u64,
    },
}
