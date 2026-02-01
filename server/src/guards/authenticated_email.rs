use crate::app::App;
use actix_web::{FromRequest, web};
use domain::models::UserId;
use http::Response;
use security::token::UserToken;
use std::future::{Ready, ready};

pub struct AuthenticatedEmail(UserId);

impl AuthenticatedEmail {
    pub fn get_id(&self) -> UserId {
        self.0.clone()
    }
}

impl FromRequest for AuthenticatedEmail {
    type Error = Response;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let app_state = req.app_data::<web::Data<App>>().unwrap();
        let auth_header = req.headers().get("Authorization");
        let key = &app_state.config.security.reset_key;

        if let Some(token_str) = auth_header {
            let token_str = token_str.to_str().unwrap();
            let token = UserToken(token_str.to_string());

            if let Some(user_id) = token.validate(key) {
                return ready(Ok(AuthenticatedEmail(user_id)));
            }
        }

        ready(Err(Response::error(
            app_state.error_code("INVALID_TOKEN".into()),
            "INVALID_TOKEN".into(),
            "authorization token is invalid".into(),
            Option::<String>::None,
        )))
    }
}
