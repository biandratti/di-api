use bson::DateTime;
use futures_util::stream::StreamExt;
use log::info;
use mongodb::{Client, Collection, Database};
use std::error::Error;

use crate::domain::repositories::FingerprintRepository;

use super::super::domain::entities::Fingerprint;

const DB_NAME: &str = "di";
const COLL_NAME: &str = "fingerprint";

#[derive(Debug, Clone)]
pub struct MongoFingerprintRepository {
    collection: Collection<Fingerprint>,
}

impl MongoFingerprintRepository {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::with_uri_str(db_url).await?;
        let db: Database = client.database(DB_NAME);
        let collection: Collection<Fingerprint> = db.collection(COLL_NAME);
        info!("Mongo Connected");
        Ok(Self { collection })
    }
}
impl FingerprintRepository for MongoFingerprintRepository {
    async fn insert(
        &self,
        fingerprint: &mut Fingerprint,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        fingerprint.created = Some(DateTime::now());
        self.collection.insert_one(fingerprint, None).await?;
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Fingerprint>, Box<dyn Error + Send + Sync>> {
        let mut fingerprints = Vec::new();
        let mut cursor = self.collection.find(None, None).await?;
        while let Some(doc) = cursor.next().await {
            fingerprints.push(doc?);
        }

        Ok(fingerprints)
    }
}
