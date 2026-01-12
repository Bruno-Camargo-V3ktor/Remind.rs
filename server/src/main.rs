use crate::app::AppState;
use actix_web::{App, HttpServer, Responder, get, web};
use repository::{
    note::NoteInMemoryRepository, property::PropertyInMemoryRepository,
    user::UserInMemoryRepository,
};
use services::CreateUserService;
use std::io;

mod app;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app_state = AppState::new(
        UserInMemoryRepository::new(vec![]),
        PropertyInMemoryRepository::new(vec![]),
        NoteInMemoryRepository::new(vec![]),
        |app| async {
            app.services
                .register(CreateUserService::builder(app.user_repo.clone()))
                .await;

            app
        },
    )
    .await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(index)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello"
}
