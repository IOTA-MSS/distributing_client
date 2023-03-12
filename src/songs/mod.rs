mod download;
use std::path::PathBuf;

pub use download::download;

use crate::{
    library::app::App,
    util::{try_from_hex_prefix, SongId},
};

pub async fn remove(ids: Vec<String>, cfg: App) -> eyre::Result<()> {
    for id in &ids {
        if cfg.database.remove_song(&SongId::try_from_hex(id)?).await? {
            println!("Succesfully removed song {id:?}!");
        } else {
            println!("Song with id {id:?} does not exist");
        }
    }
    Ok(())
}

pub async fn add_from_path(paths: Vec<String>, distribute: bool, cfg: App) -> eyre::Result<()> {
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
            .add_song(&SongId::try_from_hex(song_id)?, distribute, &data)
            .await?;
        println!("Succesfully added song with id {}", song_id);
    }
    Ok(())
}

pub async fn stop_distribution(ids: Vec<String>, cfg: App) -> eyre::Result<()> {
    for id in ids {
        cfg.database
            .set_distribution(&SongId::try_from_hex(id)?, false)
            .await?;
    }
    Ok(())
}

pub async fn start_distribution(ids: Vec<String>, cfg: App) -> eyre::Result<()> {
    for id in ids {
        cfg.database
            .set_distribution(&SongId::try_from_hex(id)?, true)
            .await?;
    }
    Ok(())
}

pub async fn set_fee(ids: Vec<String>, fee: u32, app: App) -> eyre::Result<()> {
    todo!()
}

pub(crate) async fn list(app: App) -> eyre::Result<()> {
    todo!()
}
