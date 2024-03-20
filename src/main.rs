use std::env;
use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

mod adapters;
mod domain;
mod graceful_shutdown;
mod http_utils;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }
    dotenv::from_filename(environment_file).ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let client: infrastructure::mongo::MongoClient = infrastructure::mongo::MongoClient::new(
        &(dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set")),
    )
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
