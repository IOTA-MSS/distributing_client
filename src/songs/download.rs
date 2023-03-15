use crate::{
    library::util::SongId,
    library::{
        app::AppData,
        tcp::{RequestChunksEncoder, SendChunksDecoder},
    },
    BYTES_PER_CHUNK_USIZE,
};
use ethers_providers::StreamExt;
use futures::SinkExt;
use std::{fs::OpenOptions, io::Write};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

const CHUNKS_PER_REQUEST: usize = 5;
const REQUESTS_CONCURRENT: usize = 5;

pub async fn run_download(
    app: &'static AppData,
    ip_address: String,
    song_id: String,
    to_file: Option<(String, usize, usize)>,
) -> eyre::Result<()> {
    let Some((file, chunk_index, chunks_to_receive)) = to_file else {
        bail!("Downloading to database not yet implemented. Specify `--to_file <FILE>`")
    };

    let song_id = SongId::try_from_hex(&song_id)?;
    let mut stream = TcpStream::connect(ip_address).await?;
    let (read_stream, write_stream) = stream.split();
    let mut read_stream = FramedRead::new(read_stream, SendChunksDecoder::new());
    let mut write_stream = FramedWrite::new(write_stream, RequestChunksEncoder);
    let distributor_address = app.client.wallet_address();

    let mut song_buffer = Vec::<u8>::with_capacity(chunks_to_receive);
    let mut next_chunk_request_id = chunk_index;

    while song_buffer.len() < chunks_to_receive {
        if song_buffer.len() < next_chunk_request_id {

        }
        let request_size = Ord::min(
            CHUNKS_PER_REQUEST,
            chunks_to_receive - next_chunk_request_id,
        );
        let tx_rlp = app
            .client
            .get_chunks_rlp(
                song_id.clone(),
                next_chunk_request_id,
                request_size,
                distributor_address,
            )
            .await?;
        write_stream.send(&tx_rlp.0).await?;
        next_chunk_request_id += CHUNKS_PER_REQUEST;

        let result = read_stream.next().await.ok_or(eyre!(
            "Distributor closed stream before all data was received"
        ))?;
        let (start_chunk_id, chunks) = result?;

        for (chunk, chunk_id) in chunks.chunks(BYTES_PER_CHUNK_USIZE).zip(start_chunk_id..) {
            assert_eq!(chunk_id as usize, song_buffer.len());
            next_chunk_request_id += 1;
            song_buffer.extend(chunk);
        }
    }

    let mut file = OpenOptions::new().write(true).create(true).open(file)?;
    file.write_all(&song_buffer)?;
    file.flush()?;

    Ok(())
}
