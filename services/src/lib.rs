use serde::Serialize;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    error::Error,
    fmt::Debug,
    sync::Arc,
};
use tokio::sync::RwLock;

mod user;
pub use user::*;

#[async_trait::async_trait]
pub trait Service: Sync + Send {
    type Out: Send + Sync;
    type Builder: ServiceBuilder + Sync + Send;

    async fn run(&self) -> Result<Self::Out, impl ServiceError>;
}

pub trait ServiceBuilder: Sync + Send {
    fn type_service(&self) -> TypeId;
}

#[derive(Default)]
pub struct ServiceManager {
    map: RwLock<HashMap<TypeId, Arc<dyn Any + Sync + Send>>>,
}

impl ServiceManager {
    pub async fn register<S: ServiceBuilder + 'static>(&self, service: S) {
        let mut map = self.map.write().await;
        map.insert(service.type_service(), Arc::new(service));
    }

    pub async fn get<S: Service + 'static>(&self) -> Option<Arc<S::Builder>> {
        let map = self.map.read().await;

        if let Some(service_any) = map.get(&TypeId::of::<S>()) {
            let s = service_any.clone();
            return s.downcast().ok();
        }

        None
    }
}

pub trait ServiceError: Error + Debug {
    fn code(&self) -> String;
    fn description(&self) -> String;
    fn content(&self) -> &impl Serialize;
}
