use std::error::Error;

use log::error;
use warp::Rejection;

use crate::domain::error::CustomRejection;

pub struct ErrorHandling {}

impl ErrorHandling {
    pub fn application_error(error_message: &str, err: Box<dyn Error + Send + Sync>) -> Rejection {
        ErrorHandling::log_error(error_message, &err);
        warp::reject::custom(CustomRejection(err))
    }

    fn log_error(message: &str, err: &Box<dyn Error + Send + Sync>) {
        error!("Error {}: {}", message, err);
    }
}
