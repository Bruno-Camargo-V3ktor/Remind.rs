use actix_web::{post, web};
use dtos::{CreateUserDTO, LoginUserDTO};
use security::token::UserToken;
use services::{CreateUserService, LoginUserService, Service, ServiceError};

use crate::app::App;

#[post("/users")]
pub async fn register_user(
    app: web::Data<App>,
    create_dto: web::Json<CreateUserDTO>,
) -> http::Response {
    let dto = create_dto.0;
    let service = app.services.get::<CreateUserService>().await.unwrap();

    let result = service.run(dto).await;

    match result {
        Ok(user) => {
            let token = UserToken::new(&app.config.security.key, 1, user.id.clone());
            http::Response::success(201, &token, &app.config.server.api_version)
        }

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[post("/users/login")]
pub async fn login_user(
    app: web::Data<App>,
    create_dto: web::Json<LoginUserDTO>,
) -> http::Response {
    let dto = create_dto.0;
    let service = app.services.get::<LoginUserService>().await.unwrap();

    let result = service.run((dto.email, dto.password)).await;

    match result {
        Ok(id) => {
            let token = UserToken::new(&app.config.security.key, 1, id.clone());
            http::Response::success(200, &token, &app.config.server.api_version)
        }

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}
