use std::error::Error;

use bson::DateTime;
use log::info;
use mongodb::{Client, Collection, Database};

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
        //fingerprint.id = Uuid::new();
        self.collection.insert_one(fingerprint, None).await?;
        Ok(())
    }

    /*async fn get_all(&self) -> Result<Vec<Fingerprint>, Box<dyn Error + Send + Sync>> {
        self.collection.find(None, None).await?;
        Ok(Vec::new())
    }*/
}
