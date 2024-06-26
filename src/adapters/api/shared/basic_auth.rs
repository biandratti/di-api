use crate::adapters::api::shared::error_handler;
use serde::{Deserialize, Serialize};
use warp::http::header::AUTHORIZATION;
use warp::{Filter, Rejection};

struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    fn new(username: String, password: String) -> Self {
        BasicAuth { username, password }
    }

    async fn authenticate(&self, authorization: Option<String>) -> Result<(), Rejection> {
        let (u, p) = authorization
            .and_then(|auth| {
                auth.strip_prefix("Basic ")
                    .map(|cred| base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cred).ok())
            })
            .flatten()
            .and_then(|decoded| String::from_utf8(decoded).ok())
            .and_then(|decoded_str| {
                let mut parts = decoded_str.splitn(2, ':');
                match (parts.next().map(|s| s.to_owned()), parts.next().map(|s| s.to_owned())) {
                    (Some(u), Some(p)) => Some((u, p)),
                    _ => None,
                }
            })
            .unwrap_or_else(|| ("".to_owned(), "".to_owned()));

        if u == self.username && p == self.password {
            Ok(())
        } else {
            Err(warp::reject::custom(error_handler::Error::WrongPassword))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Session {}
pub fn basic_auth() -> impl Filter<Extract = (Session,), Error = Rejection> + Clone {
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
