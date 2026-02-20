use super::messages::ServerMessage;
use std::collections::HashMap;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

const BROADCAST_CAPACITY: usize = 256;

#[derive(Debug)]
pub struct WsPool {
    /// Jugadores conectados: id → nombre de personaje
    players: RwLock<HashMap<Uuid, String>>,
    /// Canal broadcast para todos los jugadores
    broadcast_tx: broadcast::Sender<ServerMessage>,
}

impl WsPool {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(BROADCAST_CAPACITY);
        Self {
            players: RwLock::new(HashMap::new()),
            broadcast_tx,
        }
    }

    pub async fn add(&self, id: Uuid, character_name: String) {
        self.players.write().await.insert(id, character_name);
    }

    pub async fn remove(&self, id: &Uuid) {
        self.players.write().await.remove(id);
    }

    pub async fn connected_players(&self) -> HashMap<Uuid, String> {
        self.players.read().await.clone()
    }

    pub async fn player_count(&self) -> usize {
        self.players.read().await.len()
    }

    /// Enviar a todos los jugadores conectados
    pub fn broadcast(&self, msg: ServerMessage) {
        // Si no hay receivers activos simplemente se descarta
        let _ = self.broadcast_tx.send(msg);
    }

    /// Cada handler WS llama a esto para obtener su receiver
    pub fn subscribe(&self) -> broadcast::Receiver<ServerMessage> {
        self.broadcast_tx.subscribe()
    }
}
