use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};

use crate::domain::entities::Fingerprint;
use crate::presentation::routes;

mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let config = Arc::new(Config::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
        paths(routes::fingerprint_post, routes::fingerprint_get_all),
        components(
            schemas(Fingerprint)
        ),
        tags(
            (name = "fingerprint", description = "Fingerprint items management API")
        )
    )]
    struct ApiDoc;

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    let repo: infrastructure::mongo::MongoFingerprintRepository =
        infrastructure::mongo::MongoFingerprintRepository::new("mongodb://localhost:27017")
            .await
            .unwrap();

    //TODO: WIP
    let _routes = presentation::routes::routes(repo);

    warp::serve(api_doc.or(swagger_ui))
        //.or(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
