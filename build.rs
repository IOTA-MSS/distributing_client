use std::{error::Error, fs::File};
use ethers::prelude::Abigen;

fn main() -> Result<(), Box<dyn Error>> {
    if File::open("src/library/abi.rs").is_err() {
        Abigen::new(
            "TangleTunesAbi",
            "../smart_contract/abi/contracts/TangleTunes.sol/TangleTunes.json",
        )?
        .generate()?
        .write_to_file("src/library/abi.rs")?;
    }

    Ok(())
}
