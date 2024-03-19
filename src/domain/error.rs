#[derive(Debug)]
pub struct CustomRejection(pub Box<dyn std::error::Error + Send + Sync>);

impl warp::reject::Reject for CustomRejection {}
