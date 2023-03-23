use serde::{Deserialize, Serialize};

#[derive(clap::Parser, Debug, Clone, Serialize, Deserialize)]
#[command(
    name = "TangleTunes distribution client",
    author = "The TangleTunes foundation",
    version = "0.0.1-beta.0",
    about = "A distribution client for TangleTunes"
)]
pub struct Arguments {
    /// The path to the configuration file
    #[arg(short, long, default_value = "./TangleTunes.toml", global = true)]
    pub config: String,

    /// The password for encryption of private key
    #[arg(short, long, global = true)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    /// Manage your IOTA wallet
    #[command(subcommand)]
    Wallet(WalletCommand),

    /// Manage your TangleTunes account
    #[command(subcommand)]
    Account(AccountCommand),

    /// Manage your downloaded songs
    #[command(subcommand)]
    Songs(SongsCommand),

    /// Manage the local song-index
    #[command(subcommand)]
    SongIndex(SongIndexCommand),

    /// Start distributing.
    Distribute {
        /// Automatically download and distribute songs from other distributors
        #[arg(long)]
        auto_download: bool,
    },
}

#[derive(clap::Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum WalletCommand {
    /// Import a wallet with the given private key
    Import {
        /// The private key to import
        key: String,

        /// This command uses a UNENCRYPTED private key!
        #[arg(long)]
        plaintext: bool,

        /// The password used to encrypt the private key
        #[arg(long, short = 'k', required_unless_present("plaintext"))]
        password: Option<String>,
    },

    /// Generate a new wallet with randomized private key
    Generate {
        /// This command uses a UNENCRYPTED private key!
        #[arg(long)]
        plaintext: bool,

        /// The password used to encrypt the private key
        #[arg(long, short = 'k', required_unless_present("plaintext"))]
        password: Option<String>,
    },

    /// Remove the current wallet
    Remove,

    /// Export the address
    Address,

    /// Export the private key
    PrivateKey {
        /// This command uses a UNENCRYPTED private key!
        #[arg(long, required = true)]
        plaintext: bool,
    },
}

#[derive(clap::Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum AccountCommand {
    /// Deposit into your account from your wallet
    Deposit {
        /// The amount to deposit
        amount: u64,
    },

    /// Withdraw from your account into your wallet
    Withdraw {
        /// The amount to withdraw
        amount: u64,
    },

    /// Create a new account coupled to your wallet
    Create {
        /// The account name
        #[arg(long, short)]
        name: String,

        /// The account description
        #[arg(long, short)]
        description: Option<String>,
    },

    /// Delete the account coupled to your wallet
    Delete,
}

#[derive(clap::Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum SongIndexCommand {
    /// Update the list of songs from the smart-contract
    Update,

    /// Reset the list of songs from the smart-contract
    Reset {
        #[arg(long)]
        no_update: bool,
    },

    /// List all songs
    List,

    /// Download indexed songs from another distributor.
    Download {
        /// Download `amount` random songs not yet downloaded.
        #[arg(long)]
        amount: Option<usize>,

        /// The ids of the songs to be downloaded
        #[arg(long)]
        index: Option<Vec<usize>>,
    },
}

#[derive(clap::Subcommand, Debug, Clone, Serialize, Deserialize)]
pub enum SongsCommand {
    /// Download chunks from a distributor's ip-address
    DownloadDirect {
        /// The ip-address of the distributor
        #[arg(long)]
        ip: String,

        /// The song-id to download
        #[arg(long)]
        song: String,

        /// Optionally write the output to a file instead of the database
        #[arg(long)]
        to_file: String,

        /// The chunk to start at
        #[arg(long, default_value_t = 0)]
        start: usize,

        /// The amount of chunks
        #[arg(long)]
        chunks: usize,

        /// The distributor address
        #[arg(long)]
        distributor_address: String,
    },

    /// Download a song from a random distributor
    Download {
        /// The song-id to download
        #[arg(long)]
        song_id: String,

        /// The file to download to
        #[arg(long)]
        to_file: Option<String>,
    },

    /// Add a song from the file-system
    Add {
        /// The paths to find the songs stored as "{(0x)0AC..34}.mp3"
        paths: Vec<String>,
        // /// Do not distribute this song
        // #[arg(long, short)]
        // no_distribute: bool,
    },

    /// Remove a song from the database
    Remove {
        /// The songs to selected
        ids: Vec<String>,
    },

    /// List all songs in the database
    List,
}
