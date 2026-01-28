use actix_web::{
    delete, put,
    web::{self, Json},
};
use dtos::UpdateUserDTO;
use services::{DeleteUserService, Service, ServiceError};

use crate::{app::App, guards::AuthenticatedUser};

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
    _dto: Json<UpdateUserDTO>,
    auth: AuthenticatedUser,
) -> http::Response {
    let _user_id = auth.get_id();
    let _service = app.services.get::<DeleteUserService>().await.unwrap();

    //let result = service.run(user_id).await;

    todo!();
    /*match result {
        Ok(_) => http::Response::success(201, &(), &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }*/
}
