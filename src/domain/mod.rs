pub mod entities {
    use bson::DateTime;
    use mongodb::bson::Uuid;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Fingerprint {
        pub id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub created: Option<DateTime>,
    }
}

pub mod use_cases {
    use std::error::Error;

    use super::entities::Fingerprint;

    pub trait FingerprintUseCase {
        type ConcreteType: FingerprintUseCase;

        async fn new(db_url: &str) -> Result<Box<Self::ConcreteType>, Box<dyn Error + Send + Sync>>;
        async fn insert(&self, fingerprint: &mut Fingerprint) -> Result<(), Box<dyn Error + Send + Sync>>;
        async fn get_all(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
    }
}