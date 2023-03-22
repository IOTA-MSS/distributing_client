use crate::{library::app::AppData, songs};
use rand::{seq::IteratorRandom, thread_rng};

pub(crate) async fn run_update(app: &'static AppData) -> eyre::Result<()> {
    let index = app.database.get_next_song_index().await?;
    let new_ids = app.client.get_song_ids_from_index(index).await?;
    println!("Songs added to index:");
    for (i, id) in &new_ids {
        println!("{i}: {id}");
    }
    app.database.add_to_song_index(&new_ids).await?;
    Ok(())
}

pub(crate) async fn run_reset(app: &'static AppData, update: bool) -> eyre::Result<()> {
    app.database.clear_song_index().await?;
    println!("Song index cleared.\n");
    if update {
        run_update(app).await?;
    }
    Ok(())
}

pub(crate) async fn run_list(app: &'static AppData) -> eyre::Result<()> {
    println!("Song index:");
    for (i, id) in app.database.get_song_index().await? {
        println!("{i}: {id}");
    }
    Ok(())
}

pub(crate) async fn run_download(
    app: &'static AppData,
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
            let downloaded_song_ids = app.database.get_all_song_ids().await?;
            app.database
                .get_song_index()
                .await?
                .into_iter()
                .filter(|(_, id)| !downloaded_song_ids.contains(&id))
                .choose_multiple(&mut thread_rng(), amount)
        }
    };

    for (index, id) in indexes {
        println!("\nDownloading song {index}: {id}:");
        if let Err(e) = songs::run_download(app, id.to_string(), None).await {
            println!("Could not download song: {e}")
        }
    }

    Ok(())
}
