use clap::Parser;
use library::{
    app::{App, Args, Command},
    crypto,
};
use tokio::runtime::Runtime;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod account;
mod distribute;
mod download;
mod library;
mod songs;
mod wallet;

fn main() -> eyre::Result<()> {
    Runtime::new().unwrap().block_on(async move {
        color_eyre::install().unwrap();
        let args = Args::parse();
        let app = App::from_config_file(args.config, args.password).await?;
        _main(app, args.command).await
    })
}

async fn _main(config: App, command: Command) -> eyre::Result<()> {
    match command {
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
        Command::DownloadLocal {
            distributor_port,
            song_id,
            index,
            chunks,
            file,
        } => {
            crate::download::run(config, distributor_port, song_id, index, chunks, file).await?;
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

#[cfg(test)]
pub mod test {
    use crate::library::app::App;
    use std::path::PathBuf;

    pub const SONG_HEX_ID: &str =
        "0800000722040506080000072204050608000007220405060800000722040506";
    pub const SONG_ID: [u8; 32] = [
        8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34,
        4, 5, 6,
    ];
    pub const NODE_URL: &'static str = "http://localhost:9090/chains/tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s/evm";
    pub const CONTRACT_ADDRESS: &'static str = "0xAD3781Bd2FEC290b01c8C410eF6a7e8Baae632Db";
    pub const IOTA_CHAIN_ID: &'static str =
        "tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s";
    pub const CHAIN_ID: u16 = 1074;

    pub fn create_app() -> App {
        App {
            password: None,
            port: 3000,
            contract_address: CONTRACT_ADDRESS.to_string(),
            node_url: NODE_URL.to_string(),
            database_path: PathBuf::from("test/database"),
            chain_id: CHAIN_ID,
            database: todo!(),
        }
    }
}
