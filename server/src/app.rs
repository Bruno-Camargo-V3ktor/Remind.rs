use std::sync::Arc;

use repository::{
    note::NoteRepository,
    property::{self, PropertyRepository},
    user::UserRepository,
};
use services::ServiceManager;

#[derive(Clone)]
pub struct AppState {
    pub services: Arc<ServiceManager>,
    pub user_repo: Arc<dyn UserRepository + 'static + Send + Sync>,
    pub property_repo: Arc<dyn PropertyRepository + 'static + Send + Sync>,
    pub note_repo: Arc<dyn NoteRepository + 'static + Send + Sync>,
}

impl AppState {
    pub async fn new<Fut: Future<Output = AppState>, F: FnMut(AppState) -> Fut>(
        user_repo: impl UserRepository + 'static + Send + Sync,
        property_repo: impl PropertyRepository + 'static + Send + Sync,
        note_repo: impl NoteRepository + 'static + Send + Sync,
        mut clousure: F,
    ) -> Self {
        let state = Self {
            services: Arc::new(ServiceManager::new()),
            user_repo: Arc::new(user_repo),
            property_repo: Arc::new(property_repo),
            note_repo: Arc::new(note_repo),
        };

        clousure(state).await
    }
}
