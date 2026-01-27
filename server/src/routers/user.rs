use actix_web::{post, web};
use dtos::CreateUserDTO;
use security::token::UserToken;
use services::{CreateUserService, Service, ServiceError};

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
