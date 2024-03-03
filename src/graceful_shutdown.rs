use crate::{infrastructure, presentation};
use std::sync::Arc;
use tokio_graceful_shutdown::SubsystemHandle;
use utoipa_swagger_ui::Config;

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
    client: infrastructure::mongo::MongoClient,
) -> miette::Result<()> {
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
