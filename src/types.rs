use thiserror::Error;

use serde::{Deserialize, Serialize};

/// JSON RPC v2.0 envelope
/// Specs: https://www.jsonrpc.org/specification
#[derive(Serialize)]
pub(crate) struct RpcBody {
    pub(crate) id: u64,
    pub(crate) jsonrpc: &'static str,
    pub(crate) method: &'static str,
    pub(crate) params: serde_json::Value,
}

/// We expect JSON RPC 2.0 envelope in response
/// Specs: https://www.jsonrpc.org/specification
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

/// Possible errors according to specs (https://www.jsonrpc.org/specification):
/// - `-32700` - `Parse error` - Invalid JSON was received by the server. An error occurred on the server while parsing the JSON text.
/// - `-32600` - `Invalid Request` - The JSON sent is not a valid Request object.
/// - `-32601` - `Method not found` - The method does not exist / is not available.
/// - `-32602` - Invalid params Invalid method parameter(s).
/// - `-32603` - `Internal error` - Internal JSON-RPC error.
/// - `-32000` to `-32099` - `Server error` - Reserved for implementation-defined server-errors.
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

/// Errors have these variants:
/// - `Http` - returned when request didn't get success status
/// - `Rpc` - RPC request received, but with error from server
/// - `IdMismatch` - if for some reason `id` field of response was not equal to `id` value passed via request
/// - `InvalidVersion` - a bit exotic one, if for some reason returned `jsonrpc` field not the same version of `jsonrpc` requested, this error gets returned.
/// We use [JSON RPC v2.0](https://www.jsonrpc.org/specification) which is a massively used standard, but still nice to double check!
/// - `Reqwest` - propagated [reqwest](https://crates.io/crates/reqwest) error
/// - `Json` - propagated [serde_json](https://crates.io/crates/serde_json) error
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_success_response() {
        let id = 1_u64;
        let jsonrpc = "2.0";
        let result = "ok";
        let response = RpcResponse::<String>::Success {
            id,
            jsonrpc: jsonrpc.to_string(),
            result: result.to_string(),
        };
        let validated = response.validate(id, jsonrpc);
        assert_eq!(validated.unwrap(), result);
    }

    #[test]
    fn returns_id_mismatch_on_wrong_id() {
        let expected_id = 1_u64;
        let returned_id = 2_u64;
        let jsonrpc = "2.0";
        let result = "ok";
        let response = RpcResponse::<String>::Success {
            id: returned_id,
            jsonrpc: jsonrpc.to_string(),
            result: result.to_string(),
        };
        let validated = response.validate(expected_id, jsonrpc);
        assert!(matches!(
            validated,
            Err(RpcClientError::IdMismatch {
                expected: 1,
                got: 2
            })
        ));
    }
}
