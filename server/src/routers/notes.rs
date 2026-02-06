use std::str::FromStr;

use crate::{app::App, guards::AuthenticatedUser};
use actix_web::{
    delete, post, put,
    web::{self, Json},
};
use domain::models::NoteId;
use dtos::{CreateNoteDTO, UpdateNoteDTO};
use services::{CreateNoteService, DeleteNoteService, Service, ServiceError, UpdateNoteService};

#[post("/")]
pub async fn create_note(
    app: web::Data<App>,
    auth: AuthenticatedUser,
    create_note: Json<CreateNoteDTO>,
) -> http::Response {
    let user_id = auth.get_id();
    let dto = create_note.0;
    let service = app.services.get::<CreateNoteService>().await.unwrap();

    let result = service.run((user_id, dto)).await;

    match result {
        Ok(n) => http::Response::success(200, &n, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[put("/{note_id}")]
pub async fn update_note(
    app: web::Data<App>,
    auth: AuthenticatedUser,
    update_note: Json<UpdateNoteDTO>,
    note_id_str: String,
) -> http::Response {
    let user_id = auth.get_id();
    let dto = update_note.0;
    let service = app.services.get::<UpdateNoteService>().await.unwrap();

    let note_id = match NoteId::from_str(&note_id_str) {
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

    let result = service.run((user_id, note_id, dto)).await;

    match result {
        Ok(n) => http::Response::success(200, &n, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}

#[delete("/{note_id}")]
pub async fn delete_note(
    app: web::Data<App>,
    _auth: AuthenticatedUser,
    note_id_str: String,
) -> http::Response {
    let service = app.services.get::<DeleteNoteService>().await.unwrap();

    let note_id = match NoteId::from_str(&note_id_str) {
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

    let result = service.run(note_id).await;

    match result {
        Ok(n) => http::Response::success(200, &n, &app.config.server.api_version),

        Err(err) => {
            let status_code = app.error_code(err.code());
            http::Response::error(status_code, err.code(), err.description(), &err)
        }
    }
}
