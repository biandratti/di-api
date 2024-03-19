use std::sync::Arc;

use utoipa::OpenApi;
use utoipa_swagger_ui::Config;
use warp::Filter;

use crate::domain::entities::Fingerprint;
use crate::domain::use_cases::FingerprintUseCase;
use crate::http_utils::error_handling;
use crate::http_utils::swagger::serve_swagger;
use crate::infrastructure;

#[utoipa::path(
    post,
    path = "/fingerprint",
    request_body = Fingerprint,
    responses(
        (status = 201, description = "Create fingerprint", body = [Fingerprint])
    )
)]
fn fingerprint_post(
    repo: infrastructure::repository::fingerprint_repository::MongoFingerprintRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("fingerprint"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(move |mut fingerprint: Fingerprint| {
            let use_case = repo.clone(); // Use your repository instance
            async move {
                match use_case.create_fingerprint(&mut fingerprint).await {
                    Ok(()) => Ok(warp::reply()),
                    Err(error) => Err(error_handling::ErrorHandling::application_error(
                        "create fingerprint",
                        error,
                    )),
                }
            }
        })
}

#[utoipa::path(
    get,
    path = "/fingerprint",
    responses(
        (status = 200, description = "List fingerprint")
    )
)]
fn fingerprint_get_all(
    repo: infrastructure::repository::fingerprint_repository::MongoFingerprintRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("fingerprint"))
        .and(warp::path::end())
        .and_then(move || {
            let use_case = repo.clone(); // Use your repository instance
            async move {
                match use_case.get_all_fingerprints().await {
                    Ok(fingerprint_list) => Ok(warp::reply::json(&fingerprint_list)),
                    Err(error) => Err(error_handling::ErrorHandling::application_error(
                        "get all fingerprints",
                        error,
                    )),
                }
            }
        })
}

#[derive(OpenApi)]
#[openapi(
    paths(fingerprint_post, fingerprint_get_all),
    components(
        schemas(Fingerprint)
    ),
    tags(
        (name = "fingerprint", description = "Fingerprint items management API")
    )
)]
struct ApiDoc;

pub fn routes_with_swagger(
    repo: infrastructure::repository::fingerprint_repository::MongoFingerprintRepository,
    config: Arc<Config<'static>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    api_doc
        .or(swagger_ui)
        .or(fingerprint_post(repo.clone()).or(fingerprint_get_all(repo.clone())))
}
