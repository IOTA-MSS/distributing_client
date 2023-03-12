# Development setup
1. Install the rust-toolchain. (rustup)
2. Clone the repository and cd into it.
3. Generate or import a wallet with on of the following commands.
    - `cargo run -- wallet generate --plaintext`.
    - `cargo run -- wallet import <PRIVATE_KEY> --plaintext`.
4. Optionally create a TangleTunes account with `cargo run -- account create --name <NAME>`.
5. Deposit to your account with `cargo run -- account deposit <AMOUNT>`.
6. Add the songs to your database with.
    - `cargo run -- songs add mp3/0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2.mp3` (Validated song)
    - `cargo run -- songs add mp3/0x0800000722040506080000072204050608000007220405060800000722040506.mp3` (Unvalidated song)
7. Run `cargo run -- distribute` to start distributing on port 3000.

## Reset
Resetting can be done easily by deleting the created `./target/database` file.

## Download example
```bash
cargo run -- download-local --distributor-port 3000 --index 0 --chunks 20 --file ./target/output.mp3 --song-id 0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2
```