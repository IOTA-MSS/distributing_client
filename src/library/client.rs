use super::{
    abi::{GetChunksCall, TangleTunesAbi},
    crypto::Wallet,
};
use ethers::{
    abi::AbiDecode,
    prelude::*,
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, Address, TransactionReceipt, U256},
};
use ethers_providers::{Http, Middleware, Provider};
use hex::FromHex;
use std::{str::FromStr, sync::Arc};

/// The client used to connect to the IOTA network.
#[derive(Debug)]
pub struct TangleTunesClient {
    raw_client: Provider<Http>,
    abi_client:
        TangleTunesAbi<SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>>,
}

impl TangleTunesClient {
    pub async fn initialize(
        wallet: &Wallet,
        node_url: &str,
        contract_address: &str,
    ) -> eyre::Result<Self> {
        let provider = Provider::try_from(node_url)?;
        let client = NonceManagerMiddleware::new(provider.clone(), wallet.address());
        client.initialize_nonce(None).await?;
        let client = SignerMiddleware::new(client, wallet.local_wallet().clone());

        let contract = Self {
            raw_client: provider,
            abi_client: TangleTunesAbi::new(
                Address::from_str(contract_address).unwrap(),
                Arc::new(client),
            ),
        };
        Ok(contract)
    }

    pub fn get_chunks_call(
        &self,
        song_id: &str,
        from: u32,
        amount: u32,
        distributor: Address,
    ) -> eyre::Result<
        FunctionCall<
            Arc<SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>>,
            SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>,
            (),
        >,
    > {
        Ok(self
            .abi_client
            .get_chunks(
                FromHex::from_hex(song_id)?,
                from.into(),
                amount.into(),
                distributor,
            )
            .legacy()
            .gas(100_000))
    }

    pub async fn send_raw_tx(
        &self,
        tx: impl Into<TypedTransaction> + Send + Sync,
    ) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self.raw_client.send_transaction(tx, None).await?.await?)
    }

    pub fn decode_get_chunks_call(&self, input_data: &[u8]) -> eyre::Result<GetChunksCall> {
        Ok(AbiDecode::decode(input_data)?)
    }

    pub async fn call_deposit(&self, amount: u64) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .deposit()
            .value(amount)
            .legacy()
            .send()
            .await?
            .await?)
    }

    pub async fn call_users(
        &self,
        address: Address,
    ) -> eyre::Result<(bool, String, String, String, U256, bool)> {
        Ok(self.abi_client.users(address).await?)
    }

    pub async fn call_create_user(
        &self,
        name: &str,
        description: &str,
    ) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .create_user(String::from(name), String::from(description))
            .legacy()
            .gas(100_000)
            .send()
            .await?
            .await?)
    }

    pub(crate) fn address(&self) -> Address {
        self.abi_client.address()
    }
}

#[cfg(test)]
mod test {
    use ethers::abi::{AbiEncode, Address};
    use hex::FromHex;

    use crate::{library::crypto::Wallet, TEST_SONG_HEX_ID, TEST_SONG_ID_SLICE};

    #[tokio::test]
    async fn deposit_money_to_wallet() {
        let wallet = Wallet::generate();

        // let client = TangleTunesClient::initialize(&wallet, TEST_NODE_URL, TEST_CONTRACT_ADDRESS)
        //     .await
        //     .unwrap(); TODO: Rewrite this test

        // let address = client.client.address();
        // send_funds_to(&address, 1000).await;
        // client.call_users(address).await.unwrap();
        // client
        //     .call_create_user("Testing", "Test account")
        //     .await
        //     .unwrap();
        // client.call_users(address).await.unwrap();
    }

    #[test]
    fn hex_encode() {
        use hex::ToHex;
        let hex_id = ToHex::encode_hex::<String>(&TEST_SONG_ID_SLICE);
        assert_eq!(&hex_id, TEST_SONG_HEX_ID);
        let new_song_id: Vec<u8> = FromHex::from_hex(&hex_id).unwrap();
        assert_eq!(&TEST_SONG_ID_SLICE, new_song_id.as_slice());
    }

    #[tokio::test]
    async fn test() {
        // let call: ContractCall<_, ()> = self
        //     .client
        //     .get_chunk(
        //         [
        //             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        //             0, 0, 0, 0, 0, 0,
        //         ],
        //         10.into(),
        //     )
        //     .legacy();
        // let TypedTransaction::Legacy(tx) = call.tx else {
        //     panic!();
        // };
    }

    async fn send_funds_to(address: &Address, amount: u64) -> std::process::Output {
        tokio::process::Command::new("wasp-cli")
            .arg("chain")
            .arg("deposit")
            .arg(address.encode_hex())
            .arg("--chain=testchain")
            .arg("base")
            .arg(":")
            .arg(amount.to_string())
            .output()
            .await
            .unwrap()
    }
}
