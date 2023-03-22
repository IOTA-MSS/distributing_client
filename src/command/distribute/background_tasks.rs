use crate::{command, library::app::AppData};
use chrono::{DateTime, Utc};
use interprocess::local_socket::tokio::{LocalSocketListener, LocalSocketStream};
use std::{convert::Infallible, sync::Mutex, time::Duration};
use tokio::{sync::oneshot, time::MissedTickBehavior};

/// Automatically downloads new songs from the smart-contract, and watches for new songs added
/// to the database.
pub async fn auto_distribute(app: &'static AppData) -> Infallible {
    println!("Auto-distributor spawned");
    const INTERVAL: Duration = Duration::from_secs(10);
    let mut interval = tokio::time::interval_at(tokio::time::Instant::now() + INTERVAL, INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut last_distribution = Utc::now();

    loop {
        interval.tick().await;

        if let Err(e) = download_new_songs(app).await {
            println!("Couldn't download new songs: {e}");
        }

        if let Err(e) = distribute_new_songs(app, &last_distribution).await {
            println!("Couldn't distribute new songs: {e}");
        }
        last_distribution = Utc::now() - chrono::Duration::milliseconds(10);
    }
}

pub async fn run_local_socket(app: &'static AppData) -> eyre::Result<Infallible> {
    let listener = LocalSocketListener::bind(app.server_address.to_string())?;
    loop {
        match listener.accept().await {
            Ok(conn) => {
                tokio::task::spawn(accept_local_socket_conn(app, conn));
            }
            Err(e) => {
                println!("Local socket error: {e}");
                continue;
            }
        };
    }
}

pub async fn accept_local_socket_conn(
    app: &'static AppData,
    conn: LocalSocketStream,
) -> eyre::Result<()> {
    todo!()
}

/// Downloads any songs newly published on the smart-contract
async fn download_new_songs(app: &'static AppData) -> eyre::Result<()> {
    let index = app.database.get_next_song_index().await?;
    let new_songs = app.client.get_song_ids_from_index(index).await?;
    app.database.add_to_song_index(&new_songs).await?;

    for (_, id) in new_songs {
        match command::songs::download(app, id.to_string(), None).await {
            Ok(()) => println!("Succesfully downloaded song {id}"),
            Err(e) => println!("Couldn't download song: {e}"),
        }
    }

    Ok(())
}

/// Distributes any songs added from a certain time.
async fn distribute_new_songs(app: &'static AppData, from: &DateTime<Utc>) -> eyre::Result<()> {
    for song_id in app.database.get_new_songs(from).await? {
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
    Ok(())
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
