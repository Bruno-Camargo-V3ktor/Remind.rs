use crate::{app::App, guards::AuthenticatedUser};
use actix_web::{get, post, web};
use dtos::{CreateUserDTO, LoginUserDTO};
use http::Response;
use security::token::UserToken;
use serde::Deserialize;
use services::{
    CreateUserService, InfoUserService, LoginUserService, SendEmailService, Service, ServiceError,
    To,
};

#[derive(Deserialize)]
struct ResetPasswordInfo {
    email: String,
    url: String,
}

#[get("/user")]
pub async fn authenticated_user(app: web::Data<App>, auth: AuthenticatedUser) -> http::Response {
    let user_id = auth.get_id();
    let service = app.services.get::<InfoUserService>().await.unwrap();

    let result = service.run(user_id).await;

    match result {
        Ok(info) => http::Response::success(200, &info, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[post("/register")]
pub async fn register_user(
    app: web::Data<App>,
    create_dto: web::Json<CreateUserDTO>,
) -> http::Response {
    let dto = create_dto.0;
    let service = app.services.get::<CreateUserService>().await.unwrap();

    let result = service.run(dto).await;

    match result {
        Ok(user) => {
            let token = UserToken::new(&app.config.security.users_key, 1, user.id.clone());
            http::Response::success(201, &token, &app.config.server.api_version)
        }

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[post("/login")]
pub async fn login_user(app: web::Data<App>, login_dto: web::Json<LoginUserDTO>) -> http::Response {
    let dto = login_dto.0;
    let service = app.services.get::<LoginUserService>().await.unwrap();

    let result = service.run((dto.email, dto.password)).await;

    match result {
        Ok(id) => {
            let token = UserToken::new(&app.config.security.users_key, 1, id.clone());
            http::Response::success(200, &token, &app.config.server.api_version)
        }

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[post("/reset-password")]
pub async fn send_email_password(
    app: web::Data<App>,
    info: web::Query<ResetPasswordInfo>,
) -> http::Response {
    let service = app.services.get::<SendEmailService>().await.unwrap();
    let result = app.user_repo.get_by_email(info.email.clone()).await;

    match result {
        Ok(u) => {
            let token = UserToken::new(&app.config.security.reset_key, 1, u.id);
            let url = info.url.clone();

            let args = (
                To {
                    name: u.name,
                    email: u.email,
                },
                "Reset Password".to_string(),
                format!(
                    "<h3>Click for Reset your Password</h3> <a href=\"{}?t={}\">Reset Password</a>",
                    url, token.0
                ),
            );

            match service.run(args).await {
                Ok(()) => Response::success(200, &(), &app.config.server.api_version),
                Err(err) => {
                    let status_code = app.error_code(err.code());
                    http::Response::error(status_code, err.code(), err.description(), &err)
                }
            }
        }

        Err(_) => Response::success(200, &(), &app.config.server.api_version),
    }
}
