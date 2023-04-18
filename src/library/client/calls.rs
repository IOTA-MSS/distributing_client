use std::future::IntoFuture;

use super::{TTCall, TTMiddleWare, TangleTunesClient, WEI_PER_IOTA};
use crate::library::{
    abi::{DistributionListing, SongInfo, UserInfo},
    util::{SongId, TTCallExt},
};
use ethers::{
    prelude::ContractError,
    types::{Address, U256},
};
use futures::future::join;
use itertools::Itertools;

pub type TTCallError = ContractError<TTMiddleWare>;

impl TangleTunesClient {
    //------------------------------------------------------------------------------------------------
    //  Pure
    //------------------------------------------------------------------------------------------------

    /// Get a random distributor from the smart contract
    pub async fn get_rand_distributor(
        &self,
        song: SongId,
    ) -> Result<DistributionListing, TTCallError> {
        self.abi_client
            .get_rand_distributor(song.into(), rand::random::<u128>().into())
            .set_defaults()
            .await
    }

    pub async fn get_song_info(&self, song_id: SongId) -> Result<SongInfo, TTCallError> {
        Ok(self
            .abi_client
            .songs(song_id.into())
            .set_defaults()
            .await?
            .into())
    }

    pub async fn get_user_info(&self, address: Address) -> Result<UserInfo, TTCallError> {
        Ok(self.abi_client.users(address).set_defaults().await?.into())
    }

    pub async fn call_song_list_length(&self) -> Result<U256, TTCallError> {
        self.abi_client.song_list_length().set_defaults().await
    }

    pub async fn call_check_chunks(
        &self,
        song_id: SongId,
        index: usize,
        chunks: usize,
    ) -> Result<Vec<SongId>, TTCallError> {
        Ok(self
            .abi_client
            .check_chunks(song_id.into(), index.into(), chunks.into())
            .set_defaults()
            .await?
            .into_iter()
            .map(Into::into)
            .collect_vec())
    }

    //------------------------------------------------------------------------------------------------
    //  Non-pure
    //------------------------------------------------------------------------------------------------

    pub async fn distribute_call(&self, songs: Vec<(SongId, U256)>) -> eyre::Result<TTCall<()>> {
        let mut song_ids = Vec::with_capacity(songs.len());
        let mut fees = Vec::with_capacity(songs.len());
        for (song_id, fee) in songs {
            song_ids.push(song_id.into());
            fees.push(fee * U256::from(WEI_PER_IOTA));
        }

        let indexes = join(
            self.abi_client
                .find_dist_indexes(song_ids.clone(), self.wallet_address())
                .into_future(),
            self.abi_client
                .find_insert_indexes(song_ids.clone(), fees.clone())
                .into_future(),
        )
        .await;
        let (dist_indexes, insert_indexes) = (indexes.0?, indexes.1?);

        Ok(self
            .abi_client
            .distribute(song_ids, fees, dist_indexes, insert_indexes)
            .set_defaults())
    }

    pub async fn undistribute_call(&self, song_id: Vec<SongId>) -> eyre::Result<TTCall<()>> {
        let song_ids: Vec<[u8; 32]> = song_id.into_iter().map(Into::into).collect();

        let indexes = self
            .abi_client
            .find_dist_indexes(song_ids.clone(), self.wallet_address())
            .await?;

        Ok(self
            .abi_client
            .undistribute(song_ids, indexes)
            .set_defaults())
    }

    pub fn edit_server_info_call(&self, address: String) -> TTCall<()> {
        self.abi_client.edit_server_info(address).set_defaults()
    }

    pub fn deposit_call(&self, iota: u128) -> TTCall<()> {
        self.abi_client
            .deposit()
            .value(iota * WEI_PER_IOTA)
            .set_defaults()
    }

    pub fn withdraw_call(&self, iota: u128) -> TTCall<()> {
        self.abi_client
            .withdraw_to_chain((iota * WEI_PER_IOTA).into())
            .set_defaults()
    }

    pub fn delete_user_call(&self) -> TTCall<()> {
        self.abi_client.delete_user().set_defaults()
    }

    pub fn create_user_call(&self, name: String, description: String) -> TTCall<()> {
        self.abi_client
            .create_user(name, description)
            .set_defaults()
    }
}

#[cfg(test)]
mod test {
    use crate::library::{app::App, util::try_from_hex_prefix};

    #[tokio::test]
    #[ignore]
    async fn insert_indexes() -> eyre::Result<()> {
        let app: &'static App = App::init_for_test(None, false).await?;

        dbg!(
            app.client
                .abi_client
                .find_insert_indexes(
                    vec![
                        try_from_hex_prefix(
                            "0x20b1967566b72692dbaa773f79c972a352068d4df19fc3eb04ab83bd2c3f716d"
                        )?,
                        try_from_hex_prefix(
                            "0x752d9170532899a0b362ac3cbff4e1fb3a609851927203e64339931ed0ddfe42"
                        )?,
                    ],
                    vec![100.into(), 100.into()]
                )
                .await?
        );

        Ok(())
    }
}
