use crate::lib::app::App;

pub async fn create(name: String, description: Option<String>, cfg: App) -> eyre::Result<()> {
    let client = cfg.initialize_client(&cfg.database).await?;
    dbg!(client
        .create_user(name, description.unwrap_or("".to_string()))
        .await?);
    println!("Succesfully created user!");
    Ok(())
}

pub async fn delete(cfg: App) -> eyre::Result<()> {
    let client = cfg.initialize_client(&cfg.database).await?;
    client.delete_user().await?;
    println!("Succesfully deleted user!");
    Ok(())
}

pub async fn deposit(amount: u64, cfg: App) -> eyre::Result<()> {
    let client = cfg.initialize_client(&cfg.database).await?;
    client.deposit(amount).await?;
    println!("Succesfully deposited to the smart contract!");
    Ok(())
}

pub async fn withdraw(amount: u64, cfg: App) -> eyre::Result<()> {
    let client = cfg.initialize_client(&cfg.database).await?;
    client.withdraw(amount).await?;
    println!("Succesfully withdrew from the smart contract!");
    Ok(())
}
