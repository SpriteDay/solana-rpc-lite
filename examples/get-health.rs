use clap::Parser;
use solana_rpc_lite::RpcClientError;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), RpcClientError> {
    let args = Args::parse();

    let client = solana_rpc_lite::RpcClient::new(&args.url);

    let id = 1_u64;
    let result = client.get_health(id).await?;

    println!("{result:#?}");
    Ok(())
}
