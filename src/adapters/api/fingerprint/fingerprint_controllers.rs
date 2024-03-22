use warp::Filter;

use crate::adapters::api::shared::error_response::ErrorResponseHandling;
use crate::adapters::spi::db::fingerprint_repository::MongoFingerprintRepository;
use crate::application::usecases::fingerprint_usecases::FingerprintUseCase;
use crate::domain::entities::Fingerprint;

#[utoipa::path(
post,
path = "/fingerprint",
request_body = Fingerprint,
responses(
(status = 201, description = "Create fingerprint", body = [Fingerprint])
)
)]
pub fn fingerprint_post(
    repo: MongoFingerprintRepository,
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
                    Err(error) => Err(ErrorResponseHandling::map_io_error(error)),
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
pub fn fingerprint_get_all(
    repo: MongoFingerprintRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("fingerprint"))
        .and(warp::path::end())
        .and_then(move || {
            let use_case = repo.clone(); // Use your repository instance
            async move {
                match use_case.get_all_fingerprints().await {
                    Ok(fingerprint_list) => Ok(warp::reply::json(&fingerprint_list)),
                    Err(error) => Err(ErrorResponseHandling::map_io_error(error)),
                }
            }
        })
}
