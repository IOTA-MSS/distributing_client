use crate::{
    library::app::AppData,
    library::{abi::SongInfo, util::SongId},
    BYTES_PER_CHUNK_USIZE,
};
use eyre::Context;
use num_integer::div_ceil;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

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

pub async fn run_add(
    paths: Vec<String>,
    distribute: bool,
    cfg: &'static AppData,
) -> eyre::Result<()> {
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

pub async fn run_set_fee(ids: Vec<String>, fee: u32, app: &'static AppData) -> eyre::Result<()> {
    todo!()
}

pub(crate) async fn run_list(app: &'static AppData) -> eyre::Result<()> {
    for (i, song_id) in app
        .database
        .get_all_song_ids()
        .await?
        .into_iter()
        .enumerate()
    {
        println!("{i}: {song_id}");
    }
    Ok(())
}

pub async fn run_download(
    app: &'static AppData,
    song_id: String,
    to_file: Option<String>,
) -> eyre::Result<()> {
    let song_id = song_id.parse()?;

    let song_info = app.client.call_get_song_info(song_id).await?;

    let (distr_wallet_address, distr_socket_address) =
        app.client.call_get_rand_distributor(song_id).await?;

    if distr_socket_address == "" {
        bail!("No distributor found for song {song_id}");
    }

    let song = app
        .client
        .download_from_distributor(
            distr_socket_address.parse()?,
            song_id,
            0,
            div_ceil(song_info.len.as_usize(), BYTES_PER_CHUNK_USIZE),
            distr_wallet_address,
        )
        .await?;

    match to_file {
        Some(to_file) => {
            let mut file = OpenOptions::new().write(true).create(true).open(&to_file)?;
            file.write_all(&song)?;
            file.flush()?;
            println!("Wrote mp3 to {}", to_file)
        }
        None => {
            app.database.add_song(&song_id, &song).await?;
            println!("Succesfully added song {song_id} to the database");
        }
    }

    Ok(())
}

pub async fn run_download_direct(
    app: &'static AppData,
    socket_address: String,
    song_id: String,
    file: String,
    first_chunk_id: usize,
    chunks_requested: usize,
    distributor_address: String,
) -> eyre::Result<()> {
    let song = app
        .client
        .download_from_distributor(
            socket_address.parse()?,
            SongId::try_from_hex(&song_id)?,
            first_chunk_id,
            chunks_requested,
            distributor_address.parse()?, 
        )
        .await?;

    let mut file = OpenOptions::new().write(true).create(true).open(file)?;
    file.write_all(&song)?;
    file.flush()?;

    Ok(())
}