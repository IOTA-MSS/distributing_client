use clap::Parser;

static DEFAULT_PORT: &'static str = "3000";
static DEFAULT_NODE_URL: &'static str = "http://default_node_url.com";
static DEFAULT_CONTRACT_ADDRESS: &'static str = "default_contract_address";

#[derive(Parser, Debug)]
#[command(
    name = "Iota-Mss distribution client",
    author = "Iota-Mss foundation",
    version = "0.0.1-beta.0",
    about = "A distribution client for Iota-Mss"
)]
pub enum Command {
    TestClient {
        /// Hexadecimally encoded secret key.
        #[arg(short, long)]
        key: String,

        /// The port to which to connect.
        #[arg(short, long, default_value = DEFAULT_PORT)]
        port: u16,

        /// The IotaMss server http address.
        #[arg(short, long, default_value = DEFAULT_PORT)]
        node_url: String,

        /// The smart-contract's address to connect to.
        #[arg(short, long, default_value = DEFAULT_CONTRACT_ADDRESS)]
        contract_address: String,
    },
    /// Download and add a song for distribution.
    Add {
        /// The folder into which to download the song.
        #[arg(short, long)]
        folder: String,

        /// The song-ids to add.
        #[arg(required = true)]
        song_ids: Vec<String>,
    },
    /// Remove a song from distribution.
    Remove {
        /// The folder from which to remove the song.
        #[arg(short, long)]
        folder: String,

        /// The song-ids to remove.
        #[arg(required = true)]
        song_ids: Vec<String>,
    },
    /// Start a server to distributing.
    Distribute {
        /// The folder from which to serve.
        #[arg(short, long)]
        folder: String,

        /// Hexadecimally encoded secret key.
        #[arg(short, long)]
        key: String,

        /// The port on which to serve.
        #[arg(short, long, default_value = DEFAULT_PORT)]
        port: u16,

        /// The IotaMss server http address.
        #[arg(short, long, default_value = DEFAULT_NODE_URL)]
        node_url: String,

        /// The smart-contract's address to connect to.
        #[arg(short, long, default_value = DEFAULT_CONTRACT_ADDRESS)]
        contract_address: String,
    },
    /// Generate a new hex-encoded secret key for the wallet.
    Generate,
}
