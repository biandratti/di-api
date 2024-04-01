use warp::http::StatusCode;
use warp::Filter;

use crate::adapters::api::fingerprint::fingerprint_mapper::FingerprintMapper;
use crate::adapters::api::fingerprint::fingerprint_payload::FingerprintPayload;
use crate::adapters::api::fingerprint::fingerprint_presenters::FingerprintPresenter;
use crate::adapters::api::shared::basic_auth::{basic_auth, Session};
use crate::adapters::api::shared::error_handler;
use crate::adapters::spi::db::fingerprint_repository::MongoFingerprintRepository;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::usecases::fingerprint_usecases::FingerprintUseCase;

pub fn build(repo: MongoFingerprintRepository) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fingerprint_post(repo.clone()).or(fingerprint_get_all(repo.clone()))
}

#[utoipa::path(
post,
path = "/fingerprint",
request_body = FingerprintPayload,
responses(
(status = 201, description = "Create fingerprint", body = [FingerprintPayload])
),
security(
("Authorization" = [])
)
)]
fn fingerprint_post(repo: MongoFingerprintRepository) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("fingerprint"))
        .and(warp::path::end())
        .and(basic_auth())
        .and(warp::body::json())
        .and_then(move |_session: Session, fingerprint: FingerprintPayload| {
            let use_case = repo.clone(); // Use your repository instance
            async move {
                match use_case.create_fingerprint(&mut (FingerprintMapper::to_entity(fingerprint))).await {
                    Ok(()) => Ok(StatusCode::CREATED),
                    Err(error) => Err(warp::reject::custom(error_handler::Error::ClientError(error_handler::APILayerError {
                        status: error.code,
                        message: error.message,
                    }))),
                }
            }
        })
}

#[utoipa::path(
get,
path = "/fingerprint",
responses(
(status = 200, description = "List fingerprint")
),
security(
("Authorization" = [])
)
)]
fn fingerprint_get_all(repo: MongoFingerprintRepository) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("fingerprint"))
        .and(warp::path::end())
        .and(basic_auth())
        .and_then(move |_session: Session| {
            let use_case = repo.clone(); // Use your repository instance
            async move {
                match use_case.get_all_fingerprints().await {
                    Ok(fingerprint_list) => Ok(warp::reply::json(
                        &fingerprint_list.into_iter().map(FingerprintMapper::to_api).collect::<Vec<FingerprintPresenter>>(),
                    )),
                    Err(error) => Err(warp::reject::custom(error_handler::Error::ClientError(error_handler::APILayerError {
                        status: error.code,
                        message: error.message,
                    }))),
                }
            }
        })
}
