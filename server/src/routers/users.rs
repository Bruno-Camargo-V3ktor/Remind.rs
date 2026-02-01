use actix_web::{
    delete, put,
    web::{self, Json},
};
use dtos::UpdateUserDTO;
use services::{DeleteUserService, Service, ServiceError, UpdateUserService};

use crate::{
    app::App,
    guards::{AuthenticatedEmail, AuthenticatedUser},
};

#[delete("/")]
pub async fn delete_user(app: web::Data<App>, auth: AuthenticatedUser) -> http::Response {
    let user_id = auth.get_id();
    let service = app.services.get::<DeleteUserService>().await.unwrap();

    let result = service.run(user_id).await;

    match result {
        Ok(_) => http::Response::success(200, &(), &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[put("/")]
pub async fn update_user(
    app: web::Data<App>,
    update_dto: Json<UpdateUserDTO>,
    auth_user: Option<AuthenticatedUser>,
    auth_email: Option<AuthenticatedEmail>,
) -> http::Response {
    if auth_user.is_none() && auth_email.is_none() {
        return http::Response::error(
            app.error_code("INVALID_TOKEN".into()),
            "INVALID_TOKEN".into(),
            "authorization token is invalid".into(),
            Option::<String>::None,
        );
    }

    let update_dto = update_dto.0;
    let user_id = auth_user
        .map(|v| v.get_id())
        .unwrap_or_else(|| auth_email.unwrap().get_id());

    let service = app.services.get::<UpdateUserService>().await.unwrap();
    let result = service.run((user_id, update_dto)).await;

    match result {
        Ok(u) => http::Response::success(200, &u, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}
