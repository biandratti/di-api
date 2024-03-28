use std::sync::Arc;

use opentelemetry::metrics::{Counter, Histogram, Meter, Unit};
use prometheus::Registry;

pub struct AppState {
    pub registry: Registry,
    pub http_counter: Counter<u64>,
    pub http_body_gauge: Histogram<u64>,
    pub http_req_histogram: Histogram<f64>,
}

pub struct Metrics {
    pub app_state: Arc<AppState>,
}

impl Metrics {
    pub fn new(meter: Meter, registry: Registry) -> Self {
        Metrics {
            app_state: Arc::new(AppState::new(meter, registry)),
        }
    }
}

impl AppState {
    pub fn new(meter: Meter, registry: Registry) -> Self {
        AppState {
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
        }
    }
}
