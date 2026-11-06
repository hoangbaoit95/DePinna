//! Debug Adapter Protocol (DAP) client model.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// DAP request envelope (the transport adds Content-Length framing).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DapRequest {
    pub seq: u64,
    #[serde(rename = "type")]
    pub message_type: String,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
}

/// DAP response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DapResponse {
    pub seq: u64,
    #[serde(rename = "type")]
    pub message_type: String,
    pub request_seq: u64,
    pub success: bool,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}
