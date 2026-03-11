use serde::Serialize;

use crate::{RpcCall, methods::RpcMethod, types::CommitmentLevel};

/// Specs: https://solana.com/docs/rpc/http/getslot
pub struct GetSlot {
    pub config: GetSlotConfig,
}

#[derive(Serialize)]
pub struct GetSlotConfig {
    pub commitment: CommitmentLevel,
    pub min_context_slot: Option<u64>,
}

impl RpcMethod for GetSlot {
    type Response = u64;

    fn method_name(&self) -> &'static str {
        "getSlot"
    }

    fn params(&self) -> serde_json::Value {
        serde_json::json!([self.config])
    }
}

impl<'a> RpcCall<'a, GetSlot> {
    pub fn with_commitment(mut self, commitment: CommitmentLevel) -> Self {
        self.method.config.commitment = commitment;
        self
    }

    pub fn with_min_context_slot(mut self, slot: u64) -> Self {
        self.method.config.min_context_slot = Some(slot);
        self
    }
}
