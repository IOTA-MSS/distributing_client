use crate::library::config::Config;

pub async fn create(name: String, description: Option<String>, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let client = cfg.initialize_client(&database).await?;
    client
        .call_create_user(name, description.unwrap_or("".to_string()))
        .await?;
    println!("Succesfully created user!");
    Ok(())
}

pub async fn delete(cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let client = cfg.initialize_client(&database).await?;
    client.call_delete_user().await?;
    println!("Succesfully deleted user!");
    Ok(())
}

pub async fn deposit(amount: u64, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let client = cfg.initialize_client(&database).await?;
    client.call_deposit(amount).await?;
    println!("Succesfully deposited to the smart contract!");
    Ok(())
}

pub async fn withdraw(amount: u64, cfg: Config) -> eyre::Result<()> {
    let database = cfg.initialize_database().await?;
    let client = cfg.initialize_client(&database).await?;
    client.call_withdraw(amount).await?;
    println!("Succesfully withdrew from the smart contract!");
    Ok(())
}
