use actix_web::{post, web};
use dtos::CreateUserDTO;
use services::{CreateUserService, Service, ServiceError};

use crate::app::App;

#[post("/users")]
pub async fn register_user(
    app: web::Data<App>,
    create_dto: web::Json<CreateUserDTO>,
) -> http::Response {
    let dto = create_dto.0;
    let service = app
        .services
        .get::<CreateUserService>()
        .await
        .unwrap()
        .build(dto);

    let result = service.run().await;

    match result {
        Ok(user) => http::Response::success(201, &user, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}
