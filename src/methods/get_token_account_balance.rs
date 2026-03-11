use serde::{Deserialize, Serialize};

use crate::{RpcCall, methods::RpcMethod, types::CommitmentLevel};

/// Specs: https://solana.com/docs/rpc/http/gettokenaccountbalance
pub struct GetTokenAccountBalance {
    pub pubkey: String,
    pub config: GetTokenAccountBalanceConfig,
}

#[derive(Serialize)]
pub struct GetTokenAccountBalanceConfig {
    pub commitment: CommitmentLevel,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountBalanceResult {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWithContext<T> {
    pub context: ResponseContext,
    pub value: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseContext {
    pub slot: u64,
}

impl RpcMethod for GetTokenAccountBalance {
    type Response = ResponseWithContext<GetTokenAccountBalanceResult>;

    fn method_name(&self) -> &'static str {
        "getTokenAccountBalance"
    }

    fn params(&self) -> serde_json::Value {
        serde_json::json!([self.pubkey, self.config])
    }
}

impl<'a> RpcCall<'a, GetTokenAccountBalance> {
    pub fn with_commitment(mut self, commitment: CommitmentLevel) -> Self {
        self.method.config.commitment = commitment;
        self
    }
}
