use crate::library::app::AppData;

pub async fn run_create(
    name: String,
    description: Option<String>,
    app: &'static AppData,
) -> eyre::Result<()> {
    dbg!(
        app.client
            .create_user(name, description.unwrap_or("".to_string()))
            .await?
    );
    println!("Succesfully created user!");
    Ok(())
}

pub async fn run_delete(app: &'static AppData) -> eyre::Result<()> {
    app.client.delete_user().await?;
    println!("Succesfully deleted user!");
    Ok(())
}

pub async fn run_deposit(amount: u64, app: &'static AppData) -> eyre::Result<()> {
    app.client.deposit(amount as u128).await?;
    println!("Succesfully deposited to the smart contract!");
    Ok(())
}

pub async fn run_withdraw(iota: u64, app: &'static AppData) -> eyre::Result<()> {
    app.client.withdraw(iota as u128).await?;
    println!("Succesfully withdrew from the smart contract!");
    Ok(())
}
