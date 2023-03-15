mod download;
use crate::{library::app::AppData, library::util::SongId};
pub use download::run_download;
use std::path::PathBuf;

pub async fn run_remove(ids: Vec<String>, cfg: &'static AppData) -> eyre::Result<()> {
    for id in &ids {
        if cfg.database.remove_song(&SongId::try_from_hex(id)?).await? {
            println!("Succesfully removed song {id:?}!");
        } else {
            println!("Song with id {id:?} does not exist");
        }
    }
    Ok(())
}

pub async fn run_add(paths: Vec<String>, distribute: bool, cfg: &'static AppData) -> eyre::Result<()> {
    for path in paths {
        let path = PathBuf::from(path);
        let Some(ext) = path.extension() else {
            bail!("File name must be {{song-id}}.mp3")
        };
        if ext != "mp3" {
            bail!("File name must be {{song-id}}.mp3")
        }
        let data = std::fs::read(&path)?;
        let song_id = path.file_stem().unwrap().to_str().unwrap();
        cfg.database
            .add_song(&SongId::try_from_hex(song_id)?, &data)
            .await?;
        println!("Succesfully added song with id {}", song_id);
    }
    Ok(())
}

pub async fn run_stop_distribution(ids: Vec<String>, cfg: &'static AppData) -> eyre::Result<()> {
    // for id in ids {
    //     cfg.database
    //         .set_distribution(&SongId::try_from_hex(id)?, false)
    //         .await?;
    // }
    unimplemented!();
    Ok(())
}

pub async fn run_start_distribution(ids: Vec<String>, cfg: &'static AppData) -> eyre::Result<()> {
    todo!();
    // for id in ids {
    //     cfg.database
    //         .set_distribution(&SongId::try_from_hex(id)?, true)
    //         .await?;
    // }
    Ok(())
}

pub async fn run_set_fee(ids: Vec<String>, fee: u32, app: &'static AppData) -> eyre::Result<()> {
    todo!()
}

pub(crate) async fn run_list(app: &'static AppData) -> eyre::Result<()> {
    for (i, song_id) in app.database.get_song_ids().await?.into_iter().enumerate() {
        println!("{i}: {song_id}");
    }
    Ok(())
}
