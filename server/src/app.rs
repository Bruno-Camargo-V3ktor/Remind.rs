#![allow(dead_code)]

use crate::routers::*;
use actix_web::{HttpServer, web};
use config::ConfigApp;
use repository::{note::NoteRepository, property::PropertyRepository, user::UserRepository};
use security::hash::PasswordHash;
use services::{Service, ServiceManager};
use std::{collections::HashMap, io, sync::Arc, time::Duration};

#[derive(Default)]
pub struct AppBuilder {
    services: Option<Arc<ServiceManager>>,
    config: Option<Arc<ConfigApp>>,
    user_repo: Option<Arc<dyn UserRepository + 'static + Send + Sync>>,
    property_repo: Option<Arc<dyn PropertyRepository + 'static + Send + Sync>>,
    note_repo: Option<Arc<dyn NoteRepository + 'static + Send + Sync>>,
    erros_table: Option<Arc<HashMap<String, u16>>>,
    password_hash: Option<Arc<dyn PasswordHash + 'static + Send + Sync>>,
}

impl AppBuilder {
    pub fn services(&mut self, arg: ServiceManager) {
        self.services = Some(Arc::new(arg));
    }

    pub fn config(&mut self, config: ConfigApp) {
        self.config = Some(Arc::new(config));
    }

    pub fn user_repo(&mut self, repo: Arc<impl UserRepository + 'static + Send + Sync>) {
        self.user_repo = Some(repo);
    }

    pub fn note_repo(&mut self, repo: Arc<impl NoteRepository + 'static + Send + Sync>) {
        self.note_repo = Some(repo);
    }

    pub fn property_repo(&mut self, repo: Arc<impl PropertyRepository + 'static + Send + Sync>) {
        self.property_repo = Some(repo);
    }

    pub async fn add_service(&mut self, service: impl Service + 'static) {
        self.services.as_ref().unwrap().register(service).await;
    }

    pub fn add_table_errors_code(&mut self, table: HashMap<String, u16>) {
        self.erros_table = Some(Arc::new(table));
    }

    pub fn password_hash(&mut self, hased: Arc<dyn PasswordHash + 'static + Send + Sync>) {
        self.password_hash = Some(hased);
    }

    pub fn build(self) -> App {
        App {
            services: self.services.expect("not defined services"),
            config: self.config.expect("not defined config"),
            user_repo: self.user_repo.expect("not defined user_repo"),
            property_repo: self.property_repo.expect("not defined property_repo"),
            note_repo: self.note_repo.expect("not defined note_repo"),
            erros_table: self.erros_table.expect("not defined errors table"),
            password_hash: self.password_hash.expect("not defined password_hash"),
        }
    }
}

#[derive(Clone)]
pub struct App {
    pub services: Arc<ServiceManager>,
    pub config: Arc<ConfigApp>,
    pub user_repo: Arc<dyn UserRepository + 'static + Send + Sync>,
    pub property_repo: Arc<dyn PropertyRepository + 'static + Send + Sync>,
    pub note_repo: Arc<dyn NoteRepository + 'static + Send + Sync>,
    pub erros_table: Arc<HashMap<String, u16>>,
    pub password_hash: Arc<dyn PasswordHash + 'static + Send + Sync>,
}

impl App {
    pub async fn new<Fut: Future<Output = App>, F: FnMut(AppBuilder) -> Fut>(
        mut clousure: F,
    ) -> Self {
        let mut builder = AppBuilder::default();
        builder.services(ServiceManager::default());

        clousure(builder).await
    }

    pub async fn run(&self) -> io::Result<()> {
        let app_state = web::Data::new(self.clone());

        HttpServer::new(move || {
            let mut actix_app = actix_web::App::new().app_data(app_state.clone()).service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(register_user)
                            .service(login_user)
                            .service(send_email_password)
                            .service(authenticated_user),
                    )
                    .service(
                        web::scope("users")
                            .service(update_user)
                            .service(delete_user)
                            .service(upload_image),
                    )
                    .service(
                        web::scope("notes")
                            .service(create_note)
                            .service(update_note)
                            .service(delete_note),
                    )
                    .service(
                        web::scope("propertys")
                            .service(create_property)
                            .service(update_property)
                            .service(delete_property),
                    ),
            );

            if let Some(public_dir) = &app_state.config.local_storage.public_dir {
                actix_app = actix_app.service(actix_files::Files::new("/public", public_dir));
            }

            actix_app
        })
        .server_hostname(&self.config.server.hostname)
        .workers(self.config.server.workers)
        .keep_alive(Duration::from_secs(self.config.server.keep_alive_secs))
        .bind(&self.config.server.addr)?
        .run()
        .await?;

        Ok(())
    }

    pub fn error_code(&self, code: String) -> u16 {
        *self.erros_table.get(&code).unwrap_or(&500u16)
    }
}
