# Solana RPC Lite
Minimal Solana RPC crate, hand-rolled using only reqwest and serde

# Examples
To run one of the examples in `/examples` folder, use this command template:
```sh
cargo run --example <example-name> -- --url <rpc-url>
```

You can spin up local `surfpool` and use local url, for example:
```sh
surfpool start
cargo run --example get-health -- --url http://127.0.0.1:8899
```