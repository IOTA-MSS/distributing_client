mod args;
mod chunks;
mod iota;
mod state;

use args::Command;
use clap::Parser;
use ethers::{
    abi::Abi,
    types::Transaction,
    utils::rlp::{Decodable, Rlp},
};
use iota::new_wallet;
use state::{state, DistributionState};
use std::{fs::File, net::SocketAddr, path::Path};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tracing::warn;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    match Command::parse() {
        args::Command::Add { folder, song_ids } => add_songs(folder, song_ids).await,
        args::Command::Remove { folder, song_ids } => remove_songs(folder, song_ids),
        args::Command::GenerateKey => generate_key(),
        args::Command::Distribute {
            port,
            folder,
            key,
            iota_address,
        } => serve(folder, port, key, iota_address).await,
    }
}

async fn add_songs(folder: String, ids: Vec<String>) -> eyre::Result<()> {
    todo!()
}

fn remove_songs(folder: String, ids: Vec<String>) -> eyre::Result<()> {
    for id in &ids {
        let path = Path::new(&folder).join(Path::new(id));
        if let Err(e) = std::fs::remove_file(&path) {
            println!("{e}: {:?}", path);
        }
    }
    Ok(())
}

fn generate_key() -> eyre::Result<()> {
    let (key, _wallet) = new_wallet();
    println!("Your secret key: {key}");
    Ok(())
}

async fn serve(
    folder: String,
    port: u16,
    secret_key: String,
    iota_address: String,
) -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    DistributionState::init(&folder, port, secret_key, &iota_address)?;

    let listener = TcpListener::bind(String::from("localhost:") + &state().port.to_string())
        .await
        .unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        let _ = tokio::task::spawn(async move {
            if let Err(e) = handle_incoming_tcp_connection(stream, addr).await {
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
) -> eyre::Result<()> {
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
