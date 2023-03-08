use library::{
    config::{Command, Config},
    crypto,
};
use tokio::runtime::Runtime;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod account;
mod distribute;
mod library;
mod download;
mod songs;
mod wallet;

fn main() -> eyre::Result<()> {
    Runtime::new().unwrap().block_on(async move {
        color_eyre::install().unwrap();
        _main(Config::from_args()?).await
    })
}

async fn _main(config: Config) -> eyre::Result<()> {
    match config.args.command.clone() {
        Command::GenerateWallet {
            plaintext: _,
            password,
        } => {
            crate::wallet::generate(password, config).await?;
        }
        Command::ImportWallet {
            key,
            password,
            plaintext: _,
        } => {
            crate::wallet::import(password, key, config).await?;
        }
        Command::ExportAddress => {
            crate::wallet::export_address(config).await?;
        }
        Command::ExportPrivateKey { plaintext: _ } => {
            crate::wallet::export_private_key(config).await?;
        }

        Command::Run => {
            // tracing_subscriber::fmt::init();
            crate::distribute::run(config).await?;
        }
        Command::Listen { port } => {
            // tracing_subscriber::fmt::init();
            crate::download::run(config, port).await?;
        }

        Command::Download { ids, distribute } => {
            crate::songs::download(ids, distribute, config).await?;
        }
        Command::Remove { song_ids } => {
            crate::songs::remove(song_ids, config).await?;
        }
        Command::AddFromPath { paths, distribute } => {
            crate::songs::add_from_path(paths, distribute, config).await?;
        }
        Command::StopDistribution { song_ids } => {
            crate::songs::stop_distribution(song_ids, config).await?;
        }
        Command::StartDistribution { song_ids } => {
            crate::songs::start_distribution(song_ids, config).await?;
        }

        Command::CreateAccount { name, description } => {
            crate::account::create(name, description, config).await?;
        }
        Command::DeleteAccount => {
            crate::account::delete(config).await?;
        }
        Command::Deposit { amount } => {
            crate::account::deposit(amount, config).await?;
        }
        Command::Withdraw { amount } => {
            crate::account::withdraw(amount, config).await?;
        }
    };

    Ok(())
}

const BYTES_PER_CHUNK: u32 = 32_500;
const BYTES_PER_CHUNK_USIZE: usize = BYTES_PER_CHUNK as usize;
const TEST_SONG_HEX_ID: &str = "0800000722040506080000072204050608000007220405060800000722040506";
const TEST_SONG_ID_SLICE: [u8; 32] = [
    8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4,
    5, 6,
];
const TEST_NODE_URL: &'static str = "http://localhost:9090/chains/tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s/evm";
const TEST_CONTRACT_ADDRESS: &'static str = "0xAD3781Bd2FEC290b01c8C410eF6a7e8Baae632Db";
const CHAIN_ID_IOTA: &'static str =
    "tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s";
const CHAIN_ID_ETH: u16 = 1074;

type Chunk = Vec<u8>;
