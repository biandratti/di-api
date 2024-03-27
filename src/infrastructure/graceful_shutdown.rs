use opentelemetry::metrics::{MeterProvider, Unit};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::Registry;
use std::net::SocketAddrV4;
use std::sync::Arc;

use tokio_graceful_shutdown::SubsystemHandle;
use utoipa_swagger_ui::Config;

use crate::adapters::api::metrics::app_state::AppState;
use crate::adapters::api::metrics::metrics_controller;
use crate::{adapters, infrastructure};

pub async fn mongo_graceful_shutdown(subsys: SubsystemHandle, client: infrastructure::mongo::MongoClient) -> miette::Result<()> {
    subsys.on_shutdown_requested().await;
    tracing::info!("Starting mongo shutdown ...");
    client.client.shutdown().await;
    tracing::info!("Mongo Shutdown finished.");
    Ok(())
}

pub async fn server_graceful_shutdown(subsys: SubsystemHandle, mongo_client: infrastructure::mongo::MongoClient, socket_addr: SocketAddrV4) -> miette::Result<()> {
    let config: Arc<Config> = Arc::new(Config::from("/api-doc.json"));

    let repo: adapters::spi::db::fingerprint_repository::MongoFingerprintRepository =
        adapters::spi::db::fingerprint_repository::MongoFingerprintRepository::new(mongo_client.client, &dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set"))
            .await
            .unwrap();

    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter().with_registry(registry.clone()).build().unwrap();
    let provider = SdkMeterProvider::builder().with_reader(exporter).build();

    let meter = provider.meter("hyper-example");
    let state: Arc<AppState> = Arc::new(AppState {
        registry,
        http_counter: meter.u64_counter("http_requests_total").with_description("Total number of HTTP requests made.").init(),
        http_body_gauge: meter
            .u64_histogram("example.http_response_size")
            .with_unit(Unit::new("By"))
            .with_description("The metrics HTTP response sizes in bytes.")
            .init(),
        http_req_histogram: meter
            .f64_histogram("example.http_request_duration")
            .with_unit(Unit::new("ms"))
            .with_description("The HTTP request latencies in milliseconds.")
            .init(),
    });

    let routes = adapters::api::shared::routes::routes_with_swagger(repo, config).or(metrics_controller::build(state));
    let (addr, server) = warp::serve(routes).bind_with_graceful_shutdown(socket_addr, async move {
        subsys.on_shutdown_requested().await;
        tracing::info!("Starting server shutdown ...");
    });

    tracing::info!("Listening on http://{}/swagger-ui/", addr);

    server.await;

    Ok(())
}
