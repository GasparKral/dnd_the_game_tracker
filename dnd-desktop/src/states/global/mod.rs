use crate::backend::{registry::Registry, WsPool};
use crate::persistence::PersistenceManager;
use crate::vault::VaultManager;
use shared::api_types::character_draft::CharacterDraft;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct AppState {
    pub ws_pool: WsPool,
    pub vault: VaultManager,
    pub registry: Registry,
    pub persistence: PersistenceManager,
    /// Drafts de personaje en curso, indexados por draft_id
    pub drafts: RwLock<HashMap<Uuid, CharacterDraft>>,
}

impl AppState {
    pub fn new(data_dir: impl AsRef<std::path::Path>) -> Self {
        Self {
            ws_pool: WsPool::new(),
            vault: VaultManager::new(),
            registry: Registry::new(),
            persistence: PersistenceManager::new(data_dir),
            drafts: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Clone)]
pub struct SharedState(pub Arc<AppState>);
