use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

mod domain;
mod graceful_shutdown;
mod http_utils;
mod infrastructure;
mod presentation;

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
            graceful_shutdown::mongo_graceful_shutdown(subsys, client_clone_mongo).await
        }));

        s.start(SubsystemBuilder::new(
            "Warp Server",
            move |subsys| async move {
                graceful_shutdown::server_graceful_shutdown(subsys, client).await
            },
        ));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(2))
    .await;
}
