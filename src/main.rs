use args::{AccountCommand, Args, Command, SongsCommand, WalletCommand, SongIndexCommand};
use clap::Parser;
use config::ConfigFile;
use library::{app::AppData, crypto, database::Database};
use tokio::runtime::Runtime;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod account;
mod args;
mod config;
mod distribute;
mod library;
mod songs;
mod wallet;
mod song_index;

fn main() -> eyre::Result<()> {
    Runtime::new().unwrap().block_on(async move {
        color_eyre::install().unwrap();
        let args = Args::parse();
        let config = ConfigFile::from_path(&args.config)?;

        match &args.command {
            Command::Wallet(WalletCommand::Import {
                key,
                plaintext: _,
                password,
            }) => {
                let database = Database::initialize(&config.database_path).await?;
                crate::wallet::run_import(password.to_owned(), key.to_owned(), database).await
            }
            Command::Wallet(WalletCommand::Generate {
                plaintext: _,
                password,
            }) => {
                let database = Database::initialize(&config.database_path).await?;
                crate::wallet::run_generate(password.to_owned(), database).await
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
            WalletCommand::Import {
                key: _,
                plaintext: _,
                password: _,
            } => unreachable!(),
            WalletCommand::Remove => crate::wallet::run_remove(app).await,
            WalletCommand::Address => crate::wallet::run_export_address(app).await,
            WalletCommand::PrivateKey { plaintext: _ } => {
                crate::wallet::run_export_private_key(app).await
            }
            WalletCommand::Generate {
                plaintext: _,
                password,
            } => unreachable!(),
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
                crate::songs::run_download_direct(
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
            SongsCommand::Add { paths } => crate::songs::run_add(paths, false, app).await,
            SongsCommand::Remove { ids } => crate::songs::run_remove(ids, app).await,
            SongsCommand::SetFee { ids, fee } => crate::songs::run_set_fee(ids, fee, app).await,
            SongsCommand::List => crate::songs::run_list(app).await,
            SongsCommand::Download { song_id, to_file } => {
                crate::songs::run_download(app, song_id, to_file).await
            }
        },
        Command::Account(command) => match command {
            AccountCommand::Deposit { amount } => crate::account::run_deposit(amount, app).await,
            AccountCommand::Withdraw { amount } => crate::account::run_withdraw(amount, app).await,
            AccountCommand::Create { name, description } => {
                crate::account::run_create(name, description, app).await
            }
            AccountCommand::Delete => crate::account::run_delete(app).await,
        },
        Command::Distribute => crate::distribute::run_distribute(app).await,
        Command::SongIndex(command) => match command {
            SongIndexCommand::Update => crate::song_index::run_update(app).await,
            SongIndexCommand::Reset { no_update } => crate::song_index::run_reset(app, !no_update).await,
            SongIndexCommand::List => crate::song_index::run_list(app).await,
            SongIndexCommand::Download { amount, index: indexes } => crate::song_index::run_download(app, amount, indexes).await,
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
