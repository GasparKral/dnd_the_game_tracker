mod models;
mod routes;
mod ws;

use crate::states::AppState;
pub use models::*;

use axum::{routing::any, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub async fn run(state: Arc<AppState>) {
    let router = Router::new()
        .route("/ws/game", any(ws::handler))
        .nest("/api", routes::api_router())
        .layer(CorsLayer::permissive())
        .with_state::<AppState>(state);
}
