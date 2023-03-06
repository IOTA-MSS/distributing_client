pub use tangle_tunes_abi::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod tangle_tunes_abi {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_chunk\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"check_chunk\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"chunks_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_desc\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"create_user\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"delete_user\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"deposit\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"distribute\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"distributions\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"exists\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_desc\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"edit_description\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_price\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"edit_price\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_server\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"edit_server_info\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_distributor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"gen_distribution_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_author\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"gen_song_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_distributor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"get_chunk\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"get_rand_distributor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_songs\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"song_id\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"song_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"author_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"price\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"length\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"duration\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Song_listing[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_validator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"manage_validators\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"song_list\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"song_list_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"songs\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"exists\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"author\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"price\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"length\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"duration\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"undistribute\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_author\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_price\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_length\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_duration\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_chunks\",\n        \"type\": \"bytes32[]\"\n      }\n    ],\n    \"name\": \"upload_song\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"users\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"exists\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"username\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"description\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"server\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balance\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"is_validator\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"data\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct L1Address\",\n        \"name\": \"_target\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n";
    ///The parsed JSON ABI of the contract.
    pub static TANGLETUNESABI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct TangleTunesAbi<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TangleTunesAbi<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TangleTunesAbi<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TangleTunesAbi<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TangleTunesAbi<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(TangleTunesAbi)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TangleTunesAbi<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TANGLETUNESABI_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `check_chunk` (0x445475ff) function
        pub fn check_chunk(
            &self,
            song: [u8; 32],
            index: ::ethers::core::types::U256,
            chunk: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([68, 84, 117, 255], (song, index, chunk))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chunks_length` (0x6a604ceb) function
        pub fn chunks_length(
            &self,
            song: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 96, 76, 235], song)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `create_user` (0x2a2a69fd) function
        pub fn create_user(
            &self,
            name: ::std::string::String,
            desc: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 42, 105, 253], (name, desc))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delete_user` (0xe4282510) function
        pub fn delete_user(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 40, 37, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0xd0e30db0) function
        pub fn deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0x169ecc67) function
        pub fn distribute(
            &self,
            song: [u8; 32],
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 158, 204, 103], (song, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributions` (0xdd0e6215) function
        pub fn distributions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([221, 14, 98, 21], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edit_description` (0xceba7341) function
        pub fn edit_description(
            &self,
            desc: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 186, 115, 65], desc)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edit_price` (0xa5642b14) function
        pub fn edit_price(
            &self,
            song: [u8; 32],
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 100, 43, 20], (song, price))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `edit_server_info` (0x69587034) function
        pub fn edit_server_info(
            &self,
            server: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 88, 112, 52], server)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gen_distribution_id` (0x10c1396d) function
        pub fn gen_distribution_id(
            &self,
            song: [u8; 32],
            distributor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([16, 193, 57, 109], (song, distributor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gen_song_id` (0x1ea09895) function
        pub fn gen_song_id(
            &self,
            name: ::std::string::String,
            author: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([30, 160, 152, 149], (name, author))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_chunk` (0xd5f09957) function
        pub fn get_chunk(
            &self,
            song: [u8; 32],
            index: ::ethers::core::types::U256,
            distributor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 240, 153, 87], (song, index, distributor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_rand_distributor` (0xdddf41b3) function
        pub fn get_rand_distributor(
            &self,
            song: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([221, 223, 65, 179], song)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_songs` (0x99a7cd37) function
        pub fn get_songs(
            &self,
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<SongListing>,
        > {
            self.0
                .method_hash([153, 167, 205, 55], (index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manage_validators` (0x249c05fa) function
        pub fn manage_validators(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 156, 5, 250], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `song_list` (0x5c348f6d) function
        pub fn song_list(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([92, 52, 143, 109], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `song_list_length` (0x2f516ee8) function
        pub fn song_list_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 81, 110, 232], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `songs` (0x3095ff4e) function
        pub fn songs(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::Address,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([48, 149, 255, 78], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `undistribute` (0x5b0ec426) function
        pub fn undistribute(
            &self,
            song: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 14, 196, 38], song)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upload_song` (0x3d9debbe) function
        pub fn upload_song(
            &self,
            author: ::ethers::core::types::Address,
            name: ::std::string::String,
            price: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
            duration: ::ethers::core::types::U256,
            chunks: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [61, 157, 235, 190],
                    (author, name, price, length, duration, chunks),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `users` (0xa87430ba) function
        pub fn users(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::std::string::String,
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                bool,
            ),
        > {
            self.0
                .method_hash([168, 116, 48, 186], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x7a524970) function
        pub fn withdraw_with_amount(
            &self,
            amount: ::ethers::core::types::U256,
            target: L1Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 82, 73, 112], (amount, target))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TangleTunesAbi<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `check_chunk` function with signature `check_chunk(bytes32,uint256,bytes32)` and selector `0x445475ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "check_chunk", abi = "check_chunk(bytes32,uint256,bytes32)")]
    pub struct CheckChunkCall {
        pub song: [u8; 32],
        pub index: ::ethers::core::types::U256,
        pub chunk: [u8; 32],
    }
    ///Container type for all input parameters for the `chunks_length` function with signature `chunks_length(bytes32)` and selector `0x6a604ceb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "chunks_length", abi = "chunks_length(bytes32)")]
    pub struct ChunksLengthCall {
        pub song: [u8; 32],
    }
    ///Container type for all input parameters for the `create_user` function with signature `create_user(string,string)` and selector `0x2a2a69fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "create_user", abi = "create_user(string,string)")]
    pub struct CreateUserCall {
        pub name: ::std::string::String,
        pub desc: ::std::string::String,
    }
    ///Container type for all input parameters for the `delete_user` function with signature `delete_user()` and selector `0xe4282510`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "delete_user", abi = "delete_user()")]
    pub struct DeleteUserCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `0xd0e30db0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    ///Container type for all input parameters for the `distribute` function with signature `distribute(bytes32,uint256)` and selector `0x169ecc67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "distribute", abi = "distribute(bytes32,uint256)")]
    pub struct DistributeCall {
        pub song: [u8; 32],
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `distributions` function with signature `distributions(bytes32)` and selector `0xdd0e6215`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "distributions", abi = "distributions(bytes32)")]
    pub struct DistributionsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `edit_description` function with signature `edit_description(string)` and selector `0xceba7341`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "edit_description", abi = "edit_description(string)")]
    pub struct EditDescriptionCall {
        pub desc: ::std::string::String,
    }
    ///Container type for all input parameters for the `edit_price` function with signature `edit_price(bytes32,uint256)` and selector `0xa5642b14`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "edit_price", abi = "edit_price(bytes32,uint256)")]
    pub struct EditPriceCall {
        pub song: [u8; 32],
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `edit_server_info` function with signature `edit_server_info(string)` and selector `0x69587034`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "edit_server_info", abi = "edit_server_info(string)")]
    pub struct EditServerInfoCall {
        pub server: ::std::string::String,
    }
    ///Container type for all input parameters for the `gen_distribution_id` function with signature `gen_distribution_id(bytes32,address)` and selector `0x10c1396d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "gen_distribution_id",
        abi = "gen_distribution_id(bytes32,address)"
    )]
    pub struct GenDistributionIdCall {
        pub song: [u8; 32],
        pub distributor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `gen_song_id` function with signature `gen_song_id(string,address)` and selector `0x1ea09895`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gen_song_id", abi = "gen_song_id(string,address)")]
    pub struct GenSongIdCall {
        pub name: ::std::string::String,
        pub author: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_chunk` function with signature `get_chunk(bytes32,uint256,address)` and selector `0xd5f09957`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "get_chunk", abi = "get_chunk(bytes32,uint256,address)")]
    pub struct GetChunkCall {
        pub song: [u8; 32],
        pub index: ::ethers::core::types::U256,
        pub distributor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_rand_distributor` function with signature `get_rand_distributor(bytes32)` and selector `0xdddf41b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "get_rand_distributor", abi = "get_rand_distributor(bytes32)")]
    pub struct GetRandDistributorCall {
        pub song: [u8; 32],
    }
    ///Container type for all input parameters for the `get_songs` function with signature `get_songs(uint256,uint256)` and selector `0x99a7cd37`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "get_songs", abi = "get_songs(uint256,uint256)")]
    pub struct GetSongsCall {
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `manage_validators` function with signature `manage_validators(address)` and selector `0x249c05fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "manage_validators", abi = "manage_validators(address)")]
    pub struct ManageValidatorsCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `song_list` function with signature `song_list(uint256)` and selector `0x5c348f6d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "song_list", abi = "song_list(uint256)")]
    pub struct SongListCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `song_list_length` function with signature `song_list_length()` and selector `0x2f516ee8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "song_list_length", abi = "song_list_length()")]
    pub struct SongListLengthCall;
    ///Container type for all input parameters for the `songs` function with signature `songs(bytes32)` and selector `0x3095ff4e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "songs", abi = "songs(bytes32)")]
    pub struct SongsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `undistribute` function with signature `undistribute(bytes32)` and selector `0x5b0ec426`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "undistribute", abi = "undistribute(bytes32)")]
    pub struct UndistributeCall {
        pub song: [u8; 32],
    }
    ///Container type for all input parameters for the `upload_song` function with signature `upload_song(address,string,uint256,uint256,uint256,bytes32[])` and selector `0x3d9debbe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "upload_song",
        abi = "upload_song(address,string,uint256,uint256,uint256,bytes32[])"
    )]
    pub struct UploadSongCall {
        pub author: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
        pub duration: ::ethers::core::types::U256,
        pub chunks: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `users` function with signature `users(address)` and selector `0xa87430ba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "users", abi = "users(address)")]
    pub struct UsersCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,(bytes))` and selector `0x7a524970`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,(bytes))")]
    pub struct WithdrawWithAmountCall {
        pub amount: ::ethers::core::types::U256,
        pub target: L1Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TangleTunesAbiCalls {
        CheckChunk(CheckChunkCall),
        ChunksLength(ChunksLengthCall),
        CreateUser(CreateUserCall),
        DeleteUser(DeleteUserCall),
        Deposit(DepositCall),
        Distribute(DistributeCall),
        Distributions(DistributionsCall),
        EditDescription(EditDescriptionCall),
        EditPrice(EditPriceCall),
        EditServerInfo(EditServerInfoCall),
        GenDistributionId(GenDistributionIdCall),
        GenSongId(GenSongIdCall),
        GetChunk(GetChunkCall),
        GetRandDistributor(GetRandDistributorCall),
        GetSongs(GetSongsCall),
        ManageValidators(ManageValidatorsCall),
        SongList(SongListCall),
        SongListLength(SongListLengthCall),
        Songs(SongsCall),
        Undistribute(UndistributeCall),
        UploadSong(UploadSongCall),
        Users(UsersCall),
        Withdraw(WithdrawCall),
        WithdrawWithAmount(WithdrawWithAmountCall),
    }
    impl ::ethers::core::abi::AbiDecode for TangleTunesAbiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CheckChunkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckChunk(decoded));
            }
            if let Ok(decoded)
                = <ChunksLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChunksLength(decoded));
            }
            if let Ok(decoded)
                = <CreateUserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateUser(decoded));
            }
            if let Ok(decoded)
                = <DeleteUserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeleteUser(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <DistributeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute(decoded));
            }
            if let Ok(decoded)
                = <DistributionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distributions(decoded));
            }
            if let Ok(decoded)
                = <EditDescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EditDescription(decoded));
            }
            if let Ok(decoded)
                = <EditPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EditPrice(decoded));
            }
            if let Ok(decoded)
                = <EditServerInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EditServerInfo(decoded));
            }
            if let Ok(decoded)
                = <GenDistributionIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GenDistributionId(decoded));
            }
            if let Ok(decoded)
                = <GenSongIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GenSongId(decoded));
            }
            if let Ok(decoded)
                = <GetChunkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChunk(decoded));
            }
            if let Ok(decoded)
                = <GetRandDistributorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRandDistributor(decoded));
            }
            if let Ok(decoded)
                = <GetSongsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSongs(decoded));
            }
            if let Ok(decoded)
                = <ManageValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ManageValidators(decoded));
            }
            if let Ok(decoded)
                = <SongListCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SongList(decoded));
            }
            if let Ok(decoded)
                = <SongListLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SongListLength(decoded));
            }
            if let Ok(decoded)
                = <SongsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Songs(decoded));
            }
            if let Ok(decoded)
                = <UndistributeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Undistribute(decoded));
            }
            if let Ok(decoded)
                = <UploadSongCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UploadSong(decoded));
            }
            if let Ok(decoded)
                = <UsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Users(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <WithdrawWithAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawWithAmount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TangleTunesAbiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckChunk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChunksLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeleteUser(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Distribute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distributions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EditDescription(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EditPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EditServerInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenDistributionId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GenSongId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChunk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRandDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSongs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ManageValidators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SongList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SongListLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Songs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Undistribute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UploadSong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Users(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawWithAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TangleTunesAbiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckChunk(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChunksLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distribute(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distributions(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditDescription(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditServerInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenDistributionId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenSongId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChunk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRandDistributor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSongs(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManageValidators(element) => ::core::fmt::Display::fmt(element, f),
                Self::SongList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SongListLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Songs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undistribute(element) => ::core::fmt::Display::fmt(element, f),
                Self::UploadSong(element) => ::core::fmt::Display::fmt(element, f),
                Self::Users(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawWithAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CheckChunkCall> for TangleTunesAbiCalls {
        fn from(value: CheckChunkCall) -> Self {
            Self::CheckChunk(value)
        }
    }
    impl ::core::convert::From<ChunksLengthCall> for TangleTunesAbiCalls {
        fn from(value: ChunksLengthCall) -> Self {
            Self::ChunksLength(value)
        }
    }
    impl ::core::convert::From<CreateUserCall> for TangleTunesAbiCalls {
        fn from(value: CreateUserCall) -> Self {
            Self::CreateUser(value)
        }
    }
    impl ::core::convert::From<DeleteUserCall> for TangleTunesAbiCalls {
        fn from(value: DeleteUserCall) -> Self {
            Self::DeleteUser(value)
        }
    }
    impl ::core::convert::From<DepositCall> for TangleTunesAbiCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DistributeCall> for TangleTunesAbiCalls {
        fn from(value: DistributeCall) -> Self {
            Self::Distribute(value)
        }
    }
    impl ::core::convert::From<DistributionsCall> for TangleTunesAbiCalls {
        fn from(value: DistributionsCall) -> Self {
            Self::Distributions(value)
        }
    }
    impl ::core::convert::From<EditDescriptionCall> for TangleTunesAbiCalls {
        fn from(value: EditDescriptionCall) -> Self {
            Self::EditDescription(value)
        }
    }
    impl ::core::convert::From<EditPriceCall> for TangleTunesAbiCalls {
        fn from(value: EditPriceCall) -> Self {
            Self::EditPrice(value)
        }
    }
    impl ::core::convert::From<EditServerInfoCall> for TangleTunesAbiCalls {
        fn from(value: EditServerInfoCall) -> Self {
            Self::EditServerInfo(value)
        }
    }
    impl ::core::convert::From<GenDistributionIdCall> for TangleTunesAbiCalls {
        fn from(value: GenDistributionIdCall) -> Self {
            Self::GenDistributionId(value)
        }
    }
    impl ::core::convert::From<GenSongIdCall> for TangleTunesAbiCalls {
        fn from(value: GenSongIdCall) -> Self {
            Self::GenSongId(value)
        }
    }
    impl ::core::convert::From<GetChunkCall> for TangleTunesAbiCalls {
        fn from(value: GetChunkCall) -> Self {
            Self::GetChunk(value)
        }
    }
    impl ::core::convert::From<GetRandDistributorCall> for TangleTunesAbiCalls {
        fn from(value: GetRandDistributorCall) -> Self {
            Self::GetRandDistributor(value)
        }
    }
    impl ::core::convert::From<GetSongsCall> for TangleTunesAbiCalls {
        fn from(value: GetSongsCall) -> Self {
            Self::GetSongs(value)
        }
    }
    impl ::core::convert::From<ManageValidatorsCall> for TangleTunesAbiCalls {
        fn from(value: ManageValidatorsCall) -> Self {
            Self::ManageValidators(value)
        }
    }
    impl ::core::convert::From<SongListCall> for TangleTunesAbiCalls {
        fn from(value: SongListCall) -> Self {
            Self::SongList(value)
        }
    }
    impl ::core::convert::From<SongListLengthCall> for TangleTunesAbiCalls {
        fn from(value: SongListLengthCall) -> Self {
            Self::SongListLength(value)
        }
    }
    impl ::core::convert::From<SongsCall> for TangleTunesAbiCalls {
        fn from(value: SongsCall) -> Self {
            Self::Songs(value)
        }
    }
    impl ::core::convert::From<UndistributeCall> for TangleTunesAbiCalls {
        fn from(value: UndistributeCall) -> Self {
            Self::Undistribute(value)
        }
    }
    impl ::core::convert::From<UploadSongCall> for TangleTunesAbiCalls {
        fn from(value: UploadSongCall) -> Self {
            Self::UploadSong(value)
        }
    }
    impl ::core::convert::From<UsersCall> for TangleTunesAbiCalls {
        fn from(value: UsersCall) -> Self {
            Self::Users(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for TangleTunesAbiCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawWithAmountCall> for TangleTunesAbiCalls {
        fn from(value: WithdrawWithAmountCall) -> Self {
            Self::WithdrawWithAmount(value)
        }
    }
    ///Container type for all return fields from the `check_chunk` function with signature `check_chunk(bytes32,uint256,bytes32)` and selector `0x445475ff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckChunkReturn(pub bool);
    ///Container type for all return fields from the `chunks_length` function with signature `chunks_length(bytes32)` and selector `0x6a604ceb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ChunksLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `distributions` function with signature `distributions(bytes32)` and selector `0xdd0e6215`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DistributionsReturn {
        pub exists: bool,
        pub index: ::ethers::core::types::U256,
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `gen_distribution_id` function with signature `gen_distribution_id(bytes32,address)` and selector `0x10c1396d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GenDistributionIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `gen_song_id` function with signature `gen_song_id(string,address)` and selector `0x1ea09895`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GenSongIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `get_rand_distributor` function with signature `get_rand_distributor(bytes32)` and selector `0xdddf41b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRandDistributorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `get_songs` function with signature `get_songs(uint256,uint256)` and selector `0x99a7cd37`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSongsReturn(pub ::std::vec::Vec<SongListing>);
    ///Container type for all return fields from the `song_list` function with signature `song_list(uint256)` and selector `0x5c348f6d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SongListReturn(pub [u8; 32]);
    ///Container type for all return fields from the `song_list_length` function with signature `song_list_length()` and selector `0x2f516ee8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SongListLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `songs` function with signature `songs(bytes32)` and selector `0x3095ff4e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SongsReturn {
        pub exists: bool,
        pub author: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
        pub duration: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `users` function with signature `users(address)` and selector `0xa87430ba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UsersReturn {
        pub exists: bool,
        pub username: ::std::string::String,
        pub description: ::std::string::String,
        pub server: ::std::string::String,
        pub balance: ::ethers::core::types::U256,
        pub is_validator: bool,
    }
    ///`L1Address(bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct L1Address {
        pub data: ::ethers::core::types::Bytes,
    }
    ///`SongListing(bytes32,string,string,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SongListing {
        pub song_id: [u8; 32],
        pub song_name: ::std::string::String,
        pub author_name: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
        pub duration: ::ethers::core::types::U256,
    }
}
