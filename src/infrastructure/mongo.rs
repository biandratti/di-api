use std::error::Error;

use bson::{DateTime, Uuid};
use mongodb::{Client, Collection};

use crate::domain::use_cases::FingerprintUseCase;

use super::super::domain::entities::Fingerprint;

const DB_NAME: &str = "di";
const COLL_NAME: &str = "fingerprint";

#[derive(Debug, Clone)]
pub struct MongoFingerprintRepository {
    collection: Collection<Fingerprint>,
}

impl FingerprintUseCase for MongoFingerprintRepository {
    type ConcreteType = MongoFingerprintRepository;

    async fn new(db_url: &str) -> Result<Box<Self::ConcreteType>, Box<dyn Error + Send + Sync>> {
        let client: Client = Client::with_uri_str(db_url).await?;
        let collection: Collection<Fingerprint> = client.database(DB_NAME).collection(COLL_NAME);
        Ok(Box::new(Self { collection }))
    }

    async fn insert(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        fingerprint.created = Some(DateTime::now());
        fingerprint.id = Uuid::new();
        self.collection.insert_one(fingerprint, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Your implementation for get_all method
        Ok(())
    }
}
