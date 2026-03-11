use solana_rpc_lite::RpcClientError;

#[tokio::test]
#[ignore]
async fn test_get_token_account_balance_live() -> Result<(), RpcClientError> {
    let rpc_url = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL env variable is not set");
    let pubkey = std::env::var("TOKEN_ACCOUNT_ADDRESS")
        .expect("TOKEN_ACCOUNT_ADDRESS env variable is not set");
    let client = solana_rpc_lite::RpcClient::new(&rpc_url);

    let _result = client
        .get_token_account_balance(pubkey)
        .with_id(42)
        .with_commitment(solana_rpc_lite::CommitmentLevel::Confirmed)
        .await?;
    Ok(())
}
