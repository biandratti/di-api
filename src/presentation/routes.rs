use log::error;
use warp::Filter;

use crate::domain::entities::Fingerprint;
use crate::domain::use_cases::FingerprintUseCase;
use crate::infrastructure;

#[derive(Debug)]
struct CustomRejection(Box<dyn std::error::Error + Send + Sync>);

impl warp::reject::Reject for CustomRejection {}

pub fn routes(
    repo: infrastructure::mongo::MongoFingerprintRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    fingerprint_post(repo.clone()).or(fingerprint_get_all(repo.clone()))
}

fn fingerprint_post(
    repo: infrastructure::mongo::MongoFingerprintRepository,
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
                    Err(e) => {
                        error!("Error creating fingerprint: {}", e);
                        Err(warp::reject::custom(CustomRejection(e)))
                    }
                }
            }
        })
}

fn fingerprint_get_all(
    repo: infrastructure::mongo::MongoFingerprintRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("fingerprint"))
        .and(warp::path::end())
        .and_then(move || {
            let use_case = repo.clone(); // Use your repository instance
            async move {
                match use_case.get_all_fingerprints().await {
                    Ok(fingerprint_list) => Ok(warp::reply::json(&fingerprint_list)),
                    Err(e) => {
                        error!("Error getting all fingerprints: {}", e);
                        Err(warp::reject::custom(CustomRejection(e)))
                    }
                }
            }
        })
}
