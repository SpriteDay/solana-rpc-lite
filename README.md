# Solana RPC Lite
Minimal Solana RPC crate, hand-rolled using `reqwest` and `serde`. Done as practice/portfolio piece, but coming out quite nicely - it is thin and easy to reason about.

Implements standard HTTP JSON-RPC client so it works against any Solana node URL, specs: https://solana.com/docs/rpc/http

## Installation
I probably will keep it as portfolio piece for now, so the installation is through git link for now, in your `Cargo.toml`:
```toml
[dependencies]
solana-rpc-lite = { git = "https://github.com/SpriteDay/solana-rpc-lite" }
```

## Usage
Instantiate the RPC client with RPC URL, call method passing optional `id` (`u64`) and actual `params` (where required):
```rs
    let client = solana_rpc_lite::RpcClient::new(&rpc_url);
    let id = 1_u64;
    let result = client.get_health(id).await?; // String "ok"
```

## Errors
Errors are returned via `RpcClientError`, it has 6 variants:
- `Http` - returned when request didn't get success status
- `Rpc` - RPC request received, but with error from server
- `IdMismatch` - if for some reason `id` field of response was not equal to `id` value passed via request
- `InvalidVersion` - a bit exotic one, if for some reason returned `jsonrpc` field not the same version of `jsonrpc` requested, this error gets returned. We use [JSON RPC v2.0](https://www.jsonrpc.org/specification) which is a massively used standard, but still nice to double check!
- `Reqwest` - propagated [reqwest](https://crates.io/crates/reqwest) error
- `Json` - propagated [serde_json](https://crates.io/crates/serde_json) error

## Implemented Methods
For now impletented these methods:
- [getHealth](https://solana.com/docs/rpc/http/gethealth)


## Tests
Integration tests marked with `#[ignore]` macro, because they require working Solana RPC URL specified as `SOLANA_RPC_URL` env variable:
```sh
SOLANA_RPC_URL="http://127.0.0.1:8899" cargo test -- --ignored
```

Crate also has unit tests for core functions


## Examples
To run one of the examples in `/examples` folder, use this command template:
```sh
cargo run --example <example-name> -- --url <rpc-url>
```

You can spin up local `surfpool` and use local url, for example:
```sh
surfpool start
cargo run --example get-health -- --url http://127.0.0.1:8899
```

## Batch Requests
I haven't implemented them yet, but might look into that in the future