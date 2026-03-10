use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct RpcBody {
    pub(crate) id: u64,
    pub(crate) jsonrpc: &'static str,
    pub(crate) method: &'static str,
    pub(crate) params: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum RpcResponse<T> {
    Success {
        id: u64,
        jsonrpc: String,
        result: T,
    },
    Failure {
        #[allow(dead_code)]
        id: u64,
        #[allow(dead_code)]
        jsonrpc: String,
        error: RpcError,
    },
}

#[derive(Deserialize, Debug)]
pub(crate) struct RpcError {
    code: i64,
    message: String,
    data: Option<serde_json::Value>,
}

impl<T> RpcResponse<T> {
    /// Validates for matching Id of request and response, checks for JSON RPC version
    pub(crate) fn validate(
        self,
        expected_id: u64,
        expected_jsonrpc: &str,
    ) -> Result<T, RpcClientError> {
        match self {
            Self::Success {
                id,
                jsonrpc,
                result,
            } => {
                if id != expected_id {
                    return Err(RpcClientError::IdMismatch {
                        expected: expected_id,
                        got: id,
                    });
                };
                if jsonrpc != expected_jsonrpc {
                    return Err(RpcClientError::InvalidVersion {
                        expected: expected_jsonrpc.to_string(),
                        got: jsonrpc,
                    });
                }
                Ok(result)
            }
            Self::Failure {
                id: _,
                jsonrpc: _,
                error,
            } => Err(RpcClientError::Rpc {
                code: error.code,
                message: error.message,
                data: error.data,
            }),
        }
    }
}

#[derive(Debug, Error)]
pub enum RpcClientError {
    #[error("HTTP error, status: {status}, body: {body}")]
    Http { status: u16, body: String },

    #[error("RPC error, code: {code}, message: {message}, data: {data:?}")]
    Rpc {
        code: i64,
        message: String,
        data: Option<serde_json::Value>,
    },

    #[error("Response Id mismatch: expected {expected}, got {got}")]
    IdMismatch { expected: u64, got: u64 },

    #[error("Mismatched JSON RPC version, expected: {expected}, got: {got}")]
    InvalidVersion { expected: String, got: String },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
