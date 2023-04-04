use crate::{
    command,
    library::{app::App, util::SongId},
};
use chrono::{DateTime, Utc};
use ethers::types::U256;
use std::{
    cmp::Reverse, collections::BinaryHeap, convert::Infallible, sync::Mutex, time::Duration,
};
use tokio::{
    sync::oneshot,
    time::{Instant, MissedTickBehavior},
};

const POLL_INTERVAL: Duration = Duration::from_secs(15);

/// A future that returns when ctrl-c is pressed in the terminal.
/// This can be used to gracefully exit by undistributing songs.
pub fn exit_listener() -> eyre::Result<oneshot::Receiver<()>> {
    static EXIT_SIGNAL: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);

    let (tx, rx) = oneshot::channel();
    *EXIT_SIGNAL.lock().unwrap() = Some(tx);
    ctrlc::set_handler(|| {
        let _ = EXIT_SIGNAL.lock().unwrap().take().unwrap().send(());
    })?;
    Ok(rx)
}

/// Automatically downloads new songs from the smart-contract, and watches for new songs added
/// to the database.
pub async fn auto_distribute(app: &'static App, auto_download: bool) -> Infallible {
    println!("Auto-distributor spawned!");
    println!("Automatically downloading new songs: {auto_download}");

    // Create the interval
    let mut interval =
        tokio::time::interval_at(tokio::time::Instant::now() + POLL_INTERVAL, POLL_INTERVAL);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    // Create the song-queue
    let mut queue = {
        let mut queue = NewSongQueue::new(Duration::from_secs(60));
        let downloaded_ids = app.database.get_all_downloaded_song_ids().await.unwrap();
        app.database
            .get_song_index()
            .await
            .unwrap()
            .into_iter()
            .filter(|(_, id)| (!downloaded_ids.contains(id)))
            .for_each(|(index, id)| queue.push(index, id));
        queue
    };

    loop {
        let last_distribution = Utc::now() - chrono::Duration::milliseconds(10);
        interval.tick().await;

        if auto_download {
            if let Err(e) = download_a_new_song(app, &mut queue).await {
                println!("Couldn't download new songs: {e}");
            }
        }

        if let Err(e) = distribute_added_songs(app, &last_distribution).await {
            println!("Couldn't distribute new songs: {e}");
        }
    }
}

/// Downloads a new song newly published on the smart-contract
async fn download_a_new_song(app: &'static App, queue: &mut NewSongQueue) -> eyre::Result<()> {
    loop {
        // update the queue with new songs
        for (index, id) in command::song_index::update(app).await? {
            queue.push(index, id);
        }

        // Take the front element from the queue
        let Some((_index, id)) = queue.now() else {
            return Ok(())
        };

        // If the song is already downloaded, find the next one
        if app.database.get_chunks(id, 0, 1).await.is_ok() {
            queue.update(true);
            continue;
        };

        // Finally download the song to the database
        let id = id.to_string();
        return match command::songs::download(app, id.clone(), None, U256::MAX).await {
            Ok(()) => {
                // If it was okay we can remove it from the queue
                queue.update(true);
                println!("Succesfully downloaded song {id}");
                Ok(())
            }
            Err(e) => {
                // Otherwise we push it back
                queue.update(false);
                Err(e)
            }
        };
    }
}

/// Distributes any songs added from a certain time.
async fn distribute_added_songs(app: &'static App, from: &DateTime<Utc>) -> eyre::Result<()> {
    for song_id in app.database.get_new_songs(from).await? {
        println!("Registering for song {song_id}...");
        if let Ok(pending_tx) = app
            .client
            .distribute_call(vec![(song_id, app.fee)])
            .await?
            .send()
            .await
        {
            if (pending_tx.await).is_ok() {
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

//------------------------------------------------------------------------------------------------
//  NewSongQueue
//------------------------------------------------------------------------------------------------

/// A queue of new songs to be downloaded, implemented as a binary-heap
struct NewSongQueue {
    heap: BinaryHeap<Reverse<(Instant, usize, usize, SongId)>>,
    duration: Duration,
}

impl NewSongQueue {
    pub fn push(&mut self, index: usize, id: SongId) {
        self.heap.push(Reverse((Instant::now(), 0, index, id)))
    }

    pub fn new(duration: Duration) -> Self {
        Self {
            heap: BinaryHeap::new(),
            duration,
        }
    }

    /// Whether a new song must be downloaded now.
    ///
    /// To be combined with a `update` call.
    pub fn now(&self) -> Option<(&usize, &SongId)> {
        match self.heap.peek() {
            Some(Reverse((instant, _times, index, id))) => {
                if instant <= &Instant::now() {
                    Some((index, id))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Whether the song was succesfully downloaded.
    ///
    /// If success=true, then the first song will be removed.
    /// If success=false, then the timer before a new download is started is incremented by 30 secs.
    pub fn update(&mut self, success: bool) {
        if success {
            self.heap.pop().unwrap();
        } else {
            let mut peek = self.heap.peek_mut().unwrap();
            let Reverse((instant, times, _index, _id)) = &mut *peek;
            *times += 1;
            *instant = Instant::now() + (self.duration * (*times as u32));
        }
    }
}

#[cfg(test)]
mod test {
    use super::NewSongQueue;
    use crate::library::util::SongId;
    use std::time::Duration;

    #[test]
    fn test_new_song_queue() {
        let mut queue = NewSongQueue::new(Duration::from_millis(10));
        queue.push(0, SongId::from([0; 32]));
        std::thread::sleep(Duration::from_millis(10));
        queue.push(1, SongId::from([1; 32]));

        assert_eq!(queue.now().unwrap(), (&0, &SongId::from([0; 32])));
        queue.update(false);
        assert_eq!(queue.now().unwrap(), (&1, &SongId::from([1; 32])));
        queue.update(true);

        assert!(queue.now().is_none());
        std::thread::sleep(Duration::from_millis(20));
        assert!(queue.now().is_some());
        queue.update(true);
        assert!(queue.now().is_none());
    }
}
