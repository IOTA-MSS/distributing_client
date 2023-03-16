use crate::{
    library::{
        client::TangleTunesClient,
        tcp::{RequestChunksEncoder, SendChunksDecoder},
        util::SongId,
    },
    BYTES_PER_CHUNK_USIZE,
};
use ethers::types::Address;
use ethers_providers::StreamExt;
use futures::SinkExt;
use tokio::net::{tcp::ReadHalf, TcpStream};
use tokio_util::codec::{FramedRead, FramedWrite};

const CHUNKS_PER_REQUEST: usize = 20;
const CONCURRENT_REQUESTS: usize = 3;

impl TangleTunesClient {
    pub async fn download_from_distributor(
        &'static self,
        ip_address: String,
        song_id: SongId,
        first_chunk_id: usize,
        chunk_amount: usize,
        distributor_address: Address,
    ) -> eyre::Result<Vec<u8>> {
        let last_chunk_id = first_chunk_id + chunk_amount;

        let mut stream = TcpStream::connect(ip_address).await?;
        let (read_stream, write_stream) = stream.split();
        let mut read_stream = FramedRead::new(read_stream, SendChunksDecoder::new());
        let mut write_stream = FramedWrite::new(write_stream, RequestChunksEncoder);

        let mut request_queue = RequestQueue::new(first_chunk_id, last_chunk_id);
        let mut song = Vec::with_capacity(chunk_amount);

        // While our song has not yet been completely downloaded..
        while !song_is_complete(&song, chunk_amount) {
            // .. send requests if necessary
            while let Some((request_id, request_size)) = request_queue.request_now(&song) {
                println!("Requesting {request_size} chunks starting at id {request_id}");

                let tx_rlp = self
                    .get_chunks_signed_rlp(
                        song_id.clone(),
                        request_id,
                        request_size,
                        distributor_address,
                    )
                    .await?;
                write_stream.send(&tx_rlp.0).await?;
                break;
            }

            // And then read the next response
            read_chunks_from_stream(&mut read_stream, &mut song).await?
        }
        Ok(song)
    }
}

fn song_is_complete(song: &[u8], chunks: usize) -> bool {
    song.len() > (chunks * BYTES_PER_CHUNK_USIZE) - BYTES_PER_CHUNK_USIZE
}

async fn read_chunks_from_stream(
    read_stream: &mut FramedRead<ReadHalf<'_>, SendChunksDecoder>,
    song: &mut Vec<u8>,
) -> eyre::Result<()> {
    let result = read_stream.next().await.ok_or(eyre!(
        "Distributor closed stream before all data was received"
    ))?;
    let (start_chunk_id, chunks) = result?;
    println!(
        "Received {} bytes starting at id {start_chunk_id}",
        chunks.len()
    );
    for (chunk, chunk_id) in chunks.chunks(BYTES_PER_CHUNK_USIZE).zip(start_chunk_id..) {
        println!(" -- Decoded chunk {chunk_id}");
        assert_eq!(
            chunk_id as usize,
            (song.len() + start_chunk_id as usize) / BYTES_PER_CHUNK_USIZE
        );
        song.extend(chunk);
    }
    Ok(())
}

//------------------------------------------------------------------------------------------------
//  RequestQueue
//------------------------------------------------------------------------------------------------

/// A queue of requests for a (part of a) song.
struct RequestQueue(Vec<(usize, usize)>);

impl RequestQueue {
    /// Create a new request-queue that requests chunks from start-end
    pub fn new(first_chunk_id: usize, last_chunk_id: usize) -> Self {
        let inner = (first_chunk_id..last_chunk_id)
            .filter(|chunk_id| chunk_id % CHUNKS_PER_REQUEST == 0 || *chunk_id == last_chunk_id)
            .map(|chunk_id| {
                (
                    chunk_id,
                    Ord::min(CHUNKS_PER_REQUEST, last_chunk_id - chunk_id),
                )
            })
            .rev()
            .collect::<Vec<_>>();
        Self(inner)
    }

    /// Whether a new request should be made now.
    ///
    /// Returns (chunk_id, amount_of_chunks).
    pub fn request_now(&mut self, song: &[u8]) -> Option<(usize, usize)> {
        if let Some((request_id, _)) = self.0.last() {
            if *request_id <= (song.len() * BYTES_PER_CHUNK_USIZE) + CONCURRENT_REQUESTS {
                return Some(self.0.pop().unwrap());
            }
        };
        None
    }
}
