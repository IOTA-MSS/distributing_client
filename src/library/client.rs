use crate::library::util::SongId;

use super::{
    abi::{GetChunksCall, TangleTunesAbi},
    crypto::Wallet,
    util::TransactionReceiptExt,
};
use ethers::{
    abi::AbiDecode,
    prelude::*,
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, Address, TransactionReceipt},
    utils::rlp::{Decodable, Rlp},
};
use ethers_core::k256::ecdsa::SigningKey;
use ethers_providers::{Http, Middleware, Provider};
use std::{ops::Deref, str::FromStr, sync::Arc};

pub type TTMiddleWare = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;
pub type TTCall<T> = ContractCall<TTMiddleWare, T>;

/// The client used to connect to the IOTA network.
#[derive(Debug)]
pub struct TangleTunesClient {
    pub abi_client:
        TangleTunesAbi<NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>>,
}

impl TangleTunesClient {
    pub fn pending_tx(
        &self,
        hash: impl Into<TxHash>,
        confirmations: usize,
    ) -> PendingTransaction<'_, Http> {
        PendingTransaction::new(hash.into(), self.abi_client.client_ref().inner().inner())
            .confirmations(confirmations)
    }

    pub async fn initialize(
        wallet: Wallet,
        node_url: &str,
        contract_address: &str,
    ) -> eyre::Result<Self> {
        let wallet_address = wallet.address();
        let contract = Self {
            abi_client: TangleTunesAbi::new(
                Address::from_str(contract_address).unwrap(),
                Arc::new(
                    Provider::try_from(node_url)?
                        .with_signer(wallet.local_wallet().clone())
                        .nonce_manager(wallet_address),
                ),
            ),
        };

        contract
            .abi_client
            .client_ref()
            .initialize_nonce(None)
            .await?;

        Ok(contract)
    }

    pub fn distribute(&self, song_id: SongId, fee: u32) -> TTCall<()> {
        self.abi_client
            .distribute(song_id.into(), fee.into())
            .gas(1_000_000)
            .gas_price(1)
            .legacy()
    }

    pub fn edit_server_info(&self, address: String) -> TTCall<()> {
        println!("EDIT_SERVER_INFO: {address}");
        self.abi_client
            .edit_server_info(address)
            .legacy()
            .gas(1_000_000)
            .gas_price(1)
    }

    pub fn undistribute(&self, song_id: SongId) -> TTCall<()> {
        self.abi_client
            .undistribute(song_id.into())
            .legacy()
            .gas(1_000_000)
            .gas_price(1)
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
            .gas(1_000_000)
            .block(BlockId::Number(BlockNumber::Pending))
            .gas_price(1)
            .tx;

        tx.set_nonce(self.abi_client.client_ref().next());
        let signature = self.wallet().sign_transaction_sync(&tx);
        Ok(tx.rlp_signed(&signature))
    }

    pub fn decode_get_chunks_tx_rlp(&self, tx_rlp: &[u8]) -> eyre::Result<GetChunksCall> {
        let tx = TypedTransaction::decode(&Rlp::new(tx_rlp))?;
        Ok(AbiDecode::decode(tx.data().unwrap())?)
    }

    pub async fn send_raw_tx<'a>(
        &'a self,
        tx: Bytes,
    ) -> Result<PendingTransaction<'a, Http>, eyre::Report> {
        Ok(self
            .abi_client
            .deref()
            .client_ref()
            .inner()
            .inner()
            .send_raw_transaction(tx)
            .await?)
    }

    pub async fn deposit(&self, amount: u64) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .deposit()
            .value(amount)
            .gas(1_000_000)
            .gas_price(1)
            .legacy()
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok("Could not register deposit")?;
        Ok(receipt)
    }

    pub async fn withdraw(&self, amount: u64) -> eyre::Result<TransactionReceipt> {
        let receipt = self
            .abi_client
            .withdraw(amount.into())
            .gas(1_000_000)
            .gas_price(1)
            .legacy()
            .send()
            .await?
            .await?
            .unwrap()
            .status_is_ok("Could not register withdraw")?;
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
            .status_is_ok("Could not register delete user")?;
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

    pub async fn nonce(&self) -> eyre::Result<U256> {
        Ok(self.abi_client.client_ref().initialize_nonce(None).await?)
    }

    pub(crate) fn wallet_address(&self) -> Address {
        self.wallet().address()
    }

    pub(crate) fn wallet_private_key(&self) -> &SigningKey {
        self.wallet().signer()
    }

    fn wallet(&self) -> &LocalWallet {
        self.abi_client.client_ref().inner().signer()
    }
}

#[cfg(test)]
mod test {
    use ethers::abi::Address;
    use futures::stream::FuturesUnordered;
    // use hex::FromHex;

    use crate::{
        library::{
            app::AppData,
            util::{to_hex_prefix, PendingTransactionExt},
        },
    };

    #[ignore]
    #[tokio::test]
    async fn send_many_transactions() -> eyre::Result<()> {
        let app: &'static AppData = AppData::init_for_test(None, false).await?;

        let results = FuturesUnordered::new();
        for i in 0..100 {
            results.push(
                dbg!(
                    app.client
                        .edit_server_info("127.0.0.1:3000".to_string())
                        .send()
                        .await
                )?
                .with_client(&app.client), // .confirmations(0)
                                           // .await,
            );
            // tokio::time::sleep(Duration::from_millis(1000)).await;
        }

        // while let Some(result) = results.next().await {
        //     dbg!(result);
        // }

        Ok(())
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
