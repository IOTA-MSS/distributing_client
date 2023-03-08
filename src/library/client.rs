use crate::CHAIN_ID_ETH;

use super::{
    abi::{GetChunksCall, TangleTunesAbi},
    crypto::Wallet,
};
use ethers::{
    abi::AbiDecode,
    prelude::*,
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, Address, TransactionReceipt, U256},
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::{Http, Middleware, Provider};
use hex::FromHex;
use std::{ops::Deref, str::FromStr, sync::Arc};

/// The client used to connect to the IOTA network.
#[derive(Debug)]
pub struct TangleTunesClient {
    pub abi_client:
        TangleTunesAbi<SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>>,
    wallet: Wallet,
}

impl TangleTunesClient {
    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    pub async fn initialize(
        wallet: Wallet,
        node_url: &str,
        contract_address: &str,
    ) -> eyre::Result<Self> {
        let contract = Self {
            abi_client: TangleTunesAbi::new(
                Address::from_str(contract_address).unwrap(),
                Arc::new(SignerMiddleware::new(
                    NonceManagerMiddleware::new(Provider::try_from(node_url)?, wallet.address()),
                    wallet.local_wallet().clone(),
                )),
            ),
            wallet,
        };

        contract
            .abi_client
            .client_ref()
            .inner()
            .initialize_nonce(None)
            .await?;

        Ok(contract)
    }

    pub async fn create_get_chunks_signed_tx_rlp(
        &self,
        song_id: &str,
        from: usize,
        amount: usize,
        distributor: Address,
    ) -> eyre::Result<Bytes> {
        let mut tx = self
            .abi_client
            .get_chunks(
                FromHex::from_hex(song_id)?,
                from.into(),
                amount.into(),
                distributor,
            )
            .legacy()
            .gas(100_000)
            .tx;

        tx.set_nonce(self.abi_client.client_ref().inner().next());

        dbg!(tx.nonce());
        let rlp = tx.rlp_signed(&self.wallet.local_wallet().sign_transaction_sync(&tx));

        Ok(rlp)
    }

    pub fn decode_get_chunks_tx_rlp(&self, tx_rlp: &[u8]) -> eyre::Result<GetChunksCall> {
        let tx = TypedTransaction::decode(&Rlp::new(tx_rlp))?;
        Ok(AbiDecode::decode(tx.data().unwrap())?)
    }

    pub async fn send_raw_tx(&self, rlp: Bytes) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .deref()
            .client()
            .send_raw_transaction(rlp)
            .await?
            .await?)
    }

    pub async fn call_deposit(&self, amount: u64) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .deposit()
            .value(amount)
            .gas(100_000)
            .legacy()
            .send()
            .await?
            .await?)
    }

    pub async fn call_withdraw(&self, amount: u64) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .withdraw(amount.into())
            .gas(100_000)
            .legacy()
            .send()
            .await?
            .await?)
    }

    pub async fn call_delete_user(&self) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .delete_user()
            .legacy()
            .gas(100_000)
            .send()
            .await?
            .await?)
    }

    pub async fn call_create_user(
        &self,
        name: String,
        description: String,
    ) -> eyre::Result<Option<TransactionReceipt>> {
        Ok(self
            .abi_client
            .create_user(name, description)
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
