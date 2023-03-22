use crate::library::{app::AppData, util::TransactionReceiptExt};

pub async fn create(
    name: String,
    description: Option<String>,
    app: &'static AppData,
) -> eyre::Result<()> {
    println!("Creating user...");
    app.client
        .create_user_call(name, description.unwrap_or("".to_string()))
        .send()
        .await?
        .await?
        .unwrap()
        .status_is_ok("")?;
    println!("Succesfully created user!");
    Ok(())
}

pub async fn delete(app: &'static AppData) -> eyre::Result<()> {
    println!("Deleting user...");
    app.client
        .delete_user_call()
        .send()
        .await?
        .await?
        .unwrap()
        .status_is_ok("")?;
    println!("Succesfully deleted user!");
    Ok(())
}

pub async fn deposit(iota: u64, app: &'static AppData) -> eyre::Result<()> {
    println!("Depositing to account...");
    app.client
        .deposit_call(iota as u128)
        .send()
        .await?
        .await?
        .unwrap()
        .status_is_ok("")?;
    println!("Succesfully deposited to the smart contract!");
    Ok(())
}

pub async fn withdraw(iota: u64, app: &'static AppData) -> eyre::Result<()> {
    println!("Withdrawing from account...");
    app.client
        .withdraw_call(iota as u128)
        .send()
        .await?
        .await?
        .unwrap()
        .status_is_ok("")?;
    println!("Succesfully withdrew from the smart contract!");
    Ok(())
}
