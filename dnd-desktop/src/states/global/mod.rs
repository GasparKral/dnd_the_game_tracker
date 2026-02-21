use crate::backend::WsPool;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct AppState {
    pub ws_pool: WsPool,
    pub vault: RwLock<PathBuf>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ws_pool: WsPool::new(),
            vault: RwLock::new(PathBuf::new()),
        }
    }

    pub async fn set_vault(&self, path: PathBuf) {
        *self.vault.write().await = path;
    }

    pub async fn get_vault(&self) -> PathBuf {
        self.vault.read().await.clone()
    }
}

#[derive(Clone)]
pub struct SharedState(pub Arc<AppState>);
