# Development setup
1. Install the rust-toolchain. (rustup)
2. Clone the repository and cd into it.
3. Generate or import a wallet with on of the following commands with `--plaintext` or `--password <PASSWORD>`.
    - `cargo run -- generate-wallet`.
    - `cargo run -- import-wallet <PRIVATE_KEY>`.
4. Add the songs to your database with.
    - `cargo run -- add-from-path mp3/0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2.mp3` (Validated song)
    - `cargo run -- add-from-path mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3` (Unvalidated song)
4. Create a TangleTunes account with `cargo run -- create-account --name <NAME>`.
5. Run `cargo run -- run` to start distributing on port 3000.

## Reset
Resetting can be done easily by deleting the created `./target/database` file.

## Download example
```bash
cargo run -- download-local --distributor-port 3000 --index 0 --chunks 20 --file ./target/output.mp3 --song-id 0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2
```