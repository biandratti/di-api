use serde::{Deserialize, Serialize};
use warp::http::header::AUTHORIZATION;
use warp::reject::custom;
use warp::Filter;

use crate::adapters::api::shared::error_response::CustomRejection;

#[derive(Debug)]
struct AuthenticationError {
    error: String, //TODO: WIP
}

struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    fn new(username: String, password: String) -> Self {
        BasicAuth { username, password }
    }

    async fn authenticate(&self, authorization: Option<String>) -> Result<(), AuthenticationError> {
        // Check if Authorization header is present
        if let Some(auth) = authorization {
            // Extract username and password from Authorization header
            if let Some(credentials) = auth.strip_prefix("Basic ") {
                // Decode base64 encoded credentials
                if let Ok(decoded) = base64::decode(credentials) {
                    // Convert decoded bytes to string
                    if let Ok(decoded_str) = String::from_utf8(decoded) {
                        // Split username and password
                        let parts: Vec<&str> = decoded_str.splitn(2, ':').collect();
                        // Check if username and password match
                        if let [u, p] = parts[..] {
                            if u == &self.username && p == &self.password {
                                // Authentication successful
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        // Authentication failed
        Err(AuthenticationError {
            error: String::from("Invalid Authentication"),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Session {}
pub fn basic_auth(username: String, password: String) -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::optional(AUTHORIZATION.as_str()).and_then(move |authorization: Option<String>| {
        let basic_auth = BasicAuth::new(username.clone(), password.clone());
        async move {
            match basic_auth.authenticate(authorization).await {
                Ok(_) => Ok(Session {}),
                Err(_) => Err(custom(CustomRejection { error: None })),
            }
        }
    })
}
