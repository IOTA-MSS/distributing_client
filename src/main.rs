use args::{AccountCommand, Args, Command, SongsCommand, WalletCommand};
use clap::Parser;
use library::{app::App, crypto};
use tokio::runtime::Runtime;
#[macro_use]
extern crate eyre;
#[macro_use]
extern crate tracing;

mod account;
mod args;
mod distribute;
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

async fn _main(app: App, command: Command) -> eyre::Result<()> {
    match command {
        Command::Wallet(command) => match command {
            WalletCommand::Import {
                key,
                plaintext: _,
                password,
            } => crate::wallet::import(password, key, app).await,
            WalletCommand::Remove => crate::wallet::remove(app).await,
            WalletCommand::Address => crate::wallet::export_address(app).await,
            WalletCommand::PrivateKey { plaintext: _ } => {
                crate::wallet::export_private_key(app).await
            }
            WalletCommand::Generate {
                plaintext: _,
                password,
            } => crate::wallet::generate(password, app).await,
        },
        Command::Songs(command) => match command {
            SongsCommand::Download {
                ip,
                song,
                to_file,
                start,
                chunks,
            } => {
                let to_file = to_file.map(|file| (file, start, chunks));
                crate::songs::download(app, ip, song, to_file).await
            }
            SongsCommand::Add {
                paths,
                no_distribute,
            } => crate::songs::add_from_path(paths, !no_distribute, app).await,
            SongsCommand::Remove { ids } => crate::songs::remove(ids, app).await,
            SongsCommand::StartDistribution { ids } => {
                crate::songs::start_distribution(ids, app).await
            }
            SongsCommand::StopDistribution { ids } => {
                crate::songs::stop_distribution(ids, app).await
            }
            SongsCommand::SetFee { ids, fee } => crate::songs::set_fee(ids, fee, app).await,
            SongsCommand::List => crate::songs::list(app).await,
        },
        Command::Account(command) => match command {
            AccountCommand::Deposit { amount } => crate::account::deposit(amount, app).await,
            AccountCommand::Withdraw { amount } => crate::account::withdraw(amount, app).await,
            AccountCommand::Create { name, description } => {
                crate::account::create(name, description, app).await
            }
            AccountCommand::Delete => crate::account::delete(app).await,
        },
        Command::Distribute => crate::distribute::run(app).await,
    }
}

const BYTES_PER_CHUNK: u32 = 32_500;
const BYTES_PER_CHUNK_USIZE: usize = BYTES_PER_CHUNK as usize;

pub mod util {
    use ethers::utils::hex::{FromHex, ToHex};
    use std::{
        error::Error,
        fmt::{Debug, Display},
        ops::{Deref, DerefMut},
    };

    #[derive(Clone)]
    pub struct SongId([u8; 32]);

    impl TryFrom<Vec<u8>> for SongId {
        type Error = eyre::Report;

        fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
            Ok(Self(value.try_into().map_err(|value| {
                eyre!("Couldn't convert vec to song-id: {value:?}")
            })?))
        }
    }

    impl From<[u8; 32]> for SongId {
        fn from(value: [u8; 32]) -> Self {
            Self(value)
        }
    }

    impl Into<[u8; 32]> for SongId {
        fn into(self) -> [u8; 32] {
            self.0
        }
    }

    impl Debug for SongId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("SongId").field(&self.to_hex()).finish()
        }
    }

    impl Display for SongId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.to_hex())
        }
    }

    impl Deref for SongId {
        type Target = [u8; 32];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for SongId {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl SongId {
        pub fn try_from_hex(hex: impl AsRef<str>) -> eyre::Result<Self> {
            Ok(Self(try_from_hex_prefix(hex)?))
        }

        pub fn to_hex(&self) -> String {
            to_hex_prefix(&self.0)
        }
    }

    pub fn to_hex_prefix(bytes: impl AsRef<[u8]>) -> String {
        format!("0x{}", ToHex::encode_hex::<String>(&bytes))
    }

    pub fn try_from_hex_prefix<T>(hex: impl AsRef<str>) -> eyre::Result<T>
    where
        T: FromHex,
        T::Error: Error + Send + Sync + 'static,
    {
        match hex.as_ref().strip_prefix("0x") {
            Some(hex) => Ok(FromHex::from_hex(hex)?),
            None => Ok(FromHex::from_hex(hex.as_ref())?),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::library::app::App;
    use std::path::PathBuf;

    pub const UNVALIDATED_SONG_HEX_ID: &str =
        "0x0800000722040506080000072204050608000007220405060800000722040506";
    pub const VALIDATED_SONG_HEX_ID: &str =
        "0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2";
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
            fee: 1,
        }
    }
}
