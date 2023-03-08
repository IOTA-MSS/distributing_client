use crate::{
    library::{
        config::Config,
        protocol::{RequestChunksEncoder, SendChunksDecoder},
    },
    BYTES_PER_CHUNK_USIZE, TEST_SONG_HEX_ID,
};
use ethers_providers::StreamExt;
use futures::SinkExt;
use itertools::Itertools;
use std::iter::repeat;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

pub async fn run(cfg: Config, port: u16) -> eyre::Result<()> {
    const CHUNKS_PER_REQUEST: usize = 5;
    const SONG_SIZE_IN_CHUNKS: usize = 50;

    let mut stream = TcpStream::connect(("127.0.0.1", port)).await?;
    let (read_stream, write_stream) = stream.split();
    let mut read_stream = FramedRead::new(read_stream, SendChunksDecoder::new());
    let mut write_stream = FramedWrite::new(write_stream, RequestChunksEncoder);

    let database = cfg.initialize_database().await?;
    let client = cfg.initialize_client(&database).await?;
    let distributor_address = client.address();

    let mut song_buffer = repeat(None).take(SONG_SIZE_IN_CHUNKS).collect_vec();

    for chunk_start in (0..SONG_SIZE_IN_CHUNKS)
        .into_iter()
        .step_by(CHUNKS_PER_REQUEST)
    {
        let chunk_count = Ord::min(CHUNKS_PER_REQUEST, SONG_SIZE_IN_CHUNKS - chunk_start);

        let tx_rlp = client
            .create_get_chunks_signed_tx_rlp(
                TEST_SONG_HEX_ID,
                chunk_start,
                chunk_count,
                distributor_address,
            )
            .await?;
        dbg!(tx_rlp.len());

        write_stream.send(&tx_rlp.0).await?;

        let (start_chunk_id, chunks) = read_stream.next().await.unwrap()?;
        dbg!((start_chunk_id, chunks.len()));

        for (chunk, chunk_id) in chunks.chunks(BYTES_PER_CHUNK_USIZE).zip(start_chunk_id..) {
            song_buffer[chunk_id as usize] = Some(chunk.to_vec());
        }
    }

    let song = song_buffer.iter_mut().fold(
        Vec::with_capacity(BYTES_PER_CHUNK_USIZE * SONG_SIZE_IN_CHUNKS),
        |mut song: Vec<u8>, chunk| {
            song.append(chunk.as_mut().unwrap());
            song
        },
    );

    Ok(())
}
