use crate::backend::pool::WsPool;

#[derive(Debug)]
pub struct AppState {
    // Pool de conexiones
    pub ws_pool: WsPool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ws_pool: WsPool::new(),
        }
    }
}
