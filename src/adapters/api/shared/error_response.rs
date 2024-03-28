use std::error::Error;
use warp::reject::custom;
use warp::reject::Reject;
use warp::Rejection;

use crate::domain::error::ApiError;

pub struct ErrorResponseHandling {}

impl ErrorResponseHandling {
    pub fn map_io_error(e: ApiError) -> Rejection {
        match e.get_error_code() {
            400 => custom(CustomRejection { error: Some(e.error) }),
            _ => custom(CustomRejection { error: Some(e.error) }),
        }
    }
}

#[derive(Debug)]
pub struct CustomRejection {
    pub error: Option<Box<dyn Error + Send + Sync>>,
}

impl Reject for CustomRejection {}
