# Development setup
1. Install the rust-toolchain. (rustup)
2. Clone the repository and cd into it.
3. Create the file `TangleTunes.toml` in the root of the repository and paste in the following:
    ```toml
    # The address registered on the smart contract
    server_address = "127.0.0.1:3000"
    # The address that the tcp-listener should bind on
    bind_address = "127.0.0.1:3000"
    # The path to the database relative to this file
    database_path = "./target/database"

    # The fee per chunk in TUNES
    fee = 1

    # Smart-contract details
    node_url = "http://217.104.126.34:9090/chains/tst1pr2j82svscklywxj8gyk3dt5jz3vpxhnl48hh6h6rn0g8dfna0zsceya7up/evm"
    chain_id = 1074
    contract_address = "0xa57D405951896582EB0535f7566556FdEd498bD1"
    ```
3. Generate or import a wallet with on of the following commands.
    - `cargo run -- wallet generate --plaintext`.
    - `cargo run -- wallet import <PRIVATE_KEY> --plaintext`.
4. (New wallet only): Create a TangleTunes account with `cargo run -- account create --name <NAME>`.
5. (New wallet only): Deposit to your wallet (`cargo run -- wallet address`) with metamask or the debug faucet.
5. (New wallet only): Deposit to your account with `cargo run -- account deposit <AMOUNT>`.
6. Add songs to your database with `cargo run -- songs add mp3/<SONG_ID>.mp3`
7. Start distributing the songs with `cargo run -- distribute`.

## Downloading to a file
```sh
cargo run -- songs download --ip localhost:3000 --start 0 --chunks 10 --to-file ./target/output.mp3 --song <SONG_ID>
```

## Reset database
If the database schema changed between updates, it can be reset by deleting the database.