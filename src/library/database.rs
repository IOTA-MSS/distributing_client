use std::path::Path;


use color_eyre::Report;
// use deadpool_sqlite::{Manager, Pool, PoolConfig, Runtime};
use futures::executor::block_on;
use once_cell::sync::OnceCell;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Pool, Sqlite};
static CHUNK_BYTES: u32 = 32_766;

#[cfg(test)]
mod test {
    
    // use rusqlite::Connection;

    // #[test]
    // fn chunking_is_correct() {
    //     let folder = MusicFolder::init("", "mp3/").unwrap();

    //     let chunks = folder.read_chunks("sample_2mb", 0, 50).unwrap();
    //     assert_eq!(chunks.len(), 50 * CHUNK_BYTES);

    //     let _ = folder.read_chunks("sample_2mb", 0, 80).unwrap();
    //     assert_eq!(chunks.len(), 80 * CHUNK_BYTES);

    //     let chunks = folder.read_chunks("sample_2mb", 10, 20).unwrap();
    //     assert_eq!(chunks.len(), 20 * CHUNK_BYTES);

    //     let _ = folder.read_chunks("sample_2mb", 30, 50).unwrap();
    //     assert_eq!(chunks.len(), 50 * CHUNK_BYTES);
    // }

    // #[tokio::test]
    // async fn sqlite_test() -> eyre::Result<()> {
    //     let connection = Connection::open("database.sqlite")?;
    //     connection.execute(
    //         "
    //     CREATE TABLE IF NOT EXISTS songs (
    //         id TEXT NOT NULL UNIQUE,
    //         name TEXT NOT NULL,
    //         fee FLOAT NOT NULL,
    //         data BLOB NOT NULL
    //     );
    //     ",
    //         (),
    //     )?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn insert_song_test() -> eyre::Result<()> {
    //     let db = Database::open("database.sqlite").await?;
    //     let data = std::fs::read("sample_2mb.mp3").unwrap();
    //     db.add_song("song_id3", Some("song_name"), 10.8, &data)?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn delete_song_test() -> eyre::Result<()> {
    //     let db = Database::open("database.sqlite").await?;
    //     db.remove_song("song_id")?;
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn read_data() -> eyre::Result<()> {
    //     let db = Database::open("database.sqlite").await?;
    //     let _ = dbg!(db.read_chunks("song_id3", 0, 1));
    //     Ok(())
    // }
}

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

#[derive(Debug, Clone, Copy)]
pub struct Database {
    pool: &'static Pool<Sqlite>,
}

impl Database {
    async fn aquire(&self) -> eyre::Result<PoolConnection<Sqlite>> {
        Ok(self.pool.acquire().await?)
    }

    async fn open_db(path: impl AsRef<Path>, size: u32) -> eyre::Result<Pool<Sqlite>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(size)
            .connect_with(
                SqliteConnectOptions::new()
                    .filename(path)
                    .create_if_missing(true),
            )
            .await?;

        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS songs (
                id TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                data BLOB NOT NULL
            );
    
            CREATE TABLE IF NOT EXISTS key (
                key TEXT,
                encrypted BOOL
            );
            ",
        )
        .execute(&mut pool.acquire().await?)
        .await?;

        Ok(pool)
    }

    /// This should not be used in production since this leaks resources.
    #[cfg(test)]
    async fn initialize_for_test(path: &str) -> eyre::Result<Self> {
        let pool = Box::leak(Box::new(Self::open_db(path, 1).await?));
        let db = Database { pool };
        Ok(db)
    }

    pub async fn initialize(path: impl AsRef<Path>) -> eyre::Result<Self> {
        let pool = POOL.get_or_try_init(|| {
            block_on(async move {
                let pool = Self::open_db(path, 10).await?;
                Ok::<_, Report>(pool)
            })
        })?;

        // dbg!(initialized_now);

        let db = Database { pool };
        Ok(db)
    }

    pub async fn reset(&self) -> eyre::Result<()> {
        sqlx::query(
            "
        DELETE * FROM songs;
        DELETE * FROM key;
        ",
        )
        .execute(&mut self.aquire().await?)
        .await?;

        Ok(())
    }

    pub async fn set_key(&self, key: &str, encrypted: bool) -> eyre::Result<()> {
        sqlx::query(
            "
            DELETE FROM key;
            INSERT INTO key (key, encrypted) VALUES (?1, ?2);
            ",
        )
        .bind(key)
        .bind(encrypted)
        .execute(&mut self.aquire().await?)
        .await?;

        Ok(())
    }

    pub async fn get_key(&self) -> eyre::Result<Option<(String, bool)>> {
        let row = sqlx::query_as::<_, (String, bool)>(
            "
            SELECT key, encrypted FROM key;
            ",
        )
        .fetch_optional(&mut self.aquire().await?)
        .await?;

        Ok(row)
    }

    pub async fn add_song(
        &self,
        id: &str,
        name: Option<&str>,
        song_data: &[u8],
    ) -> eyre::Result<()> {
        let name = name.unwrap_or("UNNAMED");
        sqlx::query(
            "
            INSERT INTO songs (id, name, data) VALUES (?1, ?2, ?4);
            ",
        )
        .bind(id)
        .bind(name)
        .bind(song_data)
        .execute(&mut self.aquire().await?)
        .await?;

        Ok(())
    }

    pub async fn remove_song(&self, id: &str) -> eyre::Result<bool> {
        let res = sqlx::query(
            "
            DELETE FROM songs WHERE id = ?1;
            ",
        )
        .bind(id)
        .execute(&mut self.aquire().await?)
        .await?;

        if res.rows_affected() == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn read_chunks(
        &self,
        id: &str,
        start_at: u32,
        chunks: u32,
    ) -> eyre::Result<(Vec<u8>, f32)> {
        let byte_offset = (start_at * CHUNK_BYTES) + 1; // First char/byte has i=1 in sqlite
        let byte_len = chunks * CHUNK_BYTES;

        let row = sqlx::query_as::<_, (Vec<u8>, f32)>(
            "
            SELECT substr(data, ?1, ?2), fee
            FROM songs WHERE id = ?3
            ",
        )
        .bind(byte_offset)
        .bind(byte_len)
        .bind(id)
        .fetch_one(&mut self.aquire().await?)
        .await?;

        Ok(row)
    }
}
