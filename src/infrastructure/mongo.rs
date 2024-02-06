use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub struct MongoClient {
    pub client: Client,
}

impl MongoClient {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client_options = ClientOptions::parse(db_url).await?;
        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }

    /*pub async fn graceful_shutdown(&self) {
        //TODO: WIP
    }*/
}
