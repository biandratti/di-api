use std::net::SocketAddrV4;

use tokio::time::Duration;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

use crate::infrastructure::graceful_shutdown;
use crate::infrastructure::mongo::MongoClient;

pub async fn run(socket_addr: SocketAddrV4, db_url: &str) {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let mongo_client: MongoClient = MongoClient::new(db_url).await.unwrap();

    let client_clone_mongo = mongo_client.clone();
    // Setup and execute subsystem tree
    let _ = Toplevel::new(move |s| async move {
        s.start(SubsystemBuilder::new("Mongo", move |subsys| async move {
            graceful_shutdown::mongo_graceful_shutdown(subsys, client_clone_mongo).await
        }));

        s.start(SubsystemBuilder::new("Warp Server", move |subsys| async move {
            graceful_shutdown::server_graceful_shutdown(subsys, mongo_client, socket_addr).await
        }));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(2))
    .await;
}
