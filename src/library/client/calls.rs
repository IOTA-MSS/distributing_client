use super::{TTCall, TTMiddleWare, TangleTunesClient, WEI_PER_IOTA};
use crate::library::{
    abi::SongInfo,
    util::{SongId, TTCallExt},
};
use ethers::{
    prelude::ContractError,
    types::{Address, U256},
};
use itertools::Itertools;

pub type TTCallError = ContractError<TTMiddleWare>;

impl TangleTunesClient {
    //------------------------------------------------------------------------------------------------
    //  Pure
    //------------------------------------------------------------------------------------------------

    /// Get a random distributor from the smart contract
    pub async fn call_get_rand_distributor(
        &self,
        song: SongId,
    ) -> Result<(Address, String), TTCallError> {
        Ok(self
            .abi_client
            .get_rand_distributor(song.into())
            .set_defaults()
            .await?)
    }

    pub async fn call_get_song_info(&self, song_id: SongId) -> Result<SongInfo, TTCallError> {
        Ok(self
            .abi_client
            .songs(song_id.into())
            .set_defaults()
            .await?
            .into())
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

    pub fn distribute_call(&self, song_id: SongId, fee: u32) -> TTCall<()> {
        self.abi_client
            .distribute(song_id.into(), fee.into())
            .set_defaults()
    }

    pub fn edit_server_info_call(&self, address: String) -> TTCall<()> {
        self.abi_client.edit_server_info(address).set_defaults()
    }

    pub fn undistribute_call(&self, song_id: SongId) -> TTCall<()> {
        self.abi_client.undistribute(song_id.into()).set_defaults()
    }

    pub fn deposit_call(&self, iota: u128) -> TTCall<()> {
        self
            .abi_client
            .deposit()
            .value(iota * WEI_PER_IOTA)
            .set_defaults()
    }

    pub fn withdraw_call(&self, iota: u128) -> TTCall<()> {
        self
            .abi_client
            .withdraw((iota * WEI_PER_IOTA).into())
            .set_defaults()
    }

    pub fn delete_user_call(&self) -> TTCall<()> {
        self
            .abi_client
            .delete_user()
            .set_defaults()
    }

    pub fn create_user_call(
        &self,
        name: String,
        description: String,
    ) -> TTCall<()> {
        self
            .abi_client
            .create_user(name, description)
            .set_defaults()
    }
}
