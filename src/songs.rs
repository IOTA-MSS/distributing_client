use std::path::PathBuf;

use crate::library::config::Config;

pub async fn remove(ids: Vec<String>, cfg: Config) -> eyre::Result<()> {
    let db = cfg.initialize_database().await?;
    for id in &ids {
        if db.remove_song(id).await? {
            println!("Succesfully removed song {id:?}!");
        } else {
            println!("Song with id {id:?} does not exist");
        }
    }
    Ok(())
}

pub async fn add_from_path(paths: Vec<String>, distribute: bool, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
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
        database.add_song(song_id, distribute, &data).await?;
        println!("Succesfully added song with id {}", song_id);
    }
    Ok(())
}

pub async fn stop_distribution(ids: Vec<String>, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    for id in ids {
        database.set_distribution(&id, false).await?;
    }
    Ok(())
}

pub async fn start_distribution(ids: Vec<String>, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    for id in ids {
        database.set_distribution(&id, true).await?;
    }
    Ok(())
}

pub async fn download(ids: Vec<String>, distribute: bool, cfg: Config) -> eyre::Result<()> {
    todo!()
}