use std::error::Error;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,
}

#[derive(Serialize)]
struct RpcBody {
    id: u64,
    jsonrpc: &'static str,
    method: &'static str,
    params: serde_json::Value,
}

#[derive(Deserialize, Debug)]
struct RpcResponse<T> {
    id: u64,
    jsonrpc: String,
    result: T,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let body = RpcBody {
        id: 1,
        jsonrpc: "2.0",
        method: "getHealth",
        params: serde_json::Value::Null,
    };
    let response = client.post(&args.url).json(&body).send().await?;
    let parsed = response.json::<RpcResponse<String>>().await?;
    println!("{parsed:#?}");
    Ok(())
}
