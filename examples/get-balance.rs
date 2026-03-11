use clap::Parser;
use solana_rpc_lite::{CommitmentLevel, RpcClient, RpcClientError};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: String,
}

const ADDRESS: &str = "11111111111111111111111111111111";

#[tokio::main]
async fn main() -> Result<(), RpcClientError> {
    let args = Args::parse();

    let client = RpcClient::new(&args.url);

    let result = client
        .get_balance(ADDRESS.to_string())
        .with_commitment(CommitmentLevel::Confirmed)
        .await?;

    println!("address: {}", ADDRESS);
    println!("balance: {}", result.value);

    Ok(())
}
