use crate::library::{app::AppData, crypto::Wallet, database::Database, util::to_hex_prefix};
use std::io::stdin;

pub async fn generate(password: Option<String>, database: Database) -> eyre::Result<()> {
    let key = Wallet::generate(1074).private_key();
    set_key_with_confirmation(&database, key, password).await?;
    Ok(())
}

pub async fn import(password: Option<String>, key: String, database: Database) -> eyre::Result<()> {
    set_key_with_confirmation(&database, key, password).await?;
    Ok(())
}

pub async fn remove(app: &'static AppData) -> eyre::Result<()> {
    if ask_confirmation(
        "Are you sure? This will delete the private key. Make sure it is backed up!",
    )? {
        app.database.remove_private_key().await?;
    }
    Ok(())
}

pub async fn export_address(app: &'static AppData) -> eyre::Result<()> {
    println!("Your address: {:?}", app.client.wallet_address());
    Ok(())
}

pub async fn export_private_key(app: &'static AppData) -> eyre::Result<()> {
    println!(
        "Your private key: {:?}",
        to_hex_prefix(app.client.wallet_private_key().to_bytes())
    );
    Ok(())
}

async fn set_key_with_confirmation(
    db: &Database,
    key: String,
    password: Option<String>,
) -> eyre::Result<()> {
    if db.get_key().await?.is_some()
        && !ask_confirmation(
            "Are your sure? Setting a new key will DELETE the key currently in use.",
        )?
    {
        return Ok(());
    }
    let (key, encrypted) = match password {
        Some(password) => (crate::crypto::encrypt_private_key(&key, &password), true),
        None => (key, false),
    };
    db.set_key(&key, encrypted).await?;

    Ok(())
}

fn ask_confirmation(msg: &str) -> eyre::Result<bool> {
    println!("{msg} [y/N]");
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    if line.starts_with('y') || line.starts_with('y') {
        println!("Ok!");
        Ok(true)
    } else {
        println!("Canceling...");
        Ok(false)
    }
}
