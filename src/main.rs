use std::sync::Arc;

use miette::Result;
use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, SubsystemHandle, Toplevel};
use utoipa_swagger_ui::Config;

mod domain;
mod http_utils;
mod infrastructure;
mod presentation;

async fn mongo_subsystem(
    subsys: SubsystemHandle,
    client: infrastructure::mongo::MongoClient,
) -> Result<()> {
    subsys.on_shutdown_requested().await;
    tracing::info!("Starting mongo shutdown ...");
    client.client.shutdown().await;
    tracing::info!("Mongo Shutdown finished.");
    Ok(())
}
async fn warp_subsystem(
    subsys: SubsystemHandle,
    client: infrastructure::mongo::MongoClient,
) -> Result<()> {
    let config: Arc<Config> = Arc::new(Config::from("/api-doc.json"));

    let repo: infrastructure::repository::fingerprint_repository::MongoFingerprintRepository =
        infrastructure::repository::fingerprint_repository::MongoFingerprintRepository::new(
            client.client,
        )
        .await
        .unwrap();

    let routes = presentation::routes::routes_with_swagger(repo, config);

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async move {
            subsys.on_shutdown_requested().await;
            tracing::info!("Starting server shutdown ...");
        });

    tracing::info!("Listening on http://{}/swagger-ui/", addr);

    server.await;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let client: infrastructure::mongo::MongoClient =
        infrastructure::mongo::MongoClient::new("mongodb://localhost:27017")
            .await
            .unwrap();

    let client_clone_mongo = client.clone();
    // Setup and execute subsystem tree
    let _ = Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("Mongo", move |subsys| async move {
            mongo_subsystem(subsys, client_clone_mongo).await
        }));

        s.start(SubsystemBuilder::new(
            "Warp Server",
            move |subsys| async move { warp_subsystem(subsys, client).await },
        ));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(2))
    .await;
}
