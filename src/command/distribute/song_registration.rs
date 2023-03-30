use crate::library::{
    app::AppData,
    util::{SongId, TransactionReceiptExt},
};
use ethers::types::U256;
static REGISTER_ATTEMPTS: usize = 3;

/// Registers for distribution of all songs in the database.
/// If an error occurs, all songs are automatically deregistered.
pub async fn register_for_songs_in_database(app: &'static AppData) -> eyre::Result<()> {
    println!("Registering for all songs in database..");

    let song_ids = app.database.get_all_downloaded_song_ids().await?;

    // Send all transactions until complete or an error is encountered
    for chunk in song_ids.chunks(5) {
        println!("Registering for distribution of songs {song_ids:?}..",);

        let songs = chunk.iter().map(|id| (*id, app.fee)).collect();

        let mut i = 0;
        let result = loop {
            match register(app, &songs).await {
                Ok(()) => break Ok(()),
                Err(e) => {
                    if i == REGISTER_ATTEMPTS {
                        break Err(e);
                    }
                }
            }
            i += 1;
        };

        match result {
            Ok(_) => println!("Registered succesfully for songs"),
            Err(e) => println!("Couldn't register for songs: {e}"),
        }
    }

    println!("All songs in database have been registered!\n");
    Ok(())
}

pub async fn register(app: &'static AppData, songs: &Vec<(SongId, U256)>) -> eyre::Result<()> {
    app.client
        .distribute_call(songs.clone())
        .await?
        .send()
        .await?
        .await?
        .status_is_ok(&format!("Could not register song with ids {songs:?}"))?;

    Ok(())
}

/// Deregister for distribution of the given songs.
pub async fn deregister_for_songs_in_database(app: &'static AppData) -> eyre::Result<()> {
    println!("Registering for all songs in database..");

    // Send all transactions until complete or an error is encountered
    for song_ids in app.database.get_all_downloaded_song_ids().await?.chunks(5) {
        println!("Registering for distribution of songs {song_ids:?}..",);

        let songs = song_ids.iter().map(Clone::clone).collect();

        let mut i = 0;
        let result = loop {
            match deregister(app, &songs).await {
                Ok(()) => break Ok(()),
                Err(e) => {
                    if i == REGISTER_ATTEMPTS {
                        break Err(e);
                    }
                }
            }
            i += 1;
        };

        match result {
            Ok(_) => println!("Registered succesfully for songs"),
            Err(e) => println!("Couldn't register for songs: {e}"),
        }
    }

    println!("All songs in database have been registered!\n");
    Ok(())
}

pub async fn deregister(app: &'static AppData, songs: &Vec<SongId>) -> eyre::Result<()> {
    println!("Deregistering songs {songs:?} on the smart-contract..");

    app.client
        .undistribute_call(songs.clone())
        .await?
        .send()
        .await?
        .await?
        .status_is_ok(&format!("Could not deregister song with ids {songs:?}"))?;

    Ok(())
}
