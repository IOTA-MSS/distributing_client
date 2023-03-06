use super::{abi::TangleTunesAbi, crypto::Wallet};
use ethers::{
    abi::{Function, Token},
    prelude::*,
    prelude::{abigen, builders::ContractCall, NonceManagerMiddleware, SignerMiddleware},
    signers::{LocalWallet, Signer},
    types::{
        transaction::eip2718::TypedTransaction, Address, Transaction, TransactionReceipt,
        TransactionRequest, H160, U256,
    },
};
use ethers_providers::{Http, Middleware, PendingTransaction, Provider, ProviderError};
use std::{str::FromStr, sync::Arc};

// abigen! {
//     TangleTunesAbi,
//     "../smart_contract/abi/contracts/TangleTunes.sol/TangleTunes.json"
//     // "https://raw.githubusercontent.com/DanielMelero/IOTA-MSS/a2a7fe394601b672fd89fed4e3089b732aea5eaa/contract/Platform.abi";
// }

#[derive(Debug)]
pub struct TangleTunesClient {
    raw_client: Provider<Http>,
    pub client:
        TangleTunesAbi<SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>>,
}

impl TangleTunesClient {
    pub async fn init(
        wallet: &Wallet,
        node_url: &str,
        contract_address: &str,
    ) -> eyre::Result<Self> {
        let provider = Provider::try_from(node_url)?;
        let client = NonceManagerMiddleware::new(provider.clone(), wallet.address());
        client.initialize_nonce(None).await?;
        let client = SignerMiddleware::new(client, wallet.inner().clone());

        let contract = Self {
            // wallet,
            raw_client: provider,
            client: TangleTunesAbi::new(
                Address::from_str(contract_address).unwrap(),
                Arc::new(client),
            ),
        };
        Ok(contract)
    }

    pub async fn deposit(&self, amount: u64) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .client
            .deposit()
            .value(amount)
            .legacy()
            .send()
            .await?
            .await?)
    }

    pub async fn send_raw_tx(&self, tx: &Transaction) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self.raw_client.send_transaction(tx, None).await?.await?)
    }

    fn get_contract_function(&self, name: &str) -> eyre::Result<&Function> {
        Ok(self.client.abi().function(name)?)
    }

    pub fn decode_chunk_tx_input(&self, input_data: &[u8]) -> eyre::Result<(String, u32, u32)> {
        let res = self
            .get_contract_function("chunks")?
            .decode_input(input_data)?;

        let Some(Token::String(id)) = res.get(0) else {
            Err(eyre!("Invalid chunk tx: {res:?} from {input_data:?}"))?
        };
        let Some(Token::Uint(from)) = res.get(1) else {
            Err(eyre!("Invalid chunk tx: {res:?} from {input_data:?}"))?
        };
        let Some(Token::Uint(chunks)) = res.get(2) else {
            Err(eyre!("Invalid chunk tx: {res:?} from {input_data:?}"))?
         };

        Ok((id.to_owned(), from.as_u32(), chunks.as_u32()))
    }

    pub async fn users(
        &self,
        address: Address,
    ) -> eyre::Result<(bool, String, String, String, U256, bool)> {
        Ok(self.client.users(address).await?)
    }

    pub async fn create_account(
        &self,
        name: &str,
        description: &str,
    ) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .client
            .create_user(String::from(name), String::from(description))
            .legacy()
            .gas(100_000)
            .send()
            .await?
            .await?)
    }
}

#[cfg(test)]
mod test {
    use ethers::abi::AbiEncode;

    use super::*;
    use crate::crypto;
    use std::process::Output;

    // #[tokio::test]
    // async fn deposit_money_to_wallet() {
    //     let wallet = Wallet::from_private_key(DEFAULT_SECRET).unwrap();
    //     let wallet = wallet::generate_new();

    //     let contract =
    //         TangleTunesClient::init(wallet.clone(), DEFAULT_NODE_URL, DEFAULT_CONTRACT_ADDRESS)
    //             .await
    //             .unwrap();

    //     let address = contract.wallet_address();
    //     send_funds_to(&address, 1000).await;
    //     contract.users(address).await.unwrap();
    //     contract
    //         .create_account("Testing", "Test account")
    //         .await
    //         .unwrap();
    //     contract.users(address).await.unwrap();
    // }

    // #[tokio::test]
    // async fn test() {
    //     // let call: ContractCall<_, ()> = self
    //     //     .client
    //     //     .get_chunk(
    //     //         [
    //     //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    //     //             0, 0, 0, 0, 0, 0,
    //     //         ],
    //     //         10.into(),
    //     //     )
    //     //     .legacy();
    //     // let TypedTransaction::Legacy(tx) = call.tx else {
    //     //     panic!();
    //     // };
    // }

    // async fn send_funds_to(address: &Address, amount: u64) -> Output {
    //     tokio::process::Command::new("wasp-cli")
    //         .arg("chain")
    //         .arg("deposit")
    //         .arg(address.encode_hex())
    //         .arg("--chain=testchain")
    //         .arg("base")
    //         .arg(":")
    //         .arg(amount.to_string())
    //         .output()
    //         .await
    //         .unwrap()
    // }
}
