use ethers::{abi::Address, types::U256};

mod generated;
pub use generated::*;

// (bool, H160, String, U256, U256, U256)
// <exists>,<author>,<name>,<price>,<length>,<duration>
pub struct SongInfo {
    pub exists: bool,
    pub author: Address,
    pub name: String,
    pub price: U256,
    pub len: U256,
    pub duration: U256,
}

impl From<(bool, Address, String, U256, U256, U256)> for SongInfo {
    fn from(value: (bool, Address, String, U256, U256, U256)) -> Self {
        Self {
            exists: value.0,
            author: value.1,
            name: value.2,
            price: value.3,
            len: value.4,
            duration: value.5,
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
