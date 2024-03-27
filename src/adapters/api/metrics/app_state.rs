use opentelemetry::metrics::{Counter, Histogram};
use prometheus::Registry;

pub struct AppState {
    pub registry: Registry,
    pub http_counter: Counter<u64>,
    pub http_body_gauge: Histogram<u64>,
    pub http_req_histogram: Histogram<f64>,
}
