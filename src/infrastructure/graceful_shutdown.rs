use std::net::SocketAddrV4;
use std::sync::Arc;

use tokio_graceful_shutdown::SubsystemHandle;
use utoipa_swagger_ui::Config;

use crate::{adapters, infrastructure};

pub async fn mongo_graceful_shutdown(
    subsys: SubsystemHandle,
    client: infrastructure::mongo::MongoClient,
) -> miette::Result<()> {
    subsys.on_shutdown_requested().await;
    tracing::info!("Starting mongo shutdown ...");
    client.client.shutdown().await;
    tracing::info!("Mongo Shutdown finished.");
    Ok(())
}

pub async fn server_graceful_shutdown(
    subsys: SubsystemHandle,
    mongo_client: infrastructure::mongo::MongoClient,
    socket_addr: SocketAddrV4,
) -> miette::Result<()> {
    let config: Arc<Config> = Arc::new(Config::from("/api-doc.json"));

    let repo: adapters::spi::db::fingerprint_repository::MongoFingerprintRepository =
        adapters::spi::db::fingerprint_repository::MongoFingerprintRepository::new(
            mongo_client.client,
            &dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
        )
        .await
        .unwrap();

    let routes = adapters::api::shared::routes::routes_with_swagger(repo, config);
    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(socket_addr, async move {
        subsys.on_shutdown_requested().await;
        tracing::info!("Starting server shutdown ...");
    });

    tracing::info!("Listening on http://{}/swagger-ui/", addr);

    server.await;

    Ok(())
}
