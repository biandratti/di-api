use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::Filter;

use crate::adapters;
use crate::adapters::api::fingerprint::fingerprint_controllers;
use crate::adapters::api::fingerprint::fingerprint_payload::FingerprintPayload;
use crate::adapters::api::fingerprint::fingerprint_presenters::FingerprintPresenter;
use crate::infrastructure::swagger::serve_swagger;

#[derive(OpenApi)]
#[openapi(
paths(fingerprint_controllers::fingerprint_post, fingerprint_controllers::fingerprint_get_all),
components(
schemas(FingerprintPresenter, FingerprintPayload)
),
tags(
(name = "fingerprint", description = "Fingerprint items management API")
)
)]
struct ApiDoc;

pub fn routes_with_swagger(
    repo: adapters::spi::db::fingerprint_repository::MongoFingerprintRepository,
    config: Arc<Config<'static>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_doc = warp::path("api-doc.json").and(warp::get()).map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    api_doc.or(swagger_ui).or(fingerprint_controllers::build(repo))
}
