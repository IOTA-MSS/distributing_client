use crate::state::{state, DistributionState};
use eyre::eyre;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

static CHUNK_BYTES: usize = 32_766;

pub fn chunks_from_title(
    name: impl AsRef<Path>,
    start_at: usize,
    chunks: usize,
) -> eyre::Result<Vec<u8>> {
    let file = File::open(state().music_folder.join(name))?;
    chunks_from_file(file, start_at, chunks)
}

fn chunks_from_file(file: File, start_at: usize, chunks: usize) -> eyre::Result<Vec<u8>> {
    let bytes = BufReader::new(file)
        .bytes()
        .skip(start_at * CHUNK_BYTES)
        .take(chunks * CHUNK_BYTES)
        .collect::<Result<Vec<_>, _>>()?;

    if bytes.len() != chunks * CHUNK_BYTES {
        Err(eyre!("Requested bytes were not available"))
    } else {
        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn chunking_is_correct() {
        let file = File::open("mp3/sample_2mb.mp3").unwrap();
        let chunks = chunks_from_file(file, 0, 30).unwrap();
        assert_eq!(chunks.len(), 30 * CHUNK_BYTES);

        let file = File::open("mp3/sample_2mb.mp3").unwrap();
        let _ = chunks_from_file(file, 0, 80).unwrap_err();

        let file = File::open("mp3/sample_2mb.mp3").unwrap();
        let chunks = chunks_from_file(file, 10, 20).unwrap();
        assert_eq!(chunks.len(), 20 * CHUNK_BYTES);

        let file = File::open("mp3/sample_2mb.mp3").unwrap();
        let _ = chunks_from_file(file, 30, 50).unwrap_err();
    }
}
