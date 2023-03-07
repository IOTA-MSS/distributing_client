use crate::library::{config::Config, crypto::Wallet, database::Database};
use std::io::stdin;

pub async fn generate(password: Option<String>, cfg: Config) -> eyre::Result<()> {
    let key = Wallet::generate().private_key();
    let database = cfg.initialize_database().await?;
    set_key_with_confirmation(&database, key, password).await?;

    Ok(())
}

pub async fn import(password: Option<String>, key: String, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    set_key_with_confirmation(&database, key, password).await?;

    Ok(())
}

pub async fn export_address(cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let wallet = cfg.decrypt_wallet(&database).await?;
    println!("Your address: {:?}", wallet.address());

    Ok(())
}

pub async fn export_private_key(cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let wallet = cfg.decrypt_wallet(&database).await?;
    println!("Your address: {:?}", wallet.private_key());

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
    if line.starts_with("y") || line.starts_with("Y") {
        println!("Ok!");
        Ok(true)
    } else {
        println!("Canceling...");
        Ok(false)
    }
}
