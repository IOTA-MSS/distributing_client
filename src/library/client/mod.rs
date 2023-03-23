mod calls;
mod download;

const GAS: usize = 1_000_000;
const WEI_PER_IOTA: u128 = 1_000_000_000_000;

use super::{
    abi::{GetChunksCall, TangleTunesAbi},
    crypto::Wallet,
    util::TTCallExt,
};
use crate::library::util::SongId;
use ethers::{
    abi::AbiDecode,
    prelude::*,
    signers::LocalWallet,
    types::{transaction::eip2718::TypedTransaction, Address},
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
    pub fn create_pending_tx(
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

    /// Get the song metadata from the given index (inclusive)
    pub async fn get_song_ids_from_index(
        &self,
        index: usize,
    ) -> eyre::Result<Vec<(usize, SongId)>> {
        let last_index = self.call_song_list_length().await?.as_usize();

        if index < last_index {
            let mut song_ids = Vec::with_capacity(last_index - index);
            for i in index..last_index {
                let id = self.abi_client.song_list(i.into()).set_defaults().await?;
                song_ids.push((i, id.into()));
            }
            Ok(song_ids)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn create_get_chunks_signed_rlp(
        &self,
        song_id: SongId,
        from: usize,
        amount: usize,
        distributor: Address,
    ) -> eyre::Result<Bytes> {
        let tx = {
            let mut tx = self
                .abi_client
                .get_chunks(song_id.into(), from.into(), amount.into(), distributor)
                .legacy()
                .tx;
            tx.set_gas(GAS);
            tx.set_nonce(self.abi_client.client_ref().next());
            tx.set_gas_price(1);
            tx
        };
        println!("Tx nonce: {}", tx.nonce().unwrap());
        let signature = self.wallet().sign_transaction_sync(&tx);
        Ok(tx.rlp_signed(&signature))
    }

    pub fn decode_get_chunks_params(&self, tx_rlp: &[u8]) -> eyre::Result<GetChunksCall> {
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

    pub async fn get_nonce(&self) -> eyre::Result<U256> {
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

    use crate::library::{
        app::AppData,
        util::{to_hex_prefix, PendingTransactionExt},
    };

    #[ignore]
    #[tokio::test]
    async fn get_songs_test() -> eyre::Result<()> {
        let app: &'static AppData = AppData::init_for_test(None, false).await?;
        dbg!(app.client.get_song_ids_from_index(0).await?);
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    async fn send_many_transactions() -> eyre::Result<()> {
        let app: &'static AppData = AppData::init_for_test(None, false).await?;

        let results = FuturesUnordered::new();
        for _i in 0..100 {
            results.push(
                dbg!(
                    app.client
                        .edit_server_info_call("127.0.0.1:3000".to_string())
                        .send()
                        .await
                )?
                .with_client(&app.client), // .confirmations(0)
                                           // .await,
            );
            // tokio::time::sleep(Duration::from_millis(1000)).await;
        }

        Ok(())
    }

    #[allow(unused)]
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
