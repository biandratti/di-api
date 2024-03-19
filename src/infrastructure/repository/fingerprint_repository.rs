use std::error::Error;

use crate::domain::entities::Fingerprint;
use bson::DateTime;
use futures_util::stream::StreamExt;
use log::info;
use mongodb::{Client, Collection, Database};

use crate::domain::fingerprint_repository::FingerprintRepository;

const COLL_NAME: &str = "fingerprints";

#[derive(Debug, Clone)]
pub struct MongoFingerprintRepository {
    collection: Collection<Fingerprint>,
}

impl MongoFingerprintRepository {
    pub async fn new(
        client: Client,
        db_name: &String,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let db: Database = client.database(db_name);
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
