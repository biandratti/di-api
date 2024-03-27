use std::sync::Arc;

use once_cell::sync::Lazy;
use opentelemetry::KeyValue;
use prometheus::{Encoder, TextEncoder};
use warp::Filter;

use crate::adapters::api::metrics::app_state::AppState;

pub fn build(state: Arc<AppState>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get().and(warp::path("metrics")).and(warp::path::end()).and_then(move || metrics_handler(state.clone()))
}

//TODO: move...
static HANDLER_ALL: Lazy<[KeyValue; 1]> = Lazy::new(|| [KeyValue::new("handler", "all")]);

async fn metrics_handler(state: Arc<AppState>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = state.registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    state.http_body_gauge.record(buffer.len() as u64, HANDLER_ALL.as_ref());
    Ok(warp::reply::with_header(buffer, "content-type", "text/plain"))
}
