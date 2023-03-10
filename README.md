# Setup
1. Install the rust-toolchain. (rustup)
2. Generate or import a wallet with 
    - `cargo run -- generate-wallet` with `--plaintext` or `--password <PASSWORD>`.
    - `cargo run -- import-wallet <PRIVATE_KEY>` 
3. Run `cargo run -- export-address` and copy the address.
4. Send money to this address using metamask or the faucet.
5. Run `cargo run -- add-from-path mp3 0x51dba6a00c006f51b012f6e6c1516675ee4146e03628e3567980ed1c354441f2.mp3`
6. Run `cargo run -- run` to start distributing.