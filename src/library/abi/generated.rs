pub use tangle_tunes_abi::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod tangle_tunes_abi {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"check_chunks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"\",\n        \"type\": \"bytes32[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"chunks_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_desc\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"create_user\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"delete_song\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"delete_user\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"deposit\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_songs\",\n        \"type\": \"bytes32[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"_fees\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_dist_index_addresses\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_insert_index_addresses\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"distribute\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"distributions\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"fee\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"next_distributor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_desc\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"edit_description\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_price\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"edit_price\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_server\",\n        \"type\": \"string\"\n      }\n    ],\n    \"name\": \"edit_server_info\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_songs\",\n        \"type\": \"bytes32[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_dist_addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"find_dist_indexes\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_songs\",\n        \"type\": \"bytes32[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"_fees\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"find_insert_indexes\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_distributor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"gen_distribution_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_author\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"gen_song_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"get_author_of_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_author_of_song_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_author_of_songs\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"song_id\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"song_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"author_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"price\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"length\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"duration\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Song_listing[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_distributor\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"get_chunks\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_start\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_distributors\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"distributor\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"server\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fee\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Distribution_listing[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"get_distributors_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"get_holds_rights_to_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_holds_rights_to_song_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_holds_rights_to_songs\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"song_id\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"song_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"author_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"price\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"length\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"duration\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Song_listing[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"_song\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_seed\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_rand_distributor\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"distributor\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"server\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"fee\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Distribution_listing\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_songs\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"song_id\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"song_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"author_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"price\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"length\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"duration\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Song_listing[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"get_validates_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_validates_song_id\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_user\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"get_validates_songs\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"song_id\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"song_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"string\",\n            \"name\": \"author_name\",\n            \"type\": \"string\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"price\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"length\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"duration\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct TangleTunesI.Song_listing[]\",\n        \"name\": \"\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_songs\",\n        \"type\": \"bytes32[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_dist_addr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"is_distributing\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool[]\",\n        \"name\": \"\",\n        \"type\": \"bool[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_validator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"manage_validators\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"song_list\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"song_list_length\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"songs\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"exists\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"author\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"rightholder\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"validator\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"price\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"length\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"duration\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"distributors\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_songs\",\n        \"type\": \"bytes32[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"_index_addresses\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"name\": \"undistribute\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_author\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"_name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_price\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_length\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_duration\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"_chunks\",\n        \"type\": \"bytes32[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_nonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_signature\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"upload_song\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"users\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"exists\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"username\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"description\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"server\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"balance\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"is_validator\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"withdraw_to_chain\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"_amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"data\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct L1Address\",\n        \"name\": \"_target\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"withdraw_to_tangle\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n";
    ///The parsed JSON ABI of the contract.
    pub static TANGLETUNESABI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
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
            f.debug_tuple(stringify!(TangleTunesAbi))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TangleTunesAbi<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                TANGLETUNESABI_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `check_chunks` (0xdc70c749) function
        pub fn check_chunks(
            &self,
            song: [u8; 32],
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([220, 112, 199, 73], (song, index, amount))
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
        ///Calls the contract's `delete_song` (0x043c1618) function
        pub fn delete_song(
            &self,
            song: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 60, 22, 24], song)
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
        ///Calls the contract's `distribute` (0x993d33ae) function
        pub fn distribute(
            &self,
            songs: ::std::vec::Vec<[u8; 32]>,
            fees: ::std::vec::Vec<::ethers::core::types::U256>,
            dist_index_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
            insert_index_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 61, 51, 174],
                    (songs, fees, dist_index_addresses, insert_index_addresses),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributions` (0xdd0e6215) function
        pub fn distributions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::Address),
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
        ///Calls the contract's `find_dist_indexes` (0x6f9e540d) function
        pub fn find_dist_indexes(
            &self,
            songs: ::std::vec::Vec<[u8; 32]>,
            dist_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([111, 158, 84, 13], (songs, dist_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `find_insert_indexes` (0x97930cd8) function
        pub fn find_insert_indexes(
            &self,
            songs: ::std::vec::Vec<[u8; 32]>,
            fees: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([151, 147, 12, 216], (songs, fees))
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
        ///Calls the contract's `get_author_of_length` (0x2eaef7e7) function
        pub fn get_author_of_length(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 174, 247, 231], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_author_of_song_id` (0x74934e5b) function
        pub fn get_author_of_song_id(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([116, 147, 78, 91], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_author_of_songs` (0x822568b2) function
        pub fn get_author_of_songs(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SongListing>> {
            self.0
                .method_hash([130, 37, 104, 178], (user, index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_chunks` (0x9566d179) function
        pub fn get_chunks(
            &self,
            song: [u8; 32],
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
            distributor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 102, 209, 121], (song, index, amount, distributor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_distributors` (0x135adae4) function
        pub fn get_distributors(
            &self,
            song: [u8; 32],
            start: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DistributionListing>>
        {
            self.0
                .method_hash([19, 90, 218, 228], (song, start, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_distributors_length` (0x958c21eb) function
        pub fn get_distributors_length(
            &self,
            song: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([149, 140, 33, 235], song)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_holds_rights_to_length` (0xa9bec5c6) function
        pub fn get_holds_rights_to_length(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([169, 190, 197, 198], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_holds_rights_to_song_id` (0x3c3b3b9d) function
        pub fn get_holds_rights_to_song_id(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([60, 59, 59, 157], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_holds_rights_to_songs` (0x66ac6689) function
        pub fn get_holds_rights_to_songs(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SongListing>> {
            self.0
                .method_hash([102, 172, 102, 137], (user, index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_rand_distributor` (0x60a04a38) function
        pub fn get_rand_distributor(
            &self,
            song: [u8; 32],
            seed: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, DistributionListing> {
            self.0
                .method_hash([96, 160, 74, 56], (song, seed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_songs` (0x99a7cd37) function
        pub fn get_songs(
            &self,
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SongListing>> {
            self.0
                .method_hash([153, 167, 205, 55], (index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_validates_length` (0xa2c0e7a9) function
        pub fn get_validates_length(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 192, 231, 169], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_validates_song_id` (0xd81ebaa4) function
        pub fn get_validates_song_id(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([216, 30, 186, 164], (user, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_validates_songs` (0x62eae29d) function
        pub fn get_validates_songs(
            &self,
            user: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<SongListing>> {
            self.0
                .method_hash([98, 234, 226, 157], (user, index, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `is_distributing` (0xccfd2bd0) function
        pub fn is_distributing(
            &self,
            songs: ::std::vec::Vec<[u8; 32]>,
            dist_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([204, 253, 43, 208], (songs, dist_addr))
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
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
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
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([48, 149, 255, 78], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `undistribute` (0x5bbba900) function
        pub fn undistribute(
            &self,
            songs: ::std::vec::Vec<[u8; 32]>,
            index_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 187, 169, 0], (songs, index_addresses))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upload_song` (0x4e923df1) function
        pub fn upload_song(
            &self,
            author: ::ethers::core::types::Address,
            name: ::std::string::String,
            price: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
            duration: ::ethers::core::types::U256,
            chunks: ::std::vec::Vec<[u8; 32]>,
            nonce: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [78, 146, 61, 241],
                    (
                        author, name, price, length, duration, chunks, nonce, signature,
                    ),
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
        ///Calls the contract's `withdraw_to_chain` (0x15b6182e) function
        pub fn withdraw_to_chain(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 182, 24, 46], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw_to_tangle` (0x3c3b517e) function
        pub fn withdraw_to_tangle(
            &self,
            amount: u64,
            target: L1Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 59, 81, 126], (amount, target))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for TangleTunesAbi<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `check_chunks` function with signature `check_chunks(bytes32,uint256,uint256)` and selector `0xdc70c749`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "check_chunks", abi = "check_chunks(bytes32,uint256,uint256)")]
    pub struct CheckChunksCall {
        pub song: [u8; 32],
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "create_user", abi = "create_user(string,string)")]
    pub struct CreateUserCall {
        pub name: ::std::string::String,
        pub desc: ::std::string::String,
    }
    ///Container type for all input parameters for the `delete_song` function with signature `delete_song(bytes32)` and selector `0x043c1618`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "delete_song", abi = "delete_song(bytes32)")]
    pub struct DeleteSongCall {
        pub song: [u8; 32],
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
    ///Container type for all input parameters for the `distribute` function with signature `distribute(bytes32[],uint256[],address[],address[])` and selector `0x993d33ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "distribute",
        abi = "distribute(bytes32[],uint256[],address[],address[])"
    )]
    pub struct DistributeCall {
        pub songs: ::std::vec::Vec<[u8; 32]>,
        pub fees: ::std::vec::Vec<::ethers::core::types::U256>,
        pub dist_index_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        pub insert_index_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "edit_server_info", abi = "edit_server_info(string)")]
    pub struct EditServerInfoCall {
        pub server: ::std::string::String,
    }
    ///Container type for all input parameters for the `find_dist_indexes` function with signature `find_dist_indexes(bytes32[],address)` and selector `0x6f9e540d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "find_dist_indexes",
        abi = "find_dist_indexes(bytes32[],address)"
    )]
    pub struct FindDistIndexesCall {
        pub songs: ::std::vec::Vec<[u8; 32]>,
        pub dist_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `find_insert_indexes` function with signature `find_insert_indexes(bytes32[],uint256[])` and selector `0x97930cd8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "find_insert_indexes",
        abi = "find_insert_indexes(bytes32[],uint256[])"
    )]
    pub struct FindInsertIndexesCall {
        pub songs: ::std::vec::Vec<[u8; 32]>,
        pub fees: ::std::vec::Vec<::ethers::core::types::U256>,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "gen_song_id", abi = "gen_song_id(string,address)")]
    pub struct GenSongIdCall {
        pub name: ::std::string::String,
        pub author: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_author_of_length` function with signature `get_author_of_length(address)` and selector `0x2eaef7e7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "get_author_of_length", abi = "get_author_of_length(address)")]
    pub struct GetAuthorOfLengthCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_author_of_song_id` function with signature `get_author_of_song_id(address,uint256)` and selector `0x74934e5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_author_of_song_id",
        abi = "get_author_of_song_id(address,uint256)"
    )]
    pub struct GetAuthorOfSongIdCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_author_of_songs` function with signature `get_author_of_songs(address,uint256,uint256)` and selector `0x822568b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_author_of_songs",
        abi = "get_author_of_songs(address,uint256,uint256)"
    )]
    pub struct GetAuthorOfSongsCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_chunks` function with signature `get_chunks(bytes32,uint256,uint256,address)` and selector `0x9566d179`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_chunks",
        abi = "get_chunks(bytes32,uint256,uint256,address)"
    )]
    pub struct GetChunksCall {
        pub song: [u8; 32],
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub distributor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_distributors` function with signature `get_distributors(bytes32,address,uint256)` and selector `0x135adae4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_distributors",
        abi = "get_distributors(bytes32,address,uint256)"
    )]
    pub struct GetDistributorsCall {
        pub song: [u8; 32],
        pub start: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_distributors_length` function with signature `get_distributors_length(bytes32)` and selector `0x958c21eb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_distributors_length",
        abi = "get_distributors_length(bytes32)"
    )]
    pub struct GetDistributorsLengthCall {
        pub song: [u8; 32],
    }
    ///Container type for all input parameters for the `get_holds_rights_to_length` function with signature `get_holds_rights_to_length(address)` and selector `0xa9bec5c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_holds_rights_to_length",
        abi = "get_holds_rights_to_length(address)"
    )]
    pub struct GetHoldsRightsToLengthCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_holds_rights_to_song_id` function with signature `get_holds_rights_to_song_id(address,uint256)` and selector `0x3c3b3b9d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_holds_rights_to_song_id",
        abi = "get_holds_rights_to_song_id(address,uint256)"
    )]
    pub struct GetHoldsRightsToSongIdCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_holds_rights_to_songs` function with signature `get_holds_rights_to_songs(address,uint256,uint256)` and selector `0x66ac6689`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_holds_rights_to_songs",
        abi = "get_holds_rights_to_songs(address,uint256,uint256)"
    )]
    pub struct GetHoldsRightsToSongsCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_rand_distributor` function with signature `get_rand_distributor(bytes32,uint256)` and selector `0x60a04a38`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_rand_distributor",
        abi = "get_rand_distributor(bytes32,uint256)"
    )]
    pub struct GetRandDistributorCall {
        pub song: [u8; 32],
        pub seed: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "get_songs", abi = "get_songs(uint256,uint256)")]
    pub struct GetSongsCall {
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_validates_length` function with signature `get_validates_length(address)` and selector `0xa2c0e7a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "get_validates_length", abi = "get_validates_length(address)")]
    pub struct GetValidatesLengthCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `get_validates_song_id` function with signature `get_validates_song_id(address,uint256)` and selector `0xd81ebaa4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_validates_song_id",
        abi = "get_validates_song_id(address,uint256)"
    )]
    pub struct GetValidatesSongIdCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `get_validates_songs` function with signature `get_validates_songs(address,uint256,uint256)` and selector `0x62eae29d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "get_validates_songs",
        abi = "get_validates_songs(address,uint256,uint256)"
    )]
    pub struct GetValidatesSongsCall {
        pub user: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `is_distributing` function with signature `is_distributing(bytes32[],address)` and selector `0xccfd2bd0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "is_distributing", abi = "is_distributing(bytes32[],address)")]
    pub struct IsDistributingCall {
        pub songs: ::std::vec::Vec<[u8; 32]>,
        pub dist_addr: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethcall(name = "manage_validators", abi = "manage_validators(address)")]
    pub struct ManageValidatorsCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `song_list` function with signature `song_list(uint256)` and selector `0x5c348f6d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "songs", abi = "songs(bytes32)")]
    pub struct SongsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `undistribute` function with signature `undistribute(bytes32[],address[])` and selector `0x5bbba900`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "undistribute", abi = "undistribute(bytes32[],address[])")]
    pub struct UndistributeCall {
        pub songs: ::std::vec::Vec<[u8; 32]>,
        pub index_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `upload_song` function with signature `upload_song(address,string,uint256,uint256,uint256,bytes32[],uint256,bytes)` and selector `0x4e923df1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "upload_song",
        abi = "upload_song(address,string,uint256,uint256,uint256,bytes32[],uint256,bytes)"
    )]
    pub struct UploadSongCall {
        pub author: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
        pub duration: ::ethers::core::types::U256,
        pub chunks: ::std::vec::Vec<[u8; 32]>,
        pub nonce: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
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
        Hash,
    )]
    #[ethcall(name = "users", abi = "users(address)")]
    pub struct UsersCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdraw_to_chain` function with signature `withdraw_to_chain(uint256)` and selector `0x15b6182e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "withdraw_to_chain", abi = "withdraw_to_chain(uint256)")]
    pub struct WithdrawToChainCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw_to_tangle` function with signature `withdraw_to_tangle(uint64,(bytes))` and selector `0x3c3b517e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "withdraw_to_tangle",
        abi = "withdraw_to_tangle(uint64,(bytes))"
    )]
    pub struct WithdrawToTangleCall {
        pub amount: u64,
        pub target: L1Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TangleTunesAbiCalls {
        CheckChunks(CheckChunksCall),
        ChunksLength(ChunksLengthCall),
        CreateUser(CreateUserCall),
        DeleteSong(DeleteSongCall),
        DeleteUser(DeleteUserCall),
        Deposit(DepositCall),
        Distribute(DistributeCall),
        Distributions(DistributionsCall),
        EditDescription(EditDescriptionCall),
        EditPrice(EditPriceCall),
        EditServerInfo(EditServerInfoCall),
        FindDistIndexes(FindDistIndexesCall),
        FindInsertIndexes(FindInsertIndexesCall),
        GenDistributionId(GenDistributionIdCall),
        GenSongId(GenSongIdCall),
        GetAuthorOfLength(GetAuthorOfLengthCall),
        GetAuthorOfSongId(GetAuthorOfSongIdCall),
        GetAuthorOfSongs(GetAuthorOfSongsCall),
        GetChunks(GetChunksCall),
        GetDistributors(GetDistributorsCall),
        GetDistributorsLength(GetDistributorsLengthCall),
        GetHoldsRightsToLength(GetHoldsRightsToLengthCall),
        GetHoldsRightsToSongId(GetHoldsRightsToSongIdCall),
        GetHoldsRightsToSongs(GetHoldsRightsToSongsCall),
        GetRandDistributor(GetRandDistributorCall),
        GetSongs(GetSongsCall),
        GetValidatesLength(GetValidatesLengthCall),
        GetValidatesSongId(GetValidatesSongIdCall),
        GetValidatesSongs(GetValidatesSongsCall),
        IsDistributing(IsDistributingCall),
        ManageValidators(ManageValidatorsCall),
        Owner(OwnerCall),
        SongList(SongListCall),
        SongListLength(SongListLengthCall),
        Songs(SongsCall),
        Undistribute(UndistributeCall),
        UploadSong(UploadSongCall),
        Users(UsersCall),
        WithdrawToChain(WithdrawToChainCall),
        WithdrawToTangle(WithdrawToTangleCall),
    }
    impl ::ethers::core::abi::AbiDecode for TangleTunesAbiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckChunksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckChunks(decoded));
            }
            if let Ok(decoded) = <ChunksLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChunksLength(decoded));
            }
            if let Ok(decoded) = <CreateUserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateUser(decoded));
            }
            if let Ok(decoded) = <DeleteSongCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeleteSong(decoded));
            }
            if let Ok(decoded) = <DeleteUserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeleteUser(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DistributeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute(decoded));
            }
            if let Ok(decoded) = <DistributionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Distributions(decoded));
            }
            if let Ok(decoded) =
                <EditDescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EditDescription(decoded));
            }
            if let Ok(decoded) = <EditPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EditPrice(decoded));
            }
            if let Ok(decoded) =
                <EditServerInfoCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EditServerInfo(decoded));
            }
            if let Ok(decoded) =
                <FindDistIndexesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FindDistIndexes(decoded));
            }
            if let Ok(decoded) =
                <FindInsertIndexesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FindInsertIndexes(decoded));
            }
            if let Ok(decoded) =
                <GenDistributionIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GenDistributionId(decoded));
            }
            if let Ok(decoded) = <GenSongIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GenSongId(decoded));
            }
            if let Ok(decoded) =
                <GetAuthorOfLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAuthorOfLength(decoded));
            }
            if let Ok(decoded) =
                <GetAuthorOfSongIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAuthorOfSongId(decoded));
            }
            if let Ok(decoded) =
                <GetAuthorOfSongsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAuthorOfSongs(decoded));
            }
            if let Ok(decoded) = <GetChunksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChunks(decoded));
            }
            if let Ok(decoded) =
                <GetDistributorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDistributors(decoded));
            }
            if let Ok(decoded) =
                <GetDistributorsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetDistributorsLength(decoded));
            }
            if let Ok(decoded) =
                <GetHoldsRightsToLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHoldsRightsToLength(decoded));
            }
            if let Ok(decoded) =
                <GetHoldsRightsToSongIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHoldsRightsToSongId(decoded));
            }
            if let Ok(decoded) =
                <GetHoldsRightsToSongsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHoldsRightsToSongs(decoded));
            }
            if let Ok(decoded) =
                <GetRandDistributorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRandDistributor(decoded));
            }
            if let Ok(decoded) = <GetSongsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSongs(decoded));
            }
            if let Ok(decoded) =
                <GetValidatesLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetValidatesLength(decoded));
            }
            if let Ok(decoded) =
                <GetValidatesSongIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetValidatesSongId(decoded));
            }
            if let Ok(decoded) =
                <GetValidatesSongsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetValidatesSongs(decoded));
            }
            if let Ok(decoded) =
                <IsDistributingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsDistributing(decoded));
            }
            if let Ok(decoded) =
                <ManageValidatorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ManageValidators(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <SongListCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SongList(decoded));
            }
            if let Ok(decoded) =
                <SongListLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SongListLength(decoded));
            }
            if let Ok(decoded) = <SongsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Songs(decoded));
            }
            if let Ok(decoded) = <UndistributeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Undistribute(decoded));
            }
            if let Ok(decoded) = <UploadSongCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UploadSong(decoded));
            }
            if let Ok(decoded) = <UsersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Users(decoded));
            }
            if let Ok(decoded) =
                <WithdrawToChainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawToChain(decoded));
            }
            if let Ok(decoded) =
                <WithdrawToTangleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawToTangle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TangleTunesAbiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckChunks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChunksLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateUser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteSong(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeleteUser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Distribute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Distributions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EditDescription(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EditPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EditServerInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindDistIndexes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindInsertIndexes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GenDistributionId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GenSongId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAuthorOfLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAuthorOfSongId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAuthorOfSongs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetChunks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDistributors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDistributorsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHoldsRightsToLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHoldsRightsToSongId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetHoldsRightsToSongs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRandDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSongs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetValidatesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatesSongId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatesSongs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDistributing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ManageValidators(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SongList(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SongListLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Songs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Undistribute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UploadSong(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Users(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawToChain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawToTangle(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for TangleTunesAbiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckChunks(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChunksLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteSong(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeleteUser(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distribute(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distributions(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditDescription(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::EditServerInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindDistIndexes(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindInsertIndexes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenDistributionId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenSongId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAuthorOfLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAuthorOfSongId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAuthorOfSongs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChunks(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDistributors(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDistributorsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHoldsRightsToLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHoldsRightsToSongId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHoldsRightsToSongs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRandDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSongs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatesSongId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatesSongs(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDistributing(element) => ::core::fmt::Display::fmt(element, f),
                Self::ManageValidators(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SongList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SongListLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Songs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undistribute(element) => ::core::fmt::Display::fmt(element, f),
                Self::UploadSong(element) => ::core::fmt::Display::fmt(element, f),
                Self::Users(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawToChain(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawToTangle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckChunksCall> for TangleTunesAbiCalls {
        fn from(value: CheckChunksCall) -> Self {
            Self::CheckChunks(value)
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
    impl ::core::convert::From<DeleteSongCall> for TangleTunesAbiCalls {
        fn from(value: DeleteSongCall) -> Self {
            Self::DeleteSong(value)
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
    impl ::core::convert::From<FindDistIndexesCall> for TangleTunesAbiCalls {
        fn from(value: FindDistIndexesCall) -> Self {
            Self::FindDistIndexes(value)
        }
    }
    impl ::core::convert::From<FindInsertIndexesCall> for TangleTunesAbiCalls {
        fn from(value: FindInsertIndexesCall) -> Self {
            Self::FindInsertIndexes(value)
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
    impl ::core::convert::From<GetAuthorOfLengthCall> for TangleTunesAbiCalls {
        fn from(value: GetAuthorOfLengthCall) -> Self {
            Self::GetAuthorOfLength(value)
        }
    }
    impl ::core::convert::From<GetAuthorOfSongIdCall> for TangleTunesAbiCalls {
        fn from(value: GetAuthorOfSongIdCall) -> Self {
            Self::GetAuthorOfSongId(value)
        }
    }
    impl ::core::convert::From<GetAuthorOfSongsCall> for TangleTunesAbiCalls {
        fn from(value: GetAuthorOfSongsCall) -> Self {
            Self::GetAuthorOfSongs(value)
        }
    }
    impl ::core::convert::From<GetChunksCall> for TangleTunesAbiCalls {
        fn from(value: GetChunksCall) -> Self {
            Self::GetChunks(value)
        }
    }
    impl ::core::convert::From<GetDistributorsCall> for TangleTunesAbiCalls {
        fn from(value: GetDistributorsCall) -> Self {
            Self::GetDistributors(value)
        }
    }
    impl ::core::convert::From<GetDistributorsLengthCall> for TangleTunesAbiCalls {
        fn from(value: GetDistributorsLengthCall) -> Self {
            Self::GetDistributorsLength(value)
        }
    }
    impl ::core::convert::From<GetHoldsRightsToLengthCall> for TangleTunesAbiCalls {
        fn from(value: GetHoldsRightsToLengthCall) -> Self {
            Self::GetHoldsRightsToLength(value)
        }
    }
    impl ::core::convert::From<GetHoldsRightsToSongIdCall> for TangleTunesAbiCalls {
        fn from(value: GetHoldsRightsToSongIdCall) -> Self {
            Self::GetHoldsRightsToSongId(value)
        }
    }
    impl ::core::convert::From<GetHoldsRightsToSongsCall> for TangleTunesAbiCalls {
        fn from(value: GetHoldsRightsToSongsCall) -> Self {
            Self::GetHoldsRightsToSongs(value)
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
    impl ::core::convert::From<GetValidatesLengthCall> for TangleTunesAbiCalls {
        fn from(value: GetValidatesLengthCall) -> Self {
            Self::GetValidatesLength(value)
        }
    }
    impl ::core::convert::From<GetValidatesSongIdCall> for TangleTunesAbiCalls {
        fn from(value: GetValidatesSongIdCall) -> Self {
            Self::GetValidatesSongId(value)
        }
    }
    impl ::core::convert::From<GetValidatesSongsCall> for TangleTunesAbiCalls {
        fn from(value: GetValidatesSongsCall) -> Self {
            Self::GetValidatesSongs(value)
        }
    }
    impl ::core::convert::From<IsDistributingCall> for TangleTunesAbiCalls {
        fn from(value: IsDistributingCall) -> Self {
            Self::IsDistributing(value)
        }
    }
    impl ::core::convert::From<ManageValidatorsCall> for TangleTunesAbiCalls {
        fn from(value: ManageValidatorsCall) -> Self {
            Self::ManageValidators(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for TangleTunesAbiCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
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
    impl ::core::convert::From<WithdrawToChainCall> for TangleTunesAbiCalls {
        fn from(value: WithdrawToChainCall) -> Self {
            Self::WithdrawToChain(value)
        }
    }
    impl ::core::convert::From<WithdrawToTangleCall> for TangleTunesAbiCalls {
        fn from(value: WithdrawToTangleCall) -> Self {
            Self::WithdrawToTangle(value)
        }
    }
    ///Container type for all return fields from the `check_chunks` function with signature `check_chunks(bytes32,uint256,uint256)` and selector `0xdc70c749`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CheckChunksReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `chunks_length` function with signature `chunks_length(bytes32)` and selector `0x6a604ceb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    pub struct DistributionsReturn {
        pub fee: ::ethers::core::types::U256,
        pub next_distributor: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `find_dist_indexes` function with signature `find_dist_indexes(bytes32[],address)` and selector `0x6f9e540d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FindDistIndexesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `find_insert_indexes` function with signature `find_insert_indexes(bytes32[],uint256[])` and selector `0x97930cd8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FindInsertIndexesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `gen_distribution_id` function with signature `gen_distribution_id(bytes32,address)` and selector `0x10c1396d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
    )]
    pub struct GenSongIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `get_author_of_length` function with signature `get_author_of_length(address)` and selector `0x2eaef7e7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAuthorOfLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_author_of_song_id` function with signature `get_author_of_song_id(address,uint256)` and selector `0x74934e5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAuthorOfSongIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `get_author_of_songs` function with signature `get_author_of_songs(address,uint256,uint256)` and selector `0x822568b2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAuthorOfSongsReturn(pub ::std::vec::Vec<SongListing>);
    ///Container type for all return fields from the `get_distributors` function with signature `get_distributors(bytes32,address,uint256)` and selector `0x135adae4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetDistributorsReturn(pub ::std::vec::Vec<DistributionListing>);
    ///Container type for all return fields from the `get_distributors_length` function with signature `get_distributors_length(bytes32)` and selector `0x958c21eb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetDistributorsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_holds_rights_to_length` function with signature `get_holds_rights_to_length(address)` and selector `0xa9bec5c6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHoldsRightsToLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_holds_rights_to_song_id` function with signature `get_holds_rights_to_song_id(address,uint256)` and selector `0x3c3b3b9d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHoldsRightsToSongIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `get_holds_rights_to_songs` function with signature `get_holds_rights_to_songs(address,uint256,uint256)` and selector `0x66ac6689`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHoldsRightsToSongsReturn(pub ::std::vec::Vec<SongListing>);
    ///Container type for all return fields from the `get_rand_distributor` function with signature `get_rand_distributor(bytes32,uint256)` and selector `0x60a04a38`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetRandDistributorReturn(pub DistributionListing);
    ///Container type for all return fields from the `get_songs` function with signature `get_songs(uint256,uint256)` and selector `0x99a7cd37`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSongsReturn(pub ::std::vec::Vec<SongListing>);
    ///Container type for all return fields from the `get_validates_length` function with signature `get_validates_length(address)` and selector `0xa2c0e7a9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetValidatesLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_validates_song_id` function with signature `get_validates_song_id(address,uint256)` and selector `0xd81ebaa4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetValidatesSongIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `get_validates_songs` function with signature `get_validates_songs(address,uint256,uint256)` and selector `0x62eae29d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetValidatesSongsReturn(pub ::std::vec::Vec<SongListing>);
    ///Container type for all return fields from the `is_distributing` function with signature `is_distributing(bytes32[],address)` and selector `0xccfd2bd0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsDistributingReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `song_list` function with signature `song_list(uint256)` and selector `0x5c348f6d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct SongsReturn {
        pub exists: bool,
        pub author: ::ethers::core::types::Address,
        pub rightholder: ::ethers::core::types::Address,
        pub validator: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub price: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
        pub duration: ::ethers::core::types::U256,
        pub distributors: ::ethers::core::types::U256,
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
        Hash,
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
        Hash,
    )]
    pub struct L1Address {
        pub data: ::ethers::core::types::Bytes,
    }
    ///`DistributionListing(address,string,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DistributionListing {
        pub distributor: ::ethers::core::types::Address,
        pub server: ::std::string::String,
        pub fee: ::ethers::core::types::U256,
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
        Hash,
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
