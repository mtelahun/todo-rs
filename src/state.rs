use std::sync::Arc;

use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<RwLock<Surreal<Client>>>,
}

impl AppState {
    pub fn new(db: Surreal<Client>) -> Self {
        Self {
            db: Arc::new(RwLock::new(db)),
        }
    }
}
