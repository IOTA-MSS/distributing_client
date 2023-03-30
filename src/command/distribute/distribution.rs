use crate::library::{app::AppData, util::SongId};
use itertools::Itertools;

/// Distributes all songs in the database, only if we are not yet distributing them.
/// It will try every distribution `attempts` times, and the chunk-size is `size`.
///
/// If an error occurs, make sure to call [`undistribute_songs_in_database`] after!
pub async fn distribute_songs_in_database(
    app: &'static AppData,
    attempts: usize,
    size: usize,
) -> eyre::Result<()> {
    println!("Registering for all songs in database..");

    // Get all songs in the database
    let song_ids = app.database.get_all_downloaded_song_ids().await?;

    // And iterate through them by REGISTRATION_SIZE
    for chunk in song_ids.chunks(size) {
        println!("Registering for distribution of songs {song_ids:?}..",);

        let songs = chunk.iter().map(|id| (*id, app.fee)).collect_vec();

        // We try REGISTER_ATTEMPTS amount of times to distribute the song.
        // It might happen that someone else attempts to distribute the song while we are,
        // and that can mess up the transaction.
        let mut i = 0;
        loop {
            match app.client.try_distribute(&songs).await {
                Ok(()) => break Ok(()),
                Err(e) => {
                    if i == attempts {
                        break Err(e);
                    }
                }
            }
            i += 1;
        }?;
    }

    println!("Distributing all songs in the database!\n");
    Ok(())
}

/// Undistributes all songs in the database, only if we are not distributing them.
/// It will try every undistribution `attempts` times, and the chunk-size is `size`.
///
/// If an error occurs it may be true that there are songs which are not undistributed!.
pub async fn undistribute_songs_in_database(
    app: &'static AppData,
    attempts: usize,
    size: usize,
) -> eyre::Result<()> {
    println!("Undistributing for all songs in database..");

    let song_ids = app.database.get_all_downloaded_song_ids().await?;

    let mut errors = Vec::new();
    // Send all transactions until complete or an error is encountered
    for chunk in song_ids.chunks(size) {
        println!("Undistributing songs {song_ids:?}..",);

        let songs: Vec<SongId> = chunk.iter().map(Clone::clone).collect();

        let mut i = 0;
        let result = loop {
            match app.client.try_undistribute(&songs).await {
                Ok(()) => break Ok(()),
                Err(e) => {
                    if i == attempts {
                        break Err(e);
                    }
                }
            }
            i += 1;
        };

        if let Err(e) = result {
            errors.push(e)
        }
    }

    if errors.is_empty() {
        println!("All songs in database have been undistributed!\n");
        Ok(())
    } else {
        println!("Could not undistribute all songs");
        Err(errors
            .into_iter()
            .fold(eyre!("Could not undistribute all songs"), |init, e| {
                init.wrap_err(e)
            }))
    }
}
