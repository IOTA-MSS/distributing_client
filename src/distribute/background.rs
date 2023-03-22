use crate::library::app::AppData;
use std::{convert::Infallible, time::Duration, sync::Mutex};
use tokio::{time::MissedTickBehavior, sync::oneshot};

pub async fn auto_download_new_songs(app: &'static AppData) -> eyre::Result<Infallible> {
    const INTERVAL: Duration = Duration::from_secs(10);
    let mut interval = tokio::time::interval_at(tokio::time::Instant::now() + INTERVAL, INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        interval.tick().await;
        let index = app.database.get_next_song_index().await?;
        let new_songs = app.client.get_song_ids_from_index(index).await?;
        app.database.add_to_song_index(&new_songs).await?;

        for (_, id) in new_songs {
            match crate::songs::run_download(app, id.to_string(), None).await {
                Ok(()) => println!("Succesfully downloaded song {id}"),
                Err(e) => println!("Couldn't download song: {e}"),
            }
        }
    }
}

pub async fn auto_distribute_added_songs(app: &'static AppData) -> eyre::Result<Infallible> {
    const INTERVAL: Duration = Duration::from_secs(10);
    let mut interval = tokio::time::interval_at(tokio::time::Instant::now() + INTERVAL, INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        interval.tick().await;

        for song_id in app.database.get_new_songs(&INTERVAL).await? {
            println!("Registering for song {song_id}...");
            if let Ok(pending_tx) = app.client.distribute_call(song_id, app.fee).send().await {
                if let Ok(_) = pending_tx.await {
                    println!("Succesfully registered for song {song_id}.");
                } else {
                    println!("Registration for song {song_id} failed.");
                }
            } else {
                println!("Registration for song {song_id} failed.")
            }
        }
    }
}

pub fn exit_listener() -> eyre::Result<oneshot::Receiver<()>> {
    static EXIT_SIGNAL: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);

    let (tx, rx) = oneshot::channel();
    *EXIT_SIGNAL.lock().unwrap() = Some(tx);
    ctrlc::set_handler(|| {
        let _ = EXIT_SIGNAL.lock().unwrap().take().unwrap().send(());
    })?;
    Ok(rx)
}