use actix_web::{
    HttpResponse, HttpResponseBuilder, Responder,
    http::StatusCode,
    post,
    web::{self, Json},
};
use dtos::CreateUserDTO;
use services::{CreateUserError, CreateUserService, Service};

use crate::app::App;

#[post("/users")]
pub async fn register_user(app: web::Data<App>, create_dto: Json<CreateUserDTO>) -> impl Responder {
    let dto = create_dto.0;
    let service = app
        .services
        .get::<CreateUserService>()
        .await
        .unwrap()
        .build(dto);

    let result = service.run().await;
    let mut http = HttpResponseBuilder::new(StatusCode::CREATED);

    if let Ok(user) = &result {
        return http.json(user);
    }

    match result.as_ref().err().unwrap() {
        CreateUserError::FieldsError(list) => {
            http.status(StatusCode::BAD_REQUEST);
            return http.json(list);
        }
        CreateUserError::EmailRegistered(err) => {
            http.status(StatusCode::BAD_REQUEST);
            return http.json(err);
        }
        CreateUserError::RepositoryError(err) => {
            http.status(StatusCode::INTERNAL_SERVER_ERROR);
            return http.json(err);
        }
        CreateUserError::Unknown => {
            http.status(StatusCode::INTERNAL_SERVER_ERROR);
            return http.json("Unknow Error");
        }
    }
}
