use actix_web::{
    HttpResponseBuilder, Responder,
    http::StatusCode,
    post,
    web::{self, Json},
};
use dtos::CreateUserDTO;
use services::{CreateUserService, Service};

use crate::app::App;

#[post("/users")]
pub async fn register_user(app: web::Data<App>, create_dto: Json<CreateUserDTO>) -> http::Response {
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
            todo!()
        }
    }

    /*if let Ok(user) = &result {
        return http.json(user);
    }

    match result.as_ref().err().unwrap() {
        CreateUserError::FieldsError(list) => {
            http.status(StatusCode::BAD_REQUEST);
            http.json(list)
        }
        CreateUserError::EmailRegistered(err) => {
            http.status(StatusCode::BAD_REQUEST);
            http.json(err)
        }
        CreateUserError::RepositoryError(err) => {
            http.status(StatusCode::INTERNAL_SERVER_ERROR);
            http.json(err)
        }
        CreateUserError::Unknown => {
            http.status(StatusCode::INTERNAL_SERVER_ERROR);
            http.json("Unknow Error")
        }
    }*/
}
