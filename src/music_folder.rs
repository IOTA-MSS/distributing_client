use eyre::{eyre, Context};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

static CHUNK_BYTES: usize = 32_766;

#[derive(Debug)]
pub struct MusicFolder {
    folder: PathBuf,
}

impl MusicFolder {
    pub fn new(folder: &str) -> Self {
        Self {
            folder: Path::new(folder).into(),
        }
    }

    fn get_path(&self, id: &str) -> PathBuf {
        self.folder.join(Path::new(id).with_extension("mp3"))
    }

    fn open_file(&self, id: &str) -> eyre::Result<File> {
        Ok(File::open(self.get_path(id))?)
    }

    pub fn read_chunks(&self, id: &str, start_at: usize, chunks: usize) -> eyre::Result<Vec<u8>> {
        let bytes = BufReader::new(self.open_file(id)?)
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

    pub fn remove_song(&self, id: &str) -> eyre::Result<()> {
        let path = self.get_path(id);
        std::fs::remove_file(&path).wrap_err(format!("Path: {:?}", path))
    }

    pub fn add_song(&self, id: &str, bytes: &[u8]) -> eyre::Result<()> {
        let path = self.get_path(id);

        if path.exists() {
            Err(eyre!("File or folder {:?} already exists!", path))?;
        }

        let mut file = File::create(path)?;
        file.write_all(bytes)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chunking_is_correct() {
        let folder = MusicFolder::new("mp3/");

        let chunks = folder.read_chunks("sample_2mb", 0, 50).unwrap();
        assert_eq!(chunks.len(), 50 * CHUNK_BYTES);

        let _ = folder.read_chunks("sample_2mb", 0, 80).unwrap_err();

        let chunks = folder.read_chunks("sample_2mb", 10, 20).unwrap();
        assert_eq!(chunks.len(), 20 * CHUNK_BYTES);

        let _ = folder.read_chunks("sample_2mb", 30, 50).unwrap_err();
    }
}
