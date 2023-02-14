mod args;
mod music_folder;
mod wallet;
mod state;
mod contract;

use args::Command;
use music_folder::MusicFolder;
use clap::Parser;
use ethers::{
    abi::Abi,
    types::Transaction,
    utils::rlp::{Decodable, Rlp},
};
use wallet::new_wallet;
use state::State;
use std::{net::SocketAddr, path::Path};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::warn;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    match Command::parse() {
        args::Command::Add { folder, song_ids } => command_add_songs(folder, song_ids).await,
        args::Command::Remove { folder, song_ids } => command_remove_songs(folder, song_ids),
        args::Command::Generate => command_generate_key(),
        args::Command::Distribute {
            folder,
            port,
            key,
            iota_address,
        } => command_distribute(folder, port, key, iota_address).await,
    }
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
    let (key, _wallet) = new_wallet();
    println!("Your secret key is: \"{key}\"");
    Ok(())
}

async fn command_distribute(
    folder: String,
    port: u16,
    secret_key: String,
    iota_address: String,
) -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();
    State::init(&folder, port, secret_key, &iota_address)?;

    let listener = TcpListener::bind(String::from("localhost:") + &State::get().port.to_string())
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

    let chunks = State::folder().read_chunks("song_id", 10, 10)?;
    stream.write(&chunks).await?;
    stream.flush().await?;

    Ok(())
}
