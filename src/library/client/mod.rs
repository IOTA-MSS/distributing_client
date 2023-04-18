mod calls;
mod download;

pub const GAS: usize = 1_000_000;
pub const WEI_PER_IOTA: u128 = 1_000_000_000_000;

use super::{
    abi::{GetChunksCall, TangleTunesAbi},
    app::App,
    crypto::Wallet,
    util::{TTCallExt, TransactionReceiptExt},
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
use itertools::Itertools;
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

    pub async fn l2_balance(&self) -> eyre::Result<U256> {
        Ok(self
            .abi_client
            .client_ref()
            .get_balance(self.wallet_address(), None)
            .await?)
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

    pub async fn send_raw_tx(
        &self,
        tx: Bytes,
    ) -> Result<PendingTransaction<'_, Http>, eyre::Report> {
        Ok(self
            .abi_client
            .deref()
            .client_ref()
            .inner()
            .inner()
            .send_raw_transaction(tx)
            .await?)
    }

    pub async fn get_local_nonce(&self) -> eyre::Result<U256> {
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

    /// Attempts to distribute the given songs in a single transaction, while checking that we
    /// are not actually distributing the songs already. It will only distribute those songs that
    /// are not yet distributed.
    pub async fn try_distribute(&'static self, songs: &[(SongId, U256)]) -> eyre::Result<()> {
        // Check which songs we are already distributing
        let distributions = self
            .abi_client
            .is_distributing(
                songs
                    .iter()
                    .map(|(song, _fee)| (*song).into())
                    .collect_vec(),
                self.wallet_address(),
            )
            .await?;

        // Only register for the songs where we are not yet distributing
        let songs: Vec<(SongId, U256)> = songs
            .iter()
            .zip(distributions)
            .filter_map(|(song, distributing)| (!distributing).then_some(*song))
            .collect();

        if songs.is_empty() {
            return Ok(());
        }

        // And finally start distributing these songs
        self.distribute_call(songs.clone())
            .await?
            .send()
            .await?
            .await?
            .status_is_ok(&format!("Could not register song with ids {songs:?}"))?;

        Ok(())
    }

    /// Attempts to undistribute the given songs in a single transaction, while checking that we
    /// are actually distributing the songs. It will only undistribute those songs that
    /// are distributed.
    pub async fn try_undistribute(&'static self, songs: &Vec<SongId>) -> eyre::Result<()> {
        println!("Deregistering songs {songs:?} on the smart-contract..");

        // Check which songs we are actually distributing
        let distributions = self
            .abi_client
            .is_distributing(
                songs.iter().map(|song| (*song).into()).collect_vec(),
                self.wallet_address(),
            )
            .await?;

        // Only deregister for the songs where we are distributing
        let songs: Vec<SongId> = songs
            .iter()
            .zip(distributions)
            .filter_map(|(song, distribution)| distribution.then_some(*song))
            .collect();

        if songs.is_empty() {
            return Ok(());
        }

        // And deregister for those filtered songs
        self.undistribute_call(songs.clone())
            .await?
            .send()
            .await?
            .await?
            .status_is_ok(&format!("Could not deregister song with ids {songs:?}"))?;

        Ok(())
    }

    pub(crate) async fn reset_nonce(&self, app: &App) -> eyre::Result<()> {
        self.edit_server_info_call(app.server_address.to_string())
            .set_defaults()
            .send()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::library::app::App;

    #[ignore]
    #[tokio::test]
    async fn get_songs() -> eyre::Result<()> {
        let app: &'static App = App::init_for_test(None, false).await?;
        dbg!(app.client.get_song_ids_from_index(0).await?);
        Ok(())
    }
}
