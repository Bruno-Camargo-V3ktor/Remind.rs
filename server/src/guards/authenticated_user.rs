use std::future::{Ready, ready};

use actix_web::FromRequest;
use domain::models::UserId;
use http::Response;
use security::token::UserToken;

use crate::config::load_config;

pub struct AuthenticatedUser(UserId);

impl AuthenticatedUser {
    pub fn get_id(&self) -> UserId {
        self.0.clone()
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Response;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let key = load_config().security.key;

        if let Some(token_str) = auth_header {
            let token_str = token_str.to_str().unwrap();
            let token = UserToken(token_str.to_string());

            if let Some(user_id) = token.validate(&key) {
                return ready(Ok(AuthenticatedUser(user_id)));
            }
        }

        ready(Err(Response::error(
            401,
            "INVALID_TOKEN".into(),
            "authorization token is invalid".into(),
            Option::<String>::None,
        )))
    }
}
