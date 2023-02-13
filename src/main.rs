mod chunks;

use ethers::{
    abi::Abi,
    contract::Contract,
    prelude::{k256::SecretKey, rand::rngs::ThreadRng},
    providers::Provider,
    signers::{LocalWallet, Signer},
    types::Transaction,
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::Http;
use once_cell::sync::OnceCell;
use std::fs::File;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    _main().await
}

static CHUNK_BYTES: usize = 32_766;

#[derive(Debug)]
struct AppState {
    music_folder: PathBuf,
    port: u16,
    wallet: LocalWallet,
    contract: Contract<Provider<Http>>,
}

static STATE: OnceCell<AppState> = OnceCell::new();

fn app_state() -> &'static AppState {
    STATE.get().unwrap()
}

fn music_folder() -> &'static PathBuf {
    &app_state().music_folder
}

fn create_new_wallet() -> LocalWallet {
    LocalWallet::new(&mut ThreadRng::default())
}

fn wallet_from_bytes(bytes: &[u8]) -> eyre::Result<LocalWallet> {
    let key = SecretKey::from_be_bytes(&bytes)?;
    Ok(LocalWallet::from(key))
}

fn get_provider(url: &str) -> eyre::Result<Provider<Http>> {
    Ok(Provider::try_from(url)?)
}

fn get_abi() -> eyre::Result<Abi> {
    Ok(serde_json::from_reader(File::open("abi/abi.json")?)?)
}

async fn _main() -> eyre::Result<()> {
    let wallet = create_new_wallet();
    STATE
        .set(AppState {
            music_folder: Path::new("mp3").into(),
            port: 3000,
            contract: Contract::new(wallet.address(), get_abi()?, get_provider("http://url")?),
            wallet,
        })
        .unwrap();

    let listener = TcpListener::bind(String::from("localhost:") + &app_state().port.to_string())
        .await
        .unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        let _child = tokio::task::spawn(async move {
            if let Err(e) = spawn_connection(stream, addr).await {
                println!("Error: {}", e)
            }
        });
    }
    Ok(())
}

async fn spawn_connection(mut stream: TcpStream, addr: SocketAddr) -> eyre::Result<()> {
    let mut raw_transaction = Vec::new();
    stream.read(&mut raw_transaction).await?;
    let transaction = <Transaction as Decodable>::decode(&Rlp::new(&raw_transaction))?;
    let data = transaction.input;
    let abi = Abi {
        constructor: todo!(),
        functions: todo!(),
        events: todo!(),
        errors: todo!(),
        receive: todo!(),
        fallback: todo!(),
    };
    Ok(())
}
