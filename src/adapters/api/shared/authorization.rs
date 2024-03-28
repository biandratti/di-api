use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    username: String,
    password: String,
}

pub struct Authorization {}

impl Authorization {
    /*fn verify_user(user: &User) -> bool {
        if user.username == "user" && user.password == "pass" {
            true
        } else {
            false
        }
    }

    pub fn verify(user: &User) -> impl Filter<Extract = (), Error = Rejection> + Copy {
        if Self::verify_user(user) {
            Ok(())
        } else {
            Err(custom(CustomRejection { error: None }))
        }
    }*/
}
