#[derive(Debug)]
pub struct CustomRejection(pub Box<dyn std::error::Error + Send + Sync>);

impl warp::reject::Reject for CustomRejection {
    fn get_error_message(&self) -> String {
        todo!()
    }
}

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ApiError {
    pub code: u16,
    pub message: String,
    pub error: Box<dyn Error + Send + Sync>,
}

// Implement std::fmt::Display for AppError
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

/*impl ApiError {
    pub fn get_error_message(&self) -> String {
        String::from(&self.message)
    }

    pub fn get_error_code(&self) -> u16 {
        self.code
    }
}*/
