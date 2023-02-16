mod args;
mod contract;
mod listener;
mod music_folder;
mod state;
mod wallet;

use args::Command;
use clap::Parser;
use contract::IotaMssContract;
use ethers::{
    abi::Abi,
    types::Transaction,
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::Provider;
use music_folder::MusicFolder;
use state::DistributionState;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::warn;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    match Command::parse() {
        Command::Add { folder, song_ids } => command_add_songs(folder, song_ids).await,
        Command::Remove { folder, song_ids } => command_remove_songs(folder, song_ids),
        Command::Generate => command_generate_key(),
        Command::Distribute {
            folder,
            port,
            key,
            node_url,
            contract_address,
        } => command_distribute(folder, port, key, node_url, contract_address).await,
        Command::TestClient {
            port,
            node_url,
            key,
            contract_address,
        } => command_test_client(port, key, node_url, contract_address).await,
    }
}

async fn command_test_client(
    port: u16,
    key: String,
    node_url: String,
    contract_address: String,
) -> eyre::Result<()> {
    // State::init(None, None, key, &node_url, &contract_address)?;
    let stream = TcpStream::connect(("127.0.0.1", port)).await?;
    Ok(())
}

async fn command_add_songs(folder: String, ids: Vec<String>) -> eyre::Result<()> {
    todo!()
}

fn command_remove_songs(folder: String, ids: Vec<String>) -> eyre::Result<()> {
    let folder = MusicFolder::new(&folder);
    for id in &ids {
        if let Err(error) = folder.remove_song(id) {
            println!("{}", error);
        }
    }
    Ok(())
}

fn command_generate_key() -> eyre::Result<()> {
    let wallet = wallet::new_rand();
    let key = wallet::to_hex_key(&wallet);
    println!("Your secret key is: \"{key}\"");
    Ok(())
}

async fn command_distribute(
    folder: String,
    port: u16,
    secret_key: String,
    node_url: String,
    contract_address: String,
) -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let state =
        DistributionState::new(&folder, port, &secret_key, &node_url, &contract_address)?.leak();

    let listener = TcpListener::bind(("127.0.0.1", state.port)).await.unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        let _ = tokio::task::spawn(async move {
            if let Err(e) = handle_incoming_tcp_connection(stream, addr, state).await {
                warn!("Handler exited with an error: {}", e);
            }
        });
    }

    Ok(())
}

/// Handles any incoming tcp connections from listening-clients.
async fn handle_incoming_tcp_connection(
    mut stream: TcpStream,
    _addr: SocketAddr,
    state: &DistributionState,
) -> eyre::Result<()> {
    let bytes_read = {
        let mut bytes_read = Vec::with_capacity(256);
        stream.read(&mut bytes_read).await?;
        bytes_read
    };

    // let call: ContractCall<Provider<Http>, _> = state().contract.undistribute(todo!());
    // let pending_tx = call.gas(10)..await.unwrap();

    let transaction = Transaction::decode(&Rlp::new(&bytes_read))?;

    let data = transaction.input;
    let abi = Abi {
        constructor: todo!(),
        functions: todo!(),
        events: todo!(),
        errors: todo!(),
        receive: todo!(),
        fallback: todo!(),
    };

    let chunks = state.folder.read_chunks("song_id", 10, 10)?;
    stream.write(&chunks).await?;
    stream.flush().await?;

    Ok(())
}
