use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// implement for POST/UPDATE requests
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct FingerprintPayload {
    #[schema(value_type = String, example = "trace_id")]
    pub trace_id: String,
    #[schema(value_type = String, example = "0.0.0.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}
