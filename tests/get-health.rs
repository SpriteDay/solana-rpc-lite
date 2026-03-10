#[tokio::test]
#[ignore]
async fn test_get_health_live() -> Result<(), solana_rpc_lite::RpcClientError> {
    let rpc_url = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL env variable is not set");
    let client = solana_rpc_lite::RpcClient::new(&rpc_url);

    let id = 1_u64;
    let result = client.get_health().await?;
    assert_eq!(result, "ok");
    Ok(())
}
