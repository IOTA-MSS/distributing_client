use std::time::Duration;

use crate::library::{app::AppData, util::TransactionReceiptExt};
use color_eyre::Report;
use ethers_providers::StreamExt;
use futures::stream::FuturesUnordered;
use tokio::time::sleep;

/// Registers for distribution of all songs in the database.
/// If an error occurs, all songs are automatically deregistered.
pub async fn register_for_songs(app: &AppData) -> eyre::Result<()> {
    let mut pending_txs = FuturesUnordered::new();

    // Send all transactions until complete or an error is encountered
    let sending_txs_result = {
        for song_id in app.database.get_all_downloaded_song_ids().await? {
            println!(
                "Registering for distribution of song {song_id} with nonce {}...",
                app.client.get_nonce().await?
            );

            let tx_hash = app
                .client
                .distribute_call(song_id, app.fee)
                .send()
                .await?
                .tx_hash();
            sleep(Duration::from_secs(2)).await;

            pending_txs.push(async move {
                let receipt = app
                    .client
                    .create_pending_tx(tx_hash, 1)
                    .await?
                    .status_is_ok(&format!("Could not register song with id {song_id}"))?;
                Ok::<_, Report>((receipt, song_id))
            })
        }

        Ok::<_, eyre::Report>(())
    };

    // Wait for all confirmations to be received.
    // If an error occurs we just log it and continue.
    while let Some(result) = pending_txs.next().await {
        match result {
            Ok((_, song_id)) => println!("Registered song {song_id}"),
            Err(e) => println!("ERROR: {e}"),
        }
    }

    // If sending of all registrations was successful return that.
    // If an error occured there, we will deregister for all registered songs.
    match sending_txs_result {
        Ok(_) => Ok(()),
        Err(e) => match deregister_for_songs(app).await {
            Ok(_) => Err(e),
            Err(e2) => Err(e.wrap_err(e2)),
        },
    }
}

/// Deregister for distribution of the given songs.
pub async fn deregister_for_songs(app: &AppData) -> eyre::Result<()> {
    let mut pending_transactions = FuturesUnordered::new();

    for song_id in app.database.get_all_downloaded_song_ids().await? {
        let result = {
            println!("Deregistering song {song_id} on the smart-contract..");
            let tx_hash = app
                .client
                .undistribute_call(song_id)
                .send()
                .await?
                .tx_hash();
            sleep(Duration::from_secs(1)).await;

            pending_transactions.push(async move {
                let receipt = app
                    .client
                    .create_pending_tx(tx_hash, 1)
                    .await?
                    .unwrap()
                    .status_is_ok(&format!("Could not deregister song with id {song_id}"))?;
                Ok::<_, eyre::Report>((receipt, song_id))
            });
            Ok::<_, eyre::Report>(())
        };
        if let Err(e) = result {
            println!("ERROR: Problem deregistering song: {e}");
        };
    }

    while let Some(result) = pending_transactions.next().await {
        match result {
            Ok((_, song_id)) => println!("Deregistered song {song_id}"),
            Err(e) => println!("ERROR: {e}"),
        }
    }
    Ok(())
}
