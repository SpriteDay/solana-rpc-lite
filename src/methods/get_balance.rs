use serde::{Deserialize, Serialize};

use crate::{methods::RpcMethod, types::CommitmentLevel};

/// Specs: https://solana.com/docs/rpc/http/getbalance
pub struct GetBalance {
    pub pubkey: String,
    pub config: GetBalanceConfig,
}

#[derive(Serialize)]
pub struct GetBalanceConfig {
    pub commitment: CommitmentLevel,
    pub min_context_slot: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceResponse {
    pub context: ResponseContext,
    pub value: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub slot: u64,
}

impl RpcMethod for GetBalance {
    type Response = GetBalanceResponse;

    fn method_name(&self) -> &'static str {
        "getBalance"
    }

    fn params(&self) -> serde_json::Value {
        serde_json::json!([self.pubkey, self.config])
    }
}
