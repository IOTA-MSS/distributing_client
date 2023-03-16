use crate::{
    library::app::AppData,
    library::util::{SongId, TransactionReceiptExt},
};
use std::{fs::OpenOptions, io::Write};

pub async fn run_download(
    app: &'static AppData,
    ip_address: String,
    song_id: String,
    to_file: Option<(String, usize, usize)>,
) -> eyre::Result<()> {
    let Some((file, first_chunk_id, chunks_requested)) = to_file else {
        bail!("Downloading to database not yet implemented. Specify `--to_file <FILE>`")
    };

    let song = app
        .client
        .download_chunks(
            ip_address,
            SongId::try_from_hex(&song_id)?,
            first_chunk_id,
            chunks_requested,
            app.client.wallet_address(),
        )
        .await?;

    let mut file = OpenOptions::new().write(true).create(true).open(file)?;
    file.write_all(&song)?;
    file.flush()?;

    Ok(())
}
