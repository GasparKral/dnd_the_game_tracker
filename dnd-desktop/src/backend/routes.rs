use crate::states::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::path::PathBuf;
use tracing::{debug, info};
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

async fn get_all_characters(State(state): State<SharedState>) -> impl IntoResponse {
    axum::http::StatusCode::NOT_IMPLEMENTED
}
async fn get_character(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    axum::http::StatusCode::NOT_IMPLEMENTED
}
async fn get_combat_state(State(state): State<SharedState>) -> impl IntoResponse {
    axum::http::StatusCode::NOT_IMPLEMENTED
}

async fn get_lore_index(State(state): State<SharedState>) -> impl IntoResponse {
    let vault_path = state.0.get_vault().await;

    // Si el vault no está configurado todavía
    if vault_path.as_os_str().is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "Vault no configurado" })),
        )
            .into_response();
    }

    // Recorrer el vault buscando .md
    let entries: Vec<LoreIndexEntry> = walkdir::WalkDir::new(&vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().map_or(false, |ext| ext == "md")
                && e.path().to_string_lossy().to_string().contains("/Lore")
        })
        .filter_map(|e| {
            let relative = e.path().strip_prefix(&vault_path).ok()?;
            let path = relative.to_string_lossy().to_string();
            let title = relative
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            Some(LoreIndexEntry { path, title })
        })
        .collect();

    (StatusCode::OK, Json(entries)).into_response()
}

// GET /api/lore/Lugares/Taberna  →  contenido de esa nota
async fn get_lore_entry(
    State(state): State<SharedState>,
    Path(path): Path<String>, // "Lugares/Taberna" o "Lugares/Taberna.md"
) -> impl IntoResponse {
    info!("Petición lore entry: {}", path);

    let vault_path = state.0.get_vault().await;

    if vault_path.as_os_str().is_empty() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "Vault no configurado" })),
        )
            .into_response();
    }

    // Construir la ruta — añadimos .md si no lo trae el cliente
    let relative = if path.ends_with(".md") {
        PathBuf::from(&path)
    } else {
        PathBuf::from(format!("{}.md", path))
    };

    let full_path = vault_path.join(&relative);

    // Leer el fichero
    match tokio::fs::read_to_string(&full_path).await {
        Ok(content) => {
            let title = relative
                .file_stem() // "Taberna"
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            (
                StatusCode::OK,
                Json(LoreEntry {
                    path: path.clone(),
                    title,
                    content,
                }),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("Nota '{}' no encontrada en el vault", path)
            })),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
struct LoreEntry {
    path: String,    // ruta relativa dentro del vault
    title: String,   // nombre del fichero sin extensión
    content: String, // contenido markdown en crudo
}

/// Entrada ligera del índice — sin contenido, solo metadatos para listar.
#[derive(Serialize)]
struct LoreIndexEntry {
    path: String,  // ruta relativa dentro del vault (ej: "Lore/Lugares/Taberna.md")
    title: String, // nombre del fichero sin extensión (ej: "Taberna")
}
