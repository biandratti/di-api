use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct Fingerprint {
    #[schema(value_type = String)]
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[schema(value_type = String, example = "trace_id")]
    pub trace_id: String,
    #[schema(value_type = String, example = "0.0.0.0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[schema(value_type = String, format = Date)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime>,
}
