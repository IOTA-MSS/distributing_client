A distributing client for the TangleTunes p2p music streaming service.

# Building
1. Install the rust-toolchain. (rustup)
2. Clone the repository and cd into it.
3. Create a binary with `cargo build --release`.
4. Binary is now located at `./target/release/tangle-tunes-distributor`

Alternatively, binaries can be downloaded from [GitHub](https://github.com/TangleTunes/distributing_client/releases).

# Basic setup

## Creating or importing an account
1. Add the `tangle-tunes-distributor` binary to your path.
1. Create s file `TangleTunes.toml` with the following contents:
    ```toml
    # The address registered on the smart contract
    server_address = "<IP>:<PORT>"
    # The address that the tcp-listener should bind on
    bind_address = "0.0.0.0:<PORT>"
    # The path to the database relative to this file
    database_path = "./path/to/database"

    # The fee per chunk in IOTA
    fee = 250

    # Smart-contract details
    chain_id = 1074
    contract_address = "0x8fA1fc1Eec824a36fD31497EAa8716Fc9C446d51"
    node_url = "http://tangletunes.com:9090/chains/tst1pregpfxyv79j5n3hhjwxjg4xvel8cj5nnhz0rh0k0exknn3lu63ax3ck5hg/evm"
    ```
1. Generate or import a wallet with on of the following commands. 
    - `wallet generate --password <PASSWORD>`.
    - `wallet import <PRIVATE_KEY> --password <PASSWORD>`.  
    
    *All commands from now on must use the flag `-p <PASSWORD>` !!*
1. (New wallet only): Request funds with `wallet request-funds`.
1. (New wallet only): Create a TangleTunes account with `account create --name <NAME>`.
1. (New wallet only): Deposit to your account with `account deposit 10000000`.

## Adding songs
Songs can either be added manually with `songs add mp3/<SONG_ID>.mp3` or downloaded with `songs download --song-id <SONG_ID>` from another distributor. Adding songs can be done while actively distributing, which will automatically register for distribution of the given song.

## Distributing
Distribution can be started with the command `distribute`. This starts distributing all songs in the database according to the configuration in `TangleTunes.toml`.

Alternatively the `--demo` flag can be enabled with values `odd`, `even` or `all`. This automatically downloads new songs on the platform, depending on whether they are even or odd. If `all` is enabled then all songs are downloaded. A maximum price can be set with `max_price` in the `TangleTunes.toml` file; the price is in IOTA/chunk.