mod models;
mod routes;
mod ws;

pub use models::pool::WsPool;

use crate::states::SharedState;
use axum::{routing::any, Router};
use tower_http::cors::CorsLayer;
use tracing::info;

pub async fn run(state: SharedState) {
    let router = Router::new()
        .route("/ws/game", any(ws::handler))
        .nest("/api", routes::api_router())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    info!("Backend escuchando en http://0.0.0.0:3000");

    axum::serve(listener, router).await.expect("Server error");
}
