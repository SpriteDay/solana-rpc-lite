use crate::methods::RpcMethod;

/// Specs: https://solana.com/docs/rpc/http/gethealth
pub struct GetHealth;

impl RpcMethod for GetHealth {
    type Response = String;

    fn method_name(&self) -> &'static str {
        "getHealth"
    }

    fn params(&self) -> serde_json::Value {
        serde_json::json!([])
    }
}
