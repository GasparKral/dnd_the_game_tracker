use crate::states::SharedState; // ← esto, no Arc<AppState>
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use uuid::Uuid;

pub fn api_router() -> Router<SharedState> {
    // ← SharedState aquí
    Router::new()
        .route("/characters", get(get_all_characters))
        .route("/characters/{:id}", get(get_character))
        .route("/combat", get(get_combat_state))
        .route("/lore", get(get_lore_index))
        .route("/lore/{*path}", get(get_lore_entry))
}

async fn get_all_characters(State(state): State<SharedState>) -> impl IntoResponse {}
async fn get_character(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
}
async fn get_combat_state(State(state): State<SharedState>) -> impl IntoResponse {}
async fn get_lore_index(State(state): State<SharedState>) -> impl IntoResponse {}
async fn get_lore_entry(
    State(state): State<SharedState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
}
