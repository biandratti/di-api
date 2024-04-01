use serde::{Deserialize, Serialize};
use warp::http::header::AUTHORIZATION;
use warp::{Filter, Rejection};

use crate::adapters::api::shared::error_handler;

struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    fn new(username: String, password: String) -> Self {
        BasicAuth { username, password }
    }

    async fn authenticate(&self, authorization: Option<String>) -> Result<(), Rejection> {
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
                            if u == self.username && p == self.password {
                                // Authentication successful
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        // Authentication failed
        Err(warp::reject::custom(error_handler::Error::WrongPassword))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Session {}
pub fn basic_auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    let username = dotenv::var("AUTHORIZATION_USER").expect("AUTHORIZATION_USER must be set");
    let password = dotenv::var("AUTHORIZATION_PASSWORD").expect("AUTHORIZATION_PASSWORD must be set");
    warp::header::optional(AUTHORIZATION.as_str()).and_then(move |authorization: Option<String>| {
        let basic_auth = BasicAuth::new(username.clone(), password.clone());
        async move {
            match basic_auth.authenticate(authorization).await {
                Ok(_) => Ok(Session {}),
                Err(error) => Err(error),
            }
        }
    })
}
