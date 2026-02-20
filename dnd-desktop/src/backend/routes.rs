use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::states::AppState;

pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        // Personajes
        .route("/characters", get(get_all_characters))
        .route("/characters/{:id}", get(get_character))
        // Combate
        .route("/combat", get(get_combat_state))
        // Lore / Obsidian vault
        .route("/lore", get(get_lore_index))
        .route("/lore/{*path}", get(get_lore_entry))
}

// GET /api/characters
async fn get_all_characters(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

// GET /api/characters/:id
async fn get_character(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
}

// GET /api/combat
async fn get_combat_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

// GET /api/lore  →  índice de todas las notas del vault
async fn get_lore_index(State(state): State<Arc<AppState>>) -> impl IntoResponse {}

// GET /api/lore/Lugares/Taberna del Dragón
async fn get_lore_entry(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
}
