use crate::{
    library::{
        app::App,
        protocol::{RequestChunksEncoder, SendChunksDecoder},
    },
    BYTES_PER_CHUNK_USIZE,
};
use ethers_providers::StreamExt;
use futures::SinkExt;
use std::{fs::OpenOptions, io::Write};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

const CHUNKS_PER_REQUEST: usize = 5;

pub async fn run(
    cfg: App,
    port: u16,
    song_id: String,
    chunk_index: usize,
    chunk_amount: usize,
    file: String,
) -> eyre::Result<()> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).await?;
    let (read_stream, write_stream) = stream.split();
    let mut read_stream = FramedRead::new(read_stream, SendChunksDecoder::new());
    let mut write_stream = FramedWrite::new(write_stream, RequestChunksEncoder);

    let database = cfg.database()?;
    let client = cfg.initialize_client(&database).await?;
    let distributor_address = client.wallet_address();

    let mut song_buffer = vec![None; chunk_amount];

    for this_chunk_index in (chunk_index..chunk_index + chunk_amount)
        .into_iter()
        .step_by(CHUNKS_PER_REQUEST)
    {
        let request_size = Ord::min(CHUNKS_PER_REQUEST, chunk_amount - this_chunk_index);

        let tx_rlp = client
            .get_chunks_rlp(
                &song_id,
                this_chunk_index,
                request_size,
                distributor_address,
            )
            .await?;
        dbg!(tx_rlp.len());
        write_stream.send(&tx_rlp.0).await?;

        let mut chunks_received = 0;
        while chunks_received < request_size {
            let (start_chunk_id, chunks) = read_stream.next().await.unwrap()?;
            dbg!((start_chunk_id, chunks.len()));

            for (chunk, chunk_id) in chunks.chunks(BYTES_PER_CHUNK_USIZE).zip(start_chunk_id..) {
                chunks_received += 1;
                song_buffer[chunk_id as usize] = Some(chunk.to_vec());
            }
        }
    }

    let mut file = OpenOptions::new().write(true).create(true).open(file)?;
    song_buffer.iter().for_each(|chunk| {
        file.write_all(chunk.as_ref().unwrap()).unwrap();
    });
    file.flush()?;

    Ok(())
}
