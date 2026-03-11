use serde::de::DeserializeOwned;

pub mod get_balance;
pub mod get_health;
pub mod get_slot;
pub mod get_token_account_balance;

pub trait RpcMethod {
    type Response: DeserializeOwned;

    fn method_name(&self) -> &'static str;
    fn params(&self) -> serde_json::Value;
}
