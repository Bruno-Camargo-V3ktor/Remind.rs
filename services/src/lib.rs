use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use thiserror::Error;
use tokio::sync::RwLock;

pub type ServiceResult<T> = Result<T, ServiceError>;

#[async_trait::async_trait]
pub trait Service: Sync + Send {
    type Input: Sync + Send;
    type Result: Sync + Send;

    async fn run(&self, dto: Self::Input) -> ServiceResult<Self::Result>;
}

pub struct ServiceManager {
    map: RwLock<HashMap<TypeId, Arc<dyn Any + Sync + Send>>>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub async fn register<S: Service + 'static>(&self, service: S) {
        let mut map = self.map.write().await;
        map.insert(TypeId::of::<S>(), Arc::new(service));
    }

    pub async fn get<S: Service + 'static>(&self) -> Option<Arc<S>> {
        let map = self.map.read().await;

        if let Some(service_any) = map.get(&TypeId::of::<S>()) {
            let s = service_any.clone();
            return s.downcast().ok();
        }

        None
    }
}

#[derive(Error, Debug)]
pub enum ServiceError {}
