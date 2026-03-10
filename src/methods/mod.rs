use serde::de::DeserializeOwned;

pub mod get_health;

pub(crate) trait RpcMethod {
    type Response: DeserializeOwned;

    fn method_name(&self) -> &'static str;
    fn params(&self) -> serde_json::Value;
}
