use crate::{
    methods::{RpcMethod, get_health::GetHealth},
    rpc_call::RpcCall,
    types::{RpcBody, RpcClientError, RpcResponse},
};

const JSON_RPC_V: &str = "2.0";

pub struct RpcClient {
    rpc_url: String,
    client: reqwest::Client,
}

impl RpcClient {
    /// Requires RPC URL as argument:
    /// ```
    /// use solana_rpc_lite::RpcClient;
    ///
    /// let client = RpcClient::new("http://127.0.0.1:8899");
    /// ```
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    async fn call<T: RpcMethod>(&self, method: T) -> Result<T::Response, RpcClientError> {
        let default_id = 1_u64;
        let body = RpcBody {
            id: default_id,
            jsonrpc: JSON_RPC_V,
            method: method.method_name(),
            params: method.params(),
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

        let parsed = response.json::<RpcResponse<T::Response>>().await?;

        let validated = parsed.validate(default_id, JSON_RPC_V)?;

        Ok(validated)
    }

    /// Returns the current health of the node. A healthy node is one that is within
    /// `HEALTH_CHECK_SLOT_DISTANCE` slots of the latest cluster confirmed slot.
    ///
    /// Specs: https://solana.com/docs/rpc/http/gethealth
    pub fn get_health(&self) -> RpcCall<GetHealth> {
        RpcCall {
            method: GetHealth,
            client: &self.client,
            rpc_url: &self.rpc_url,
        }
    }
}
