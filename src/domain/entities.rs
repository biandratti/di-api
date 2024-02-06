use bson::DateTime;
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct Fingerprint {
    #[schema(example = 1)]
    pub id: Uuid,
    #[schema(example = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[schema(example = "2024-02-05T10:19:21.966Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime>,
}
