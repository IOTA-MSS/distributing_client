use ethers::{
    abi::Address,
    types::{H160, U256},
};

mod generated;
pub use generated::*;

// (bool, H160, String, U256, U256, U256)
// <exists>,<author>,<name>,<price>,<length>,<duration>
// (bool,   address,  address,     address,    string, uint,  uint,    uint,      uint)
// <exists>,<author>,<rightholder>,<validator>,<name>,<price>,<length>,<duration>,<distributors>
pub struct SongInfo {
    pub exists: bool,
    pub author: Address,
    pub rightholder: Address,
    pub validator: Address,
    pub name: String,
    pub price: U256,
    pub len: U256,
    pub duration: U256,
    pub distributors: U256,
}

impl SongInfo {
    pub fn total_price(&self) -> U256 {
        todo!()
        // self.price * div_ceil_u256(self.len, BYTES_PER_CHUNK_USIZE.into())
    }
}

impl From<(bool, H160, H160, H160, String, U256, U256, U256, U256)> for SongInfo {
    fn from(value: (bool, H160, H160, H160, String, U256, U256, U256, U256)) -> Self {
        Self {
            exists: value.0,
            author: value.1,
            rightholder: value.2,
            validator: value.3,
            name: value.4,
            price: value.5,
            len: value.6,
            duration: value.7,
            distributors: value.8,
        }
    }
}

// (bool, String, String, String, U256, bool)
pub struct UserInfo {
    pub exists: bool,
    pub username: String,
    pub description: String,
    pub server: String,
    pub balance: U256,
    pub is_validator: bool,
}

impl From<(bool, String, String, String, U256, bool)> for UserInfo {
    fn from(value: (bool, String, String, String, U256, bool)) -> Self {
        Self {
            exists: value.0,
            username: value.1,
            description: value.2,
            server: value.3,
            balance: value.4,
            is_validator: value.5,
        }
    }
}
