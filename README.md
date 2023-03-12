# Development setup
1. Install the rust-toolchain. (rustup)
2. Clone the repository and cd into it.
3. Create the file `TangleTunes.toml` in the root of the repository and paste in the following:
    ```toml
    port = 3000
    contract_address = "0x8fA1fc1Eec824a36fD31497EAa8716Fc9C446d51"
    node_url = 	"http://217.104.126.34:9090/chains/tst1pr2j82svscklywxj8gyk3dt5jz3vpxhnl48hh6h6rn0g8dfna0zsceya7up/evm"
    database_path = "./target/database"
    chain_id = 1074
    fee = 1
    ```
3. Generate or import a wallet with on of the following commands.
    - `cargo run -- wallet generate --plaintext`.
    - `cargo run -- wallet import <PRIVATE_KEY> --plaintext`.
4. (New wallet only): Create a TangleTunes account with `cargo run -- account create --name <NAME>`.
5. (New wallet only): Deposit to your wallet (`cargo run -- wallet address`) with metamask or the debug faucet.
5. (New wallet only): Deposit to your account with `cargo run -- account deposit <AMOUNT>`.
6. Add songs to your database. For example:
    - `cargo run -- songs add mp3/0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2.mp3` (Validated song)
    - `cargo run -- songs add mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3` (Unvalidated song)
7. Start distributing the songs with `cargo run -- distribute`.

## Downloading to a file
```sh
cargo run -- songs download --ip localhost:3000 --start 0 --chunks 10 --to-file ./target/output.mp3 --song 0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2
```

## Reset database
If the database schema changed between updates, it can be reset by deleting `./target/database`.