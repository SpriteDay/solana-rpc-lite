use crate::types::{RpcBody, RpcClientError, RpcResponse};

const JSON_RPC_V: &str = "2.0";

pub struct RpcClient {
    rpc_url: String,
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_health(&self, id: u64) -> Result<String, RpcClientError> {
        let method = "getHealth";
        let body = RpcBody {
            id,
            jsonrpc: JSON_RPC_V,
            method,
            params: serde_json::Value::Null,
        };
        let response = self.client.post(&self.rpc_url).json(&body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RpcClientError::Http {
                status: status.into(),
                body,
            });
        }

        let parsed = response.json::<RpcResponse<String>>().await?;

        let validated = parsed.validate(id, JSON_RPC_V)?;

        Ok(validated)
    }
}
