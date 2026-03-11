use crate::{
    RpcClientError,
    methods::{RpcMethod, get_balance::GetBalance},
    types::{CommitmentLevel, RpcBody, RpcResponse},
};
use std::pin::Pin;

const JSON_RPC_V: &str = "2.0";

pub struct RpcCall<'a, T: RpcMethod> {
    pub(crate) id: u64,
    pub(crate) method: T,
    pub(crate) client: &'a reqwest::Client,
    pub(crate) rpc_url: &'a String,
}

impl<'a, T: RpcMethod> RpcCall<'a, T> {
    async fn call(self) -> Result<T::Response, RpcClientError> {
        let body = RpcBody {
            id: self.id,
            jsonrpc: JSON_RPC_V,
            method: self.method.method_name(),
            params: self.method.params(),
        };
        let response = self.client.post(self.rpc_url).json(&body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(RpcClientError::Http {
                status: status.into(),
                body,
            });
        }

        let parsed = response.json::<RpcResponse<T::Response>>().await?;

        let validated = parsed.validate(self.id, JSON_RPC_V)?;

        Ok(validated)
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }
}

impl<'a> RpcCall<'a, GetBalance> {
    pub fn with_commitment(mut self, commitment: CommitmentLevel) -> Self {
        self.method.config.commitment = commitment;
        self
    }

    pub fn with_min_context_slot(mut self, slot: u64) -> Self {
        self.method.config.min_context_slot = Some(slot);
        self
    }
}

impl<'a, T: RpcMethod + 'a> IntoFuture for RpcCall<'a, T> {
    type Output = Result<T::Response, RpcClientError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + 'a>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.call())
    }
}
