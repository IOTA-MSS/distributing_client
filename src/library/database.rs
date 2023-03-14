use crate::library::util::SongId;
use crate::BYTES_PER_CHUNK;
use futures::executor::block_on;
use once_cell::sync::OnceCell;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::path::Path;

static DATABASE_POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();
#[derive(Debug, Clone, Copy)]
pub struct Database {
    pool: &'static Pool<Sqlite>,
}

impl Database {
    async fn new_pool(path: impl AsRef<Path>, size: u32) -> eyre::Result<Pool<Sqlite>> {
        Ok(SqlitePoolOptions::new()
            .max_connections(size)
            .connect_with(
                SqliteConnectOptions::new()
                    .filename(path)
                    .create_if_missing(true),
            )
            .await?)
    }

    /// Initializes the database.
    pub async fn initialize(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let database = Self {
            pool: DATABASE_POOL.get_or_try_init(|| block_on(Self::new_pool(path, 5)))?,
        };
        database.create_tables().await?;
        Ok(database)
    }

    pub async fn initialize_in_memory() -> eyre::Result<Self> {
        let pool = Box::leak(Box::new(Self::new_pool(":memory:", 1).await?));
        let database = Self { pool };
        database.create_tables().await?;
        Ok(database)
    }

    /// Acquire a connection to the database-pool.
    async fn acquire(&self) -> eyre::Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }

    /// Creates all tables if they do not yet exist
    pub async fn create_tables(&self) -> eyre::Result<()> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS songs (
                id BLOB NOT NULL UNIQUE,
                distributing BOOL NOT NULL,
                data BLOB NOT NULL
            );
    
            CREATE TABLE IF NOT EXISTS key (
                key TEXT,
                encrypted BOOL
            );
            ",
        )
        .execute(&mut self.acquire().await?)
        .await?;

        Ok(())
    }

    /// Sets the private key in the database.
    /// Stores whether the key is encrypted.
    pub async fn set_key(&self, key: &str, encrypted: bool) -> eyre::Result<()> {
        sqlx::query(
            "
            DELETE FROM key;
            INSERT INTO key (key, encrypted) VALUES (?1, ?2);
            ",
        )
        .bind(key)
        .bind(encrypted)
        .execute(&mut self.acquire().await?)
        .await?;

        Ok(())
    }

    /// Get the private key from the database and whether it is encrypted.
    pub async fn get_key(&self) -> eyre::Result<Option<(String, bool)>> {
        let row = sqlx::query_as::<_, (String, bool)>(
            "
            SELECT key, encrypted FROM key;
            ",
        )
        .fetch_optional(&mut self.acquire().await?)
        .await?;

        Ok(row)
    }

    pub async fn set_distribution(&self, song_id: &SongId, distributing: bool) -> eyre::Result<()> {
        sqlx::query(
            "
            UPDATE songs SET distributing = ?1 WHERE id = $2;
            ",
        )
        .bind(distributing)
        .bind(song_id.as_slice())
        .execute(&mut self.acquire().await?)
        .await?;

        Ok(())
    }

    /// Add a song to the database
    pub async fn add_song(&self, id: &SongId, song_data: &[u8]) -> eyre::Result<()> {
        println!("Inserting song: {id:?}");
        sqlx::query(
            "
            INSERT INTO songs (id, distributing, data) VALUES (?1, ?2, ?3);
            ",
        )
        .bind(id.as_slice())
        .bind(false)
        .bind(song_data)
        .execute(&mut self.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn get_songs_info(&self) -> eyre::Result<Vec<(SongId, bool)>> {
        let row = sqlx::query_as::<_, (Vec<u8>, bool)>(
            "
            SELECT id, distributing FROM songs
            ",
        )
        .fetch_all(&mut self.acquire().await?)
        .await?
        .into_iter()
        .map(|(id, bool)| {
            println!("Got song: {id:?}");
            (id.try_into().unwrap(), bool)
        })
        .collect();

        Ok(row)
    }

    pub async fn remove_song(&self, id: &SongId) -> eyre::Result<bool> {
        let res = sqlx::query(
            "
            DELETE FROM songs WHERE id = ?1;
            ",
        )
        .bind(id.as_slice())
        .execute(&mut self.acquire().await?)
        .await?;

        if res.rows_affected() == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get the chunks from (chunk_start, chunk_start + chunks) if they exist.
    pub async fn get_chunks(
        &self,
        id: &SongId,
        chunk_start: u32,
        chunks: u32,
    ) -> eyre::Result<(Vec<u8>, bool)> {
        let byte_start = (chunk_start * BYTES_PER_CHUNK) + 1; // First char/byte has i=1 in sqlite
        let bytes = chunks * BYTES_PER_CHUNK;

        let row = sqlx::query_as::<_, (Vec<u8>, bool)>(
            "
            SELECT substr(data, ?1, ?2), distributing FROM songs WHERE id = ?3
            ",
        )
        .bind(byte_start)
        .bind(bytes)
        .bind(id.as_slice())
        .fetch_one(&mut self.acquire().await?)
        .await?;

        Ok(row)
    }
}

#[cfg(test)]
mod test {
    use crate::test;

    use super::*;

    impl Database {
        async fn drop_tables(&self) -> eyre::Result<()> {
            sqlx::query(
                "
        DROP TABLE songs;
        DROP TABLE key;
        ",
            )
            .execute(&mut self.acquire().await?)
            .await?;

            Ok(())
        }
    }

    #[tokio::test]
    async fn chunking_is_correct() -> eyre::Result<()> {
        let unvalidated_song_id = SongId::try_from_hex(test::UNVALIDATED_SONG_HEX_ID).unwrap();

        let db = Database::initialize_in_memory().await?;
        let song_data = std::fs::read(
            "mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3",
        )?;
        db.add_song(&unvalidated_song_id, &song_data).await?;

        let (chunks, _) = db.get_chunks(&unvalidated_song_id, 0, 50).await?;
        assert_eq!(chunks, song_data[0..50 * BYTES_PER_CHUNK as usize]);

        let (chunks, _) = db.get_chunks(&unvalidated_song_id, 0, 80).await?;
        assert_eq!(chunks.len(), 2113939);
        assert!(chunks.len() < 80 * BYTES_PER_CHUNK as usize);

        let (chunks, _) = db.get_chunks(&unvalidated_song_id, 10, 20).await?;
        assert_eq!(
            chunks,
            song_data[10 * BYTES_PER_CHUNK as usize..30 * BYTES_PER_CHUNK as usize]
        );

        let (chunks, _) = db.get_chunks(&unvalidated_song_id, 30, 50).await?;
        assert_eq!(
            chunks[0..20 * BYTES_PER_CHUNK as usize],
            song_data[30 * BYTES_PER_CHUNK as usize..50 * BYTES_PER_CHUNK as usize]
        );
        assert!(chunks.len() < 50 * BYTES_PER_CHUNK as usize);

        Ok(())
    }

    #[tokio::test]
    async fn add_remove_song() -> eyre::Result<()> {
        let unvalidated_song_id = SongId::try_from_hex(test::UNVALIDATED_SONG_HEX_ID).unwrap();
        let db = Database::initialize_in_memory().await?;

        assert_eq!(db.remove_song(&unvalidated_song_id).await?, false);

        let song_data = std::fs::read(
            "mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3",
        )?;
        db.add_song(&unvalidated_song_id, &song_data).await?;
        let (db_data, distribute) = db.get_chunks(&unvalidated_song_id, 0, 100).await?;
        assert!(!distribute);
        assert_eq!(song_data, db_data);

        assert_eq!(db.remove_song(&unvalidated_song_id).await?, true);
        assert!(db.get_chunks(&unvalidated_song_id, 0, 100).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn song_metadata() -> eyre::Result<()> {
        let unvalidated_song_id = SongId::try_from_hex(test::UNVALIDATED_SONG_HEX_ID).unwrap();
        let validated_song_id = SongId::try_from_hex(test::VALIDATED_SONG_HEX_ID).unwrap();
        let db = Database::initialize_in_memory().await?;

        assert_eq!(db.get_songs_info().await?.len(), 0);

        let song_data = std::fs::read(
            "mp3/0x8b3d8bfd0c161381ce232660cd0b2262109b27be18989870406b5d0b986e60f9.mp3",
        )?;
        db.add_song(&unvalidated_song_id, &song_data).await?;

        assert_eq!(db.get_songs_info().await?.len(), 1);

        let song_data = std::fs::read(
            "mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3",
        )?;
        db.add_song(&validated_song_id, &song_data).await?;

        assert_eq!(db.get_songs_info().await?.len(), 2);

        assert_eq!(db.remove_song(&unvalidated_song_id).await?, true);

        assert_eq!(db.get_songs_info().await?.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn set_get_key() -> eyre::Result<()> {
        let db = Database::initialize_in_memory().await?;

        assert_eq!(db.get_key().await?, None);
        db.set_key("test", false).await?;
        assert_eq!(db.get_key().await?, Some(("test".to_string(), false)));
        db.set_key("test2", false).await?;
        assert_eq!(db.get_key().await?, Some(("test2".to_string(), false)));

        Ok(())
    }

    #[tokio::test]
    async fn set_distribution() -> eyre::Result<()> {
        let unvalidated_song_id = SongId::try_from_hex(test::UNVALIDATED_SONG_HEX_ID).unwrap();
        let db = Database::initialize_in_memory().await?;
        let song_data = std::fs::read(
            "mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3",
        )?;
        db.add_song(&unvalidated_song_id, &song_data).await?;

        assert_eq!(db.get_chunks(&unvalidated_song_id, 0, 0).await?.1, false);
        db.set_distribution(&unvalidated_song_id, true).await?;
        assert_eq!(db.get_chunks(&unvalidated_song_id, 0, 0).await?.1, true);
        db.set_distribution(&unvalidated_song_id, false).await?;
        assert_eq!(db.get_chunks(&unvalidated_song_id, 0, 0).await?.1, false);

        Ok(())
    }
}
