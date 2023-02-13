use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Iota-Mss distribution client",
    author = "Iota-Mss foundation",
    version = "0.0.1-beta.0",
    about = "A distribution client for Iota-Mss"
)]
pub enum Command {
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
        /// The port on which to serve.
        #[arg(short, long)]
        port: u16,

        /// The folder from which to serve.
        #[arg(short, long)]
        folder: String,

        /// Hexadecimally encoded secret key.
        #[arg(short, long)]
        key: String,

        /// The IotaMss server http address.
        #[arg(short, long)]
        iota_address: String,
    },
    /// Generate a new hex-encoded secret key for the wallet.
    GenerateKey,
}
