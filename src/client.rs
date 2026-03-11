use crate::{
    methods::{
        get_balance::{GetBalance, GetBalanceConfig},
        get_health::GetHealth,
    },
    rpc_call::RpcCall,
    types::CommitmentLevel,
};

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

    /// Returns the current health of the node. A healthy node is one that is within
    /// `HEALTH_CHECK_SLOT_DISTANCE` slots of the latest cluster confirmed slot.
    ///
    /// Specs: https://solana.com/docs/rpc/http/gethealth
    pub fn get_health(&'_ self) -> RpcCall<'_, GetHealth> {
        RpcCall {
            id: 1,
            method: GetHealth,
            client: &self.client,
            rpc_url: &self.rpc_url,
        }
    }

    pub fn get_balance(&'_ self, pubkey: String) -> RpcCall<'_, GetBalance> {
        RpcCall {
            id: 1,
            method: GetBalance {
                pubkey,
                config: GetBalanceConfig {
                    commitment: CommitmentLevel::Finalized,
                    min_context_slot: None,
                },
            },
            client: &self.client,
            rpc_url: &self.rpc_url,
        }
    }
}
