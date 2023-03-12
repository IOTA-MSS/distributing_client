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

    /// The password for encryption of private key
    #[arg(short, long, global = true)]
    pub password: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    /// Commands for managing your coupled IOTA wallet.
    #[command(subcommand)]
    Wallet(WalletCommand),

    /// Commands for managing songs.
    #[command(subcommand)]
    Songs(SongsCommand),

    /// Start distributing.
    Distribute,
}

#[derive(clap::Subcommand, Debug, Clone)]
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

#[derive(clap::Subcommand, Debug, Clone)]
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

#[derive(clap::Subcommand, Debug, Clone)]
pub enum SongsCommand {
    /// Download a song from another distributor
    Download {
        /// The ip-address of the distributor
        #[arg(long)]
        ip: String,

        /// The song-id to download
        #[arg(long)]
        song: String,

        /// Optionally write the output to a file instead of the database
        #[arg(long)]
        to_file: Option<String>,

        /// The chunk to start at
        #[arg(long, requires = "to_file", default_value_t = 0)]
        start: usize,

        /// The amount of chunks
        #[arg(long, requires = "to_file", default_value_t = usize::MAX)]
        chunks: usize,
    },

    /// Add a song from the file-system
    Add {
        /// The paths to find the songs stored as "{(0x)0AC..34}.mp3"
        paths: Vec<String>,

        /// Do not distribute this song
        #[arg(long, short)]
        no_distribute: bool,
    },

    /// Remove a song from the database
    Remove {
        /// The songs to selected
        ids: Vec<String>,
    },

    /// Set a song to be distributed
    StartDistribution {
        /// The songs to selected
        ids: Vec<String>,
    },

    /// Set a song to not be distributed
    StopDistribution {
        /// The songs to selected
        ids: Vec<String>,
    },

    /// Set the distribution-fee for a given song
    SetFee {
        /// The songs to selected
        ids: Vec<String>,

        /// The distribution-fee for the given songs
        #[arg(long, short)]
        fee: u32,
    },

    /// List all songs in the database
    List,
}
