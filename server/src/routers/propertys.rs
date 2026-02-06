use std::str::FromStr;

use crate::{app::App, guards::AuthenticatedUser};
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
};
use domain::models::PropertyId;
use dtos::{CreatePropertyDTO, UpdatePropertyDTO};
use services::{
    CreatePropertyService, DeletePropertyService, ListPropertyService, Service, ServiceError,
    UpdatePropertyService,
};

#[get("/")]
pub async fn list_propertys(app: web::Data<App>, auth: AuthenticatedUser) -> http::Response {
    let user_id = auth.get_id();
    let service = app.services.get::<ListPropertyService>().await.unwrap();

    let result = service.run(user_id).await;

    match result {
        Ok(list) => http::Response::success(200, &list, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[post("/")]
pub async fn create_property(
    app: web::Data<App>,
    auth: AuthenticatedUser,
    create_property: Json<CreatePropertyDTO>,
) -> http::Response {
    let user_id = auth.get_id();
    let dto = create_property.0;
    let service = app.services.get::<CreatePropertyService>().await.unwrap();

    let result = service.run((user_id, dto)).await;

    match result {
        Ok(n) => http::Response::success(200, &n, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[put("/{property_id_str}")]
pub async fn update_property(
    app: web::Data<App>,
    auth: AuthenticatedUser,
    update_property: Json<UpdatePropertyDTO>,
    property_id_str: String,
) -> http::Response {
    let user_id = auth.get_id();
    let dto = update_property.0;
    let service = app.services.get::<UpdatePropertyService>().await.unwrap();

    let property_id = match PropertyId::from_str(&property_id_str) {
        Ok(id) => id,
        Err(_) => {
            return http::Response::error(
                400,
                "INVALID_FIELDS".into(),
                "invalid uuid in request".into(),
                (),
            );
        }
    };

    let result = service.run((user_id, property_id, dto)).await;

    match result {
        Ok(n) => http::Response::success(200, &n, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[delete("/{property_id_str}")]
pub async fn delete_property(
    app: web::Data<App>,
    _auth: AuthenticatedUser,
    property_id_str: String,
) -> http::Response {
    let service = app.services.get::<DeletePropertyService>().await.unwrap();

    let property_id = match PropertyId::from_str(&property_id_str) {
        Ok(id) => id,
        Err(_) => {
            return http::Response::error(
                400,
                "INVALID_FIELDS".into(),
                "invalid uuid in request".into(),
                (),
            );
        }
    };

    let result = service.run(property_id).await;

    match result {
        Ok(n) => http::Response::success(200, &n, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}
