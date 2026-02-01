use crate::{
    app::App,
    guards::{AuthenticatedEmail, AuthenticatedUser},
    utils::file::read_image_from_multipart,
};
use actix_multipart::Multipart;
use actix_web::{
    delete, post, put,
    web::{self, Json},
};
use chrono::{Duration, Utc};
use dtos::UpdateUserDTO;
use services::{
    DeleteUserService, FileAction, FileService, Service, ServiceError, UpdateUserService,
};

const MAX_IMAGE_SIZE: usize = 1024 * 1500;

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

#[post("/image")]
pub async fn upload_image(
    app: web::Data<App>,
    auth: AuthenticatedUser,
    mut payload: Multipart,
) -> http::Response {
    let user_id = auth.get_id();

    let user = match app.user_repo.get_by_id(user_id).await {
        Ok(user) => user,
        Err(_) => {
            return http::Response::error(
                400,
                "INVALID_FIELDS".to_string(),
                "user not found.".to_string(),
                (),
            );
        }
    };

    if Utc::now().signed_duration_since(user.updated_at) >= Duration::minutes(2) {
        return http::Response::error(
            400,
            "INVALID_FIELDS".to_string(),
            "expiration time for upload.".to_string(),
            (),
        );
    }

    let (metadata, file_bytes) = match read_image_from_multipart(&mut payload, MAX_IMAGE_SIZE).await
    {
        Ok(value) => value,
        Err(msg) => {
            return http::Response::error(400, "INVALID_FIELDS".to_string(), msg, ());
        }
    };
    let filename = match metadata.get("filename") {
        Some(f) => f,
        None => {
            return http::Response::error(
                400,
                "INVALID_FIELDS".to_string(),
                "not metadata found in multipart.".to_string(),
                (),
            );
        }
    };

    if user.photo_url != *filename {
        return http::Response::error(
            400,
            "INVALID_FIELDS".to_string(),
            "filename not equal for database register.".to_string(),
            (),
        );
    }

    let file_service = app.services.get::<FileService>().await.unwrap();

    let result = file_service
        .run(FileAction::Save {
            bytes: file_bytes,
            dst: format!("public/images/{filename}"),
        })
        .await;

    match result {
        Ok(_) => http::Response::success(200, &(), &app.config.server.api_version),
        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}
