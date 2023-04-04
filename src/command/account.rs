use crate::library::{abi::UserInfo, app::App, client::WEI_PER_IOTA, util::TransactionReceiptExt};

pub async fn create(
    name: String,
    description: Option<String>,
    app: &'static App,
) -> eyre::Result<()> {
    println!("Creating user...");
    app.client
        .create_user_call(name, description.unwrap_or(String::new()))
        .send()
        .await?
        .await?
        .unwrap()
        .status_is_ok("")?;
    println!("Succesfully created user!");
    Ok(())
}

pub async fn delete(app: &'static App) -> eyre::Result<()> {
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

pub async fn deposit(iota: u64, app: &'static App) -> eyre::Result<()> {
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

pub async fn withdraw(iota: u64, app: &'static App) -> eyre::Result<()> {
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

pub(crate) async fn view(app: &App) -> eyre::Result<()> {
    let UserInfo {
        exists: _,
        username,
        description,
        server,
        balance,
        is_validator,
    } = app
        .client
        .get_user_info(app.client.wallet_address())
        .await?;

    println!("-- Your TangleTunes account --");
    println!("| username: {username}");
    println!("| description: {description}");
    println!("| server: {server}");
    println!("| balance: {} IOTA", balance / WEI_PER_IOTA);
    println!("| validator: {is_validator}");

    Ok(())
}
