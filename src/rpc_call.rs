use crate::{
    RpcClientError,
    methods::RpcMethod,
    types::{RpcBody, RpcResponse},
};
use std::future::{Ready, ready};

const JSON_RPC_V: &str = "2.0";

pub(crate) struct RpcCall<'a, T: RpcMethod> {
    pub(crate) method: T,
    pub(crate) client: &'a reqwest::Client,
    pub(crate) rpc_url: &'a String,
}

impl<'a, T: RpcMethod> IntoFuture for RpcCall<'a, T> {
    type Output = Result<T::Response, RpcClientError>;
    type IntoFuture = Ready<Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        let default_id = 1_u64;
        let body = RpcBody {
            id: default_id,
            jsonrpc: JSON_RPC_V,
            method: self.method.method_name(),
            params: self.method.params(),
        };
        let response = self.client.post(self.rpc_url).json(&body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return ready(Err(RpcClientError::Http {
                status: status.into(),
                body,
            }));
        }

        let parsed = response.json::<RpcResponse<T::Response>>().await?;

        let validated = parsed.validate(default_id, JSON_RPC_V)?;

        ready(Ok(validated))
    }
}
