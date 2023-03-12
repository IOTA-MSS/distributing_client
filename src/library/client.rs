use crate::util::SongId;

use super::{
    abi::{DepositCall, GetChunksCall, TangleTunesAbi},
    crypto::Wallet,
    receipt_ext::TransactionReceiptExt,
};
use ethers::{
    abi::{AbiDecode, AbiEncode},
    prelude::*,
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, Address, TransactionReceipt},
    utils::rlp::{Decodable, Rlp},
};
use ethers_providers::{Http, Middleware, Provider};
use std::{ops::Deref, str::FromStr, sync::Arc};

type PendingTangleTunesTransaction<'a> = PendingTransaction<'a, Http>;

pub type TangleTunesCall<T> =
    ContractCall<SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>, T>;

/// The client used to connect to the IOTA network.
#[derive(Debug)]
pub struct TangleTunesClient {
    pub abi_client:
        TangleTunesAbi<SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>>,
    wallet: Wallet,
}

impl TangleTunesClient {
    pub async fn get_receipt(&self, hash: impl Into<TxHash>) -> eyre::Result<TransactionReceipt> {
        match self
            .abi_client
            .client_ref()
            .get_transaction_receipt(hash.into())
            .await
        {
            Ok(receipt) => receipt.ok_or(eyre!("No transaction receipt")),
            Err(e) => Err(e.into()),
        }
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

    pub async fn get_chunks_rlp(
        &self,
        song_id: SongId,
        from: usize,
        amount: usize,
        distributor: Address,
    ) -> eyre::Result<Bytes> {
        let mut tx = self
            .abi_client
            .get_chunks(song_id.into(), from.into(), amount.into(), distributor)
            .legacy()
            .gas(100_000)
            .gas_price(1)
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

    pub async fn send_raw_tx<'a>(
        &'a self,
        rlp: Bytes,
    ) -> Result<PendingTransaction<'a, Http>, eyre::Report> {
        Ok(self
            .abi_client
            .deref()
            .client_ref()
            .inner()
            .inner()
            .send_raw_transaction(rlp)
            .await?)
    }

    pub async fn deposit(&self, amount: u64) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .deposit()
            .value(amount)
            .gas(100_000)
            .gas_price(1)
            .legacy()
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok()?;
        Ok(receipt)
    }

    pub fn distribute2(&self, song_id: SongId, fee: u32) -> eyre::Result<TangleTunesCall<()>> {
        Ok(self
            .abi_client
            .distribute(song_id.into(), fee.into())
            .gas(100_000)
            .gas_price(1)
            .legacy())
    }

    pub async fn distribute(
        &self,
        song_id: SongId,
        fee: u32,
    ) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .distribute(song_id.into(), fee.into())
            .gas(100_000)
            .gas_price(1)
            .legacy()
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok()?;
        Ok(receipt)
    }

    pub fn undistribute(&self, song_id: SongId) -> eyre::Result<TangleTunesCall<()>> {
        Ok(self
            .abi_client
            .undistribute(song_id.into())
            .gas(100_000)
            .gas_price(1)
            .legacy())
    }

    pub async fn withdraw(&self, amount: u64) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .withdraw(amount.into())
            .gas(100_000)
            .gas_price(1)
            .legacy()
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok()?;
        Ok(receipt)
    }

    pub async fn delete_user(&self) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .delete_user()
            .legacy()
            .gas(100_000)
            .gas_price(1)
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok()?;
        Ok(receipt)
    }

    pub async fn create_user(
        &self,
        name: String,
        description: String,
    ) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .create_user(name, description)
            .legacy()
            .gas(100_000)
            .gas_price(1)
            .send()
            .await?
            .await?
            .unwrap();

        if receipt.status.unwrap() != 1.into() {
            bail!("Transaction status 0: {receipt:?}")
        }

        Ok(receipt)
    }

    pub(crate) fn client_address(&self) -> Address {
        self.abi_client.address()
    }

    pub(crate) fn wallet_address(&self) -> Address {
        self.abi_client.client_ref().signer().address()
    }
}

#[cfg(test)]
mod test {
    use ethers::abi::Address;
    // use hex::FromHex;

    use crate::{library::crypto::Wallet, test, util::to_hex_prefix};

    #[tokio::test]
    async fn deposit_money_to_wallet() {
        let wallet = Wallet::generate(test::CHAIN_ID);

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
            .arg(to_hex_prefix(address.as_bytes()))
            .arg("--chain=testchain")
            .arg("base")
            .arg(":")
            .arg(amount.to_string())
            .output()
            .await
            .unwrap()
    }
}
