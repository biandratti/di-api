use std::net::SocketAddrV4;
use std::sync::Arc;

use opentelemetry::metrics::{Meter, MeterProvider};
use opentelemetry_prometheus::PrometheusExporter;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::Registry;
use tokio_graceful_shutdown::SubsystemHandle;
use utoipa_swagger_ui::Config;
use warp::Filter;

use crate::adapters::api::metrics::metrics_controller;
use crate::adapters::api::shared::routes::routes_with_swagger;
use crate::adapters::spi::db::fingerprint_repository::MongoFingerprintRepository;
use crate::infrastructure::metrics::{AppState, Metrics};
use crate::infrastructure::mongo::MongoClient;

pub async fn mongo_graceful_shutdown(subsys: SubsystemHandle, client: MongoClient) -> miette::Result<()> {
    subsys.on_shutdown_requested().await;
    tracing::info!("Starting mongo shutdown ...");
    client.client.shutdown().await;
    tracing::info!("Mongo Shutdown finished.");
    Ok(())
}

pub async fn server_graceful_shutdown(subsys: SubsystemHandle, mongo_client: MongoClient, socket_addr: SocketAddrV4) -> miette::Result<()> {
    let config: Arc<Config> = Arc::new(Config::from("/api-doc.json"));

    let repo: MongoFingerprintRepository = MongoFingerprintRepository::new(mongo_client.client, &dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set"))
        .await
        .unwrap();

    let registry: Registry = Registry::new();
    let exporter: PrometheusExporter = opentelemetry_prometheus::exporter().with_registry(registry.clone()).build().unwrap();
    let provider: SdkMeterProvider = SdkMeterProvider::builder().with_reader(exporter).build();

    let meter: Meter = provider.meter("api-example");
    let state: Arc<AppState> = Metrics::new(meter, registry).app_state;
    let routes = routes_with_swagger(repo, config).or(metrics_controller::build(state));
    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(socket_addr, async move {
        subsys.on_shutdown_requested().await;
        tracing::info!("Starting server shutdown ...");
    });

    tracing::info!("Listening on http://{}/swagger-ui/", addr);

    server.await;

    Ok(())
}
