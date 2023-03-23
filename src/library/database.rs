use crate::arguments::Arguments;
use crate::library::util::SongId;
use crate::BYTES_PER_CHUNK;
use chrono::{DateTime, Local, Utc};
use ethers::utils::serialize;
use futures::executor::block_on;
use once_cell::sync::OnceCell;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
use std::fmt::Debug;
use std::path::Path;
use std::time::Duration;

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
        database.migrate_db().await?;
        Ok(database)
    }

    pub async fn initialize_in_memory() -> eyre::Result<Self> {
        let pool = Box::leak(Box::new(Self::new_pool(":memory:", 1).await?));
        let database = Self { pool };
        database.migrate_db().await?;
        Ok(database)
    }

    /// Acquire a connection to the database-pool.
    async fn acquire(&self) -> eyre::Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }

    /// Creates all tables if they do not yet exist
    pub async fn migrate_db(&self) -> eyre::Result<()> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS songs (
                id BLOB PRIMARY KEY,
                data BLOB NOT NULL,
                inserted_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
            );
    
            CREATE TABLE IF NOT EXISTS song_list (
                idx INT PRIMARY KEY,
                id BLOB NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS key (
                key TEXT PRIMARY KEY,
                encrypted BOOL
            );
            ",
        )
        .execute(&mut self.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn get_song_index(&self) -> eyre::Result<Vec<(usize, SongId)>> {
        Ok(sqlx::query_as::<_, (u32, Vec<u8>)>(
            "
            SELECT idx, id FROM song_list;
            ",
        )
        .fetch_all(&mut self.acquire().await?)
        .await?
        .into_iter()
        .map(|(i, id)| (i as usize, id.try_into().unwrap()))
        .collect())
    }

    pub async fn add_to_song_index(&self, ids: &[(usize, SongId)]) -> eyre::Result<()> {
        let mut conn = self.acquire().await?;
        for (index, id) in ids {
            if *index > 0 {
                // Check that previous one exists
                let prev_index = sqlx::query_as::<_, (u32,)>(
                    "
                    SELECT (idx) FROM song_list WHERE idx = ?1;
                    ",
                )
                .bind((index - 1) as u32)
                .fetch_optional(&mut conn)
                .await?;
                if prev_index.is_none() {
                    panic!("Cannot add song-index if prev one doesn't exist")
                }
            }

            sqlx::query(
                "
                INSERT INTO song_list (idx, id) VALUES (?1, ?2);
                ",
            )
            .bind(*index as i64)
            .bind(id.as_ref())
            .execute(&mut conn)
            .await?;
        }
        Ok(())
    }

    /// Get the last song-index stored in the databases
    pub async fn get_next_song_index(&self) -> eyre::Result<usize> {
        let val = sqlx::query_as::<_, (u32,)>(
            "
            SELECT count(*) FROM song_list;
            ",
        )
        .fetch_one(&mut self.acquire().await?)
        .await?
        .0;

        Ok(val as usize)
    }

    pub async fn get_song_id_by_index(&self, index: usize) -> eyre::Result<Option<SongId>> {
        let row = sqlx::query_as::<_, (Vec<u8>,)>(
            "
            SELECT id FROM song_list WHERE idx = ?1;
            ",
        )
        .bind(index as u32)
        .fetch_optional(&mut self.acquire().await?)
        .await?;

        Ok(row.map(|id| id.0.try_into().unwrap()))
    }

    pub async fn clear_song_index(&self) -> eyre::Result<()> {
        sqlx::query(
            "
            DELETE FROM song_list;
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

    pub async fn get_new_songs(&self, since: &DateTime<Utc>) -> eyre::Result<Vec<SongId>> {
        let date_time = since.format("%Y-%m-%d %H:%M:%S").to_string();

        let row = sqlx::query_as::<_, (Vec<u8>,)>(
            "
            SELECT id FROM songs
            WHERE inserted_at >= ?1;
            ",
        )
        .bind(date_time)
        .fetch_all(&mut self.acquire().await?)
        .await?
        .into_iter()
        .map(|e| SongId::try_from(e.0).unwrap())
        .collect();

        Ok(row)
    }

    /// Add a song to the database
    pub async fn add_song(&self, id: &SongId, song_data: &[u8]) -> eyre::Result<()> {
        sqlx::query(
            "
            INSERT INTO songs (id, data) VALUES (?1, ?2);
            ",
        )
        .bind(id.as_slice())
        .bind(song_data)
        .execute(&mut self.acquire().await?)
        .await?;

        Ok(())
    }

    pub async fn get_all_downloaded_song_ids(&self) -> eyre::Result<Vec<SongId>> {
        Ok(sqlx::query_as::<_, (Vec<u8>,)>(
            "
            SELECT id FROM songs;
            ",
        )
        .fetch_all(&mut self.acquire().await?)
        .await?
        .into_iter()
        .map(|(id,)| id.try_into().unwrap())
        .collect())
    }

    pub async fn get_index_by_song_id(&self, song_id: &SongId) -> eyre::Result<Option<u32>> {
        Ok(sqlx::query_as::<_, (u32,)>(
            "
            SELECT idx FROM song_list WHERE id = ?1;
            ",
        )
        .bind(song_id.as_ref())
        .fetch_optional(&mut self.acquire().await?)
        .await?
        .map(|(id,)| id))
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
    ) -> eyre::Result<Vec<u8>> {
        let byte_start = (chunk_start * BYTES_PER_CHUNK) + 1; // First char/byte has i=1 in sqlite
        let bytes = chunks * BYTES_PER_CHUNK;

        let row = sqlx::query_as::<_, (Vec<u8>,)>(
            "
            SELECT substr(data, ?1, ?2) FROM songs WHERE id = ?3
            ",
        )
        .bind(byte_start)
        .bind(bytes)
        .bind(id.as_slice())
        .fetch_one(&mut self.acquire().await?)
        .await?;

        Ok(row.0)
    }

    pub async fn remove_private_key(&self) -> eyre::Result<()> {
        sqlx::query(
            "
            DELETE FROM key;
            ",
        )
        .execute(&mut self.acquire().await?)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{test, BYTES_PER_CHUNK_USIZE};

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

        let chunks = db.get_chunks(&unvalidated_song_id, 0, 50).await?;
        assert_eq!(chunks, song_data[0..50 * BYTES_PER_CHUNK_USIZE]);

        let chunks = db.get_chunks(&unvalidated_song_id, 50, 100).await?;
        assert_eq!(chunks, song_data[50 * BYTES_PER_CHUNK_USIZE..]);

        let chunks = db.get_chunks(&unvalidated_song_id, 10, 20).await?;
        assert_eq!(
            chunks,
            song_data[10 * BYTES_PER_CHUNK_USIZE..30 * BYTES_PER_CHUNK_USIZE]
        );

        let chunks = db.get_chunks(&unvalidated_song_id, 30, 50).await?;
        assert_eq!(
            chunks[0..20 * BYTES_PER_CHUNK_USIZE],
            song_data[30 * BYTES_PER_CHUNK_USIZE..50 * BYTES_PER_CHUNK_USIZE]
        );
        assert!(chunks.len() < 50 * BYTES_PER_CHUNK_USIZE);

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
        let db_data = db.get_chunks(&unvalidated_song_id, 0, 100).await?;
        assert_eq!(song_data, db_data);

        assert_eq!(db.remove_song(&unvalidated_song_id).await?, true);
        assert!(db.get_chunks(&unvalidated_song_id, 0, 100).await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn song_index() -> eyre::Result<()> {
        let unvalidated_song_id = SongId::try_from_hex(test::UNVALIDATED_SONG_HEX_ID).unwrap();
        let validated_song_id = SongId::try_from_hex(test::VALIDATED_SONG_HEX_ID).unwrap();
        let db = Database::initialize_in_memory().await?;

        assert_eq!(db.get_next_song_index().await?, 0);
        db.add_to_song_index(&[(0, unvalidated_song_id)]).await?;
        assert_eq!(db.get_next_song_index().await?, 1);
        db.add_to_song_index(&[(1, validated_song_id)]).await?;
        assert_eq!(db.get_next_song_index().await?, 2);

        Ok(())
    }

    #[tokio::test]
    async fn song_metadata() -> eyre::Result<()> {
        let unvalidated_song_id = SongId::try_from_hex(test::UNVALIDATED_SONG_HEX_ID).unwrap();
        let validated_song_id = SongId::try_from_hex(test::VALIDATED_SONG_HEX_ID).unwrap();
        let db = Database::initialize_in_memory().await?;

        assert_eq!(db.get_all_downloaded_song_ids().await?.len(), 0);

        let song_data = std::fs::read(
            "mp3/0x486df48c7468457fc8fbbdc0cd1ce036b2b21e2f093559be3c37fcb024c1facf.mp3",
        )?;
        db.add_song(&unvalidated_song_id, &song_data).await?;

        assert_eq!(db.get_all_downloaded_song_ids().await?.len(), 1);

        let song_data = std::fs::read(
            "mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3",
        )?;
        db.add_song(&validated_song_id, &song_data).await?;

        assert_eq!(db.get_all_downloaded_song_ids().await?.len(), 2);

        assert_eq!(db.remove_song(&unvalidated_song_id).await?, true);

        assert_eq!(db.get_all_downloaded_song_ids().await?.len(), 1);

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
}
