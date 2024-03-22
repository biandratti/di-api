use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FingerprintEntity {
    pub id: Option<ObjectId>,
    pub trace_id: String,
    pub ip: Option<String>,
    pub created: Option<DateTime>,
}
