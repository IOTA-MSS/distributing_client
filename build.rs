use std::{error::Error, fs::File};
use ethers::prelude::Abigen;

fn main() -> Result<(), Box<dyn Error>> {
    const FILE: &str = "src/library/abi/generated.rs";

    if File::open(FILE).is_err() {
        Abigen::new(
            "TangleTunesAbi",
            "../smart_contract/abi/contracts/TangleTunes.sol/TangleTunes.json",
        )?
        .generate()?
        .write_to_file(FILE)?;
    }

    Ok(())
}
