use mongodb::{options::ClientOptions, Client};
use std::error::Error;
use tokio_graceful_shutdown::SubsystemHandle;

pub struct MongoClient {
    pub client: Client,
}

impl Clone for MongoClient {
    fn clone(&self) -> Self {
        Self { client: self.client.clone() }
    }
}

impl MongoClient {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client_options = ClientOptions::parse(db_url).await?;
        let client = Client::with_options(client_options)?;

        Ok(Self { client })
    }
}

pub struct MongoGracefulShutdown {}

impl MongoGracefulShutdown {
    pub async fn execute(subsys: SubsystemHandle, client: MongoClient) -> miette::Result<()> {
        subsys.on_shutdown_requested().await;
        tracing::info!("Starting mongo shutdown ...");
        client.client.shutdown().await;
        tracing::info!("Mongo Shutdown finished.");
        Ok(())
    }
}
