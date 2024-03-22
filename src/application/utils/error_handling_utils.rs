use std::error::Error;

use log::error;

use crate::domain::error::ApiError;

pub struct ErrorHandlingUtils {}

impl ErrorHandlingUtils {
    pub fn application_error(error_message: &str, error: Box<dyn Error + Send + Sync>) -> ApiError {
        error!("Error {}: {}", error_message, error);
        ApiError {
            code: 400,
            message: String::from(error_message),
            error,
        }
    }
}
