use arguments::{
    AccountCommand, Arguments, Command, SongIndexCommand, SongsCommand, WalletCommand,
};
use clap::Parser;
use config::ConfigFile;
use ethers::types::U256;
use library::{app::AppData, crypto, database::Database};
use tokio::runtime::Runtime;
#[macro_use]
extern crate eyre;

mod arguments;
mod config;
mod library;

pub mod command;

fn main() -> eyre::Result<()> {
    Runtime::new().unwrap().block_on(async move {
        color_eyre::install().unwrap();
        let args = Arguments::parse();
        let config = ConfigFile::from_path(&args.config)?;

        match &args.command {
            Command::Wallet(WalletCommand::Import {
                key,
                plaintext: _,
                password,
            }) => {
                let database = Database::initialize(&config.database_path).await?;
                command::wallet::import(password.to_owned(), key.to_owned(), database).await
            }
            Command::Wallet(WalletCommand::Generate {
                plaintext: _,
                password,
            }) => {
                let database = Database::initialize(&config.database_path).await?;
                command::wallet::generate(password.to_owned(), database).await
            }
            _ => {
                let app = ConfigFile::from_path(&args.config)?
                    .parse_to_app_builder(args.password, &args.config)?
                    .build()
                    .await?;
                run_command(app, args.command).await
            }
        }
    })
}

async fn run_command(app: &'static AppData, command: Command) -> eyre::Result<()> {
    match command {
        Command::Wallet(command) => match command {
            WalletCommand::Remove => command::wallet::remove(app).await,
            WalletCommand::Address => command::wallet::export_address(app).await,
            WalletCommand::PrivateKey { plaintext: _ } => {
                command::wallet::export_private_key(app).await
            }
            WalletCommand::Import { .. } | WalletCommand::Generate { .. } => unreachable!(),
            WalletCommand::Balance => command::wallet::balance(app).await,
            WalletCommand::RequestFunds => command::wallet::request_funds(app).await,
        },
        Command::Songs(command) => match command {
            SongsCommand::DownloadDirect {
                ip,
                song,
                to_file,
                start,
                chunks,
                distributor_address,
            } => {
                command::songs::download_direct(
                    app,
                    ip,
                    song,
                    to_file,
                    start,
                    chunks,
                    distributor_address,
                )
                .await
            }
            SongsCommand::Add { paths } => command::songs::add(paths, app).await,
            SongsCommand::Remove { ids } => command::songs::remove(ids, app).await,
            SongsCommand::List => command::songs::run_list(app).await,
            SongsCommand::Download { song_id, to_file } => {
                command::songs::download(app, song_id, to_file, U256::MAX).await
            }
        },
        Command::Account(command) => match command {
            AccountCommand::Deposit { amount } => command::account::deposit(amount, app).await,
            AccountCommand::Withdraw { amount } => command::account::withdraw(amount, app).await,
            AccountCommand::Create { name, description } => {
                command::account::create(name, description, app).await
            }
            AccountCommand::Delete => command::account::delete(app).await,
            AccountCommand::View => command::account::view(app).await,
        },
        Command::Distribute { auto_download } => {
            command::distribute::run_distribute(app, auto_download).await
        }
        Command::SongIndex(command) => match command {
            SongIndexCommand::Update => {
                command::song_index::update(app).await?;
                Ok(())
            }
            SongIndexCommand::Reset { no_update } => {
                command::song_index::reset(app, !no_update).await
            }
            SongIndexCommand::List => command::song_index::list(app).await,
            SongIndexCommand::Download {
                amount,
                index: indexes,
            } => command::song_index::download(app, amount, indexes).await,
        },
    }
}

const BYTES_PER_CHUNK: u32 = 32_500;
const BYTES_PER_CHUNK_USIZE: usize = BYTES_PER_CHUNK as usize;

#[cfg(test)]
pub mod test {
    pub const UNVALIDATED_SONG_HEX_ID: &str =
        "0x0800000722040506080000072204050608000007220405060800000722040506";
    pub const VALIDATED_SONG_HEX_ID: &str =
        "0x486df48c7468457fc8fbbdc0cd1ce036b2b21e2f093559be3c37fcb024c1facf";
    pub const SONG_ID: [u8; 32] = [
        8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34, 4, 5, 6, 8, 0, 0, 7, 34,
        4, 5, 6,
    ];
    pub const NODE_URL: &'static str = "http://localhost:9090/chains/tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s/evm";
    pub const CONTRACT_ADDRESS: &'static str = "0xAD3781Bd2FEC290b01c8C410eF6a7e8Baae632Db";
    pub const IOTA_CHAIN_ID: &'static str =
        "tst1pzt0gue3mhz3pftwkqmxmyk8kv3mfzsn57erv20jemcrkjftktvuj5e0k6s";
    pub const CHAIN_ID: u16 = 1074;
}
