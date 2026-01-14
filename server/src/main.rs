use crate::{app::App, config::ConfigApp};
use figment::{
    Figment,
    providers::{Env, Format, Json},
};
use repository::{
    note::NoteInMemoryRepository, property::PropertyInMemoryRepository,
    user::UserInMemoryRepository,
};
use services::CreateUserService;
use std::sync::Arc;

mod app;
mod config;
mod routers;

#[actix_web::main]
async fn main() {
    let app = App::new(|mut app| async {
        let _ = dotenv::dotenv();

        let config: ConfigApp = Figment::new()
            .merge(Env::prefixed("REMIND_").split("__"))
            .merge(Json::file("config.json"))
            .extract()
            .expect("Invalid Configuration");

        app.config(config);

        let user_repo = Arc::new(UserInMemoryRepository::new(vec![]));
        let property_repo = Arc::new(PropertyInMemoryRepository::new(vec![]));
        let note_repo = Arc::new(NoteInMemoryRepository::new(vec![]));

        app.add_service(CreateUserService::builder(user_repo.clone()))
            .await;

        app.user_repo(user_repo);
        app.property_repo(property_repo);
        app.note_repo(note_repo);

        app.build()
    })
    .await;

    let _ = app.run().await;
}
