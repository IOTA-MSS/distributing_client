use std::io::Cursor;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{library::config::Config, BYTES_PER_CHUNK, TEST_SONG_HEX_ID};

pub async fn run(cfg: Config, port: u16) -> eyre::Result<()> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await?;
    let database = cfg.initialize_database().await?;
    let wallet = cfg.decrypt_wallet(&database).await?;
    let client = &*Box::leak(Box::new(cfg.initialize_client(&wallet).await?));
    let mut buf = Vec::with_capacity(50 * BYTES_PER_CHUNK as usize);

    loop {
        // Send the transaction
        let get_chunks_call_data = client
            .get_chunks_call(TEST_SONG_HEX_ID, 0, 10, client.address())?
            .calldata()
            .unwrap();
        stream.write(&get_chunks_call_data).await?;
        stream.flush().await?;

        // Read the response
        buf.clear();
        stream.read(&mut buf).await?;
        let mut cursor = Cursor::new(&buf);

        // Decode all information
        let from = cursor.read_i32().await?;
        let amount = cursor.read_i32().await?;
        let mut song_bytes = Vec::with_capacity(amount as usize * BYTES_PER_CHUNK as usize);
        cursor.read_exact(&mut song_bytes).await?;

        // And print it
        println!("from: {from}, amount: {amount}, bytes: {song_bytes:?}");
    }
    Ok(())
}
