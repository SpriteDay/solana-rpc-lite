use clap::Parser;
use solana_rpc_lite::{CommitmentLevel, RpcClient, RpcClientError};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    pubkey: String,
}

#[tokio::main]
async fn main() -> Result<(), RpcClientError> {
    let args = Args::parse();

    let client = RpcClient::new(&args.url);

    let result = client
        .get_token_account_balance(args.pubkey.to_string())
        .with_commitment(CommitmentLevel::Finalized)
        .await?;

    println!("address: {}", args.pubkey);
    println!("token amount: {}", result.value.amount);

    Ok(())
}
