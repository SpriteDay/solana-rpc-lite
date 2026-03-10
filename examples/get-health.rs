use thiserror::Error;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    url: String,
}

#[derive(Serialize)]
struct RpcBody {
    id: u64,
    jsonrpc: &'static str,
    method: &'static str,
    params: serde_json::Value,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RpcResponse<T> {
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

impl<T> RpcResponse<T> {
    /// Validates for matching Id of request and response, checks for JSON RPC version
    pub fn validate(self, expected_id: u64, expected_jsonrpc: &str) -> Result<T, RpcClientError> {
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

#[derive(Deserialize, Debug)]
struct RpcError {
    code: i64,
    message: String,
    data: Option<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), RpcClientError> {
    let args = Args::parse();
    let client = reqwest::Client::new();
    let id = 1_u64;
    let jsonrpc_v = "2.0";
    let method = "getHealth";
    let body = RpcBody {
        id,
        jsonrpc: jsonrpc_v,
        method,
        params: serde_json::Value::Null,
    };
    let response = client.post(&args.url).json(&body).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(RpcClientError::Http {
            status: status.into(),
            body,
        });
    }

    let parsed = response.json::<RpcResponse<String>>().await?;

    let validated = parsed.validate(id, jsonrpc_v)?;

    println!("{validated:#?}");
    Ok(())
}
