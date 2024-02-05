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
    fingerprint_post(repo)
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
                        eprintln!("Error creating fingerprint: {}", e);
                        Err(warp::reject::custom(CustomRejection(e)))
                    }
                }
            }
        })
}
