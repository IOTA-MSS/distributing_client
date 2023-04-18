use crate::{
    command,
    library::{app::App, util::SongId},
};
use ethers::types::U256;
use rand::{seq::IteratorRandom, thread_rng};

pub async fn update(app: &'static App) -> eyre::Result<Vec<(usize, SongId)>> {
    let index = app.database.get_next_song_index().await?;
    let new_ids = app.client.get_song_ids_from_index(index).await?;
    println!("New songs:");
    for (i, id) in &new_ids {
        println!("{i}: {id}");
    }
    app.database.add_to_song_index(&new_ids).await?;
    Ok(new_ids)
}

pub async fn reset(app: &'static App, to_update: bool) -> eyre::Result<()> {
    app.database.clear_song_index().await?;
    println!("Song index cleared.\n");
    if to_update {
        update(app).await?;
    }
    Ok(())
}

pub async fn list(app: &'static App) -> eyre::Result<()> {
    println!("Song index:");
    for (i, id) in app.database.get_song_index().await? {
        println!("{i}: {id}");
    }
    Ok(())
}

pub async fn download(
    app: &'static App,
    amount: Option<usize>,
    indexes: Option<Vec<usize>>,
) -> eyre::Result<()> {
    let indexes = match (amount, indexes) {
        (None, None) | (Some(_), Some(_)) => bail!("Specify one of --amount, --index"),
        (None, Some(indexes)) => {
            let mut mapped_indexes = Vec::with_capacity(indexes.len());
            for index in indexes {
                let song_id = app
                    .database
                    .get_song_id_by_index(index)
                    .await?
                    .ok_or(eyre!("Index {index} not found"))?;
                mapped_indexes.push((index, song_id));
            }
            mapped_indexes
        }
        (Some(amount), None) => {
            let downloaded_song_ids = app.database.get_all_downloaded_song_ids().await?;
            app.database
                .get_song_index()
                .await?
                .into_iter()
                .filter(|(_, id)| !downloaded_song_ids.contains(id))
                .choose_multiple(&mut thread_rng(), amount)
        }
    };

    println!("Songs to be downloaded: {indexes:?}\n");
    for (index, id) in indexes {
        println!("\nDownloading song {index}: {id}:");
        if let Err(e) = command::songs::download(app, id.to_string(), None, U256::MAX).await {
            eprintln!("Could not download song: {e:#}")
        }
    }

    Ok(())
}
