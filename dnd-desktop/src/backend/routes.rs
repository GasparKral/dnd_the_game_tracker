use crate::states::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use shared::api_types::character_draft::{
    CharacterDraft, CreateDraftRequest, DraftResponse, DraftStatusResponse, UpdateDraftRequest,
};
use tracing::info;
use uuid::Uuid;

pub fn api_router() -> Router<SharedState> {
    Router::new()
        // --- Catálogos ---
        .route("/catalog/races", get(get_races_catalog))
        .route("/catalog/classes", get(get_classes_catalog))
        .route("/catalog/backgrounds", get(get_backgrounds_catalog))
        .route("/catalog/feats", get(get_feats_catalog))
        // --- Creación de personaje (draft) ---
        .route("/character/draft", post(create_draft))
        .route("/character/draft/{id}", get(get_draft))
        .route("/character/draft/{id}", put(update_draft))
        // --- Personajes finalizados ---
        .route("/characters", get(get_all_characters))
        .route("/characters/{id}", get(get_character))
        // --- Inventario ---
        .route("/characters/{id}/inventory", get(get_inventory).post(add_item))
        .route("/characters/{id}/inventory/{item_id}", put(update_item).delete(delete_item))
        .route("/characters/{id}/currency", put(update_currency))
        // --- Hechizos ---
        .route("/characters/{id}/spells", get(get_spells).post(add_known_spell))
        .route("/characters/{id}/spells/{spell_id}", delete(remove_known_spell))
        .route("/characters/{id}/spells/{spell_id}/toggle_prepared", post(toggle_prepared_spell))
        .route("/characters/{id}/spell_slots", put(update_spell_slots))
        // --- Items del vault (dnd_type: item) ---
        .route("/vault/items", get(get_vault_items))
        // --- Campaña ---
        .route("/campaign", get(get_campaign).post(create_campaign))
        .route("/campaigns", get(list_campaigns))
        .route("/campaigns/{filename}", get(load_campaign_by_file).delete(delete_campaign))
        // --- Combate ---
        .route("/combat", get(get_combat_state))
        // --- Lore ---
        .route("/lore", get(get_lore_index))
        .route("/lore/{*path}", get(get_lore_entry))
        // --- Assets (imágenes del vault) ---
        .route("/assets/image/{*path}", get(get_vault_image))
}

// ===========================================================================
// Catálogos
// ===========================================================================

async fn get_races_catalog(State(state): State<SharedState>) -> impl IntoResponse {
    let catalog = state.0.registry.races_catalog().await;
    (StatusCode::OK, Json(catalog))
}

async fn get_classes_catalog(State(state): State<SharedState>) -> impl IntoResponse {
    let catalog = state.0.registry.classes_catalog().await;
    (StatusCode::OK, Json(catalog))
}

async fn get_backgrounds_catalog(State(state): State<SharedState>) -> impl IntoResponse {
    let catalog = state.0.registry.backgrounds_catalog().await;
    (StatusCode::OK, Json(catalog))
}

async fn get_feats_catalog(State(state): State<SharedState>) -> impl IntoResponse {
    let catalog = state.0.registry.feats_catalog().await;
    (StatusCode::OK, Json(catalog))
}

// ===========================================================================
// Draft — creación de personaje paso a paso
// ===========================================================================

/// POST /api/character/draft
/// Inicia un nuevo draft y devuelve el draft_id.
async fn create_draft(
    State(state): State<SharedState>,
    Json(req): Json<CreateDraftRequest>,
) -> impl IntoResponse {
    let draft_id = Uuid::new_v4();

    let draft = CharacterDraft {
        draft_id: Some(draft_id),
        ..Default::default()
    };

    state.0.drafts.write().await.insert(draft_id, draft.clone());

    info!(
        "Draft creado: {} para jugador '{}'",
        draft_id, req.player_name
    );

    (
        StatusCode::CREATED,
        Json(DraftResponse {
            draft,
            errors: vec![],
            finalized: false,
        }),
    )
}

/// GET /api/character/draft/{id}
/// Devuelve el estado actual de un draft.
async fn get_draft(State(state): State<SharedState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match state.0.drafts.read().await.get(&id).cloned() {
        Some(draft) => {
            let is_complete = draft.step.is_complete();
            (
                StatusCode::OK,
                Json(DraftStatusResponse { draft, is_complete }),
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Draft {} no encontrado", id) })),
        )
            .into_response(),
    }
}

/// PUT /api/character/draft/{id}
/// Actualiza el draft con los datos del paso actual, valida y avanza al siguiente.
async fn update_draft(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDraftRequest>,
) -> impl IntoResponse {
    let mut drafts = state.0.drafts.write().await;

    let Some(draft) = drafts.get_mut(&id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Draft {} no encontrado", id) })),
        )
            .into_response();
    };

    // Aplicar los cambios del paso actual
    let errors = apply_step(draft, &req, &state).await;

    if errors.is_empty() {
        // Avanzar al siguiente paso solo si no hay errores
        draft.step = draft.step.next();
    }

    let finalized = draft.step.is_complete();

    // Auto-save: cuando el wizard termina, persistir el personaje en la campaña
    if finalized && errors.is_empty() {
        if let Some(saved) = build_saved_character(&req, draft) {
            if let Err(e) = state.0.persistence.upsert_character(saved).await {
                tracing::warn!("No se pudo guardar el personaje: {}", e);
            }
        }
    }

    let response = DraftResponse {
        draft: draft.clone(),
        errors,
        finalized,
    };

    (StatusCode::OK, Json(response)).into_response()
}

/// Construye un SavedCharacter desde un draft finalizado.
/// Devuelve None si faltan datos obligatorios.
fn build_saved_character(
    req: &UpdateDraftRequest,
    draft: &CharacterDraft,
) -> Option<shared::persistence::SavedCharacter> {
    let id = draft.draft_id?;
    let name = draft.name.clone()?;
    let race_id = draft.race_id.clone()?;
    let class_id = draft.class_id.clone()?;
    let background_id = draft.background_id.clone()?;
    let attributes = draft.attributes.clone()?;

    Some(shared::persistence::SavedCharacter::from_finalized_draft(
        req.player_name.clone().unwrap_or_else(|| "Jugador".into()),
        id,
        name,
        race_id,
        class_id,
        background_id,
        attributes,
        draft.feat_ids.clone(),
        draft.choices.clone(),
    ))
}

/// Aplica los datos del request al draft y devuelve errores de validación.
async fn apply_step(
    draft: &mut CharacterDraft,
    req: &UpdateDraftRequest,
    state: &SharedState,
) -> Vec<String> {
    let mut errors = Vec::new();

    match req.step {
        shared::api_types::character_draft::CreationStep::Name => match &req.name {
            Some(name) if !name.trim().is_empty() => {
                draft.name = Some(name.trim().to_string());
            }
            _ => errors.push("El nombre del personaje es obligatorio.".into()),
        },

        shared::api_types::character_draft::CreationStep::Race => {
            match &req.race_id {
                Some(id) => {
                    if state.0.registry.get_race(id).await.is_some() {
                        draft.race_id = Some(id.clone());
                        // Guardar los choices de raza
                        for (k, v) in &req.choices {
                            draft.choices.insert(k.clone(), v.clone());
                        }
                    } else {
                        errors.push(format!("Raza '{}' no encontrada en el catálogo.", id));
                    }
                }
                None => errors.push("Debes elegir una raza.".into()),
            }
        }

        shared::api_types::character_draft::CreationStep::Class => match &req.class_id {
            Some(id) => {
                if state.0.registry.get_class(id).await.is_some() {
                    draft.class_id = Some(id.clone());
                    for (k, v) in &req.choices {
                        draft.choices.insert(k.clone(), v.clone());
                    }
                } else {
                    errors.push(format!("Clase '{}' no encontrada en el catálogo.", id));
                }
            }
            None => errors.push("Debes elegir una clase.".into()),
        },

        shared::api_types::character_draft::CreationStep::Attributes => match &req.attributes {
            Some(attrs) => {
                let cost = attrs.point_buy_cost();
                if cost > 27 {
                    errors.push(format!(
                        "El coste total de atributos es {} puntos (máximo 27).",
                        cost
                    ));
                } else {
                    draft.attributes = Some(attrs.clone());
                }
            }
            None => errors.push("Debes asignar los atributos.".into()),
        },

        shared::api_types::character_draft::CreationStep::Background => match &req.background_id {
            Some(id) => {
                if state.0.registry.get_background(id).await.is_some() {
                    draft.background_id = Some(id.clone());
                    for (k, v) in &req.choices {
                        draft.choices.insert(k.clone(), v.clone());
                    }
                } else {
                    errors.push(format!("Trasfondo '{}' no encontrado en el catálogo.", id));
                }
            }
            None => errors.push("Debes elegir un trasfondo.".into()),
        },

        shared::api_types::character_draft::CreationStep::Feats => {
            // Los dones son opcionales en este paso — se guardan tal cual
            draft.feat_ids = req.feat_ids.clone();
            for (k, v) in &req.choices {
                draft.choices.insert(k.clone(), v.clone());
            }
        }

        shared::api_types::character_draft::CreationStep::Review => {
            // El paso Review no modifica datos — solo verifica que el draft esté completo
            let mut missing = Vec::new();
            if draft.name.is_none() {
                missing.push("nombre");
            }
            if draft.race_id.is_none() {
                missing.push("raza");
            }
            if draft.class_id.is_none() {
                missing.push("clase");
            }
            if draft.attributes.is_none() {
                missing.push("atributos");
            }
            if draft.background_id.is_none() {
                missing.push("trasfondo");
            }
            if !missing.is_empty() {
                errors.push(format!(
                    "Faltan datos obligatorios: {}.",
                    missing.join(", ")
                ));
            }
        }

        shared::api_types::character_draft::CreationStep::Complete => {
            // Ya está completo, no hay nada que actualizar
        }
    }

    errors
}

// ===========================================================================
// Personajes finalizados
// ===========================================================================

/// GET /api/characters?player=<nombre>
/// Devuelve todos los personajes (o filtrados por jugador) de la campaña activa.
async fn get_all_characters(
    State(state): State<SharedState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    if !state.0.persistence.is_loaded().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "No hay ninguna campaña activa" })),
        )
            .into_response();
    }

    let result = if let Some(player) = params.get("player") {
        state.0.persistence.characters_by_player(player).await
    } else {
        state.0.persistence.all_characters().await
    };

    match result {
        Ok(characters) => (
            StatusCode::OK,
            Json(shared::persistence::CharactersResponse { characters }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// GET /api/characters/:id
async fn get_character(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if !state.0.persistence.is_loaded().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "No hay ninguna campaña activa" })),
        )
            .into_response();
    }

    match state.0.persistence.get_character(id).await {
        Ok(Some(c)) => (StatusCode::OK, Json(c)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Personaje {} no encontrado", id) })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// ===========================================================================
// Inventario
// ===========================================================================

use crate::backend::models::messages::ServerMessage;
use shared::api_types::inventory::{AddItemRequest, InventoryItem, InventoryResponse, UpdateCurrencyRequest, UpdateItemRequest};

async fn get_inventory(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.0.persistence.get_inventory(id).await {
        Ok((items, currency)) => (
            StatusCode::OK,
            Json(InventoryResponse::from_parts(items, currency)),
        ).into_response(),
        Err(e) => inventory_error(e),
    }
}

async fn add_item(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddItemRequest>,
) -> impl IntoResponse {
    let item = InventoryItem::new(req.name, req.category, req.description, req.quantity);
    let item = InventoryItem { weight: req.weight, notes: req.notes, ..item };
    match state.0.persistence.add_item(id, item).await {
        Ok(i) => {
            state.0.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id: id });
            (StatusCode::CREATED, Json(i)).into_response()
        }
        Err(e) => inventory_error(e),
    }
}

async fn update_item(
    State(state): State<SharedState>,
    Path((character_id, item_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateItemRequest>,
) -> impl IntoResponse {
    match state.0.persistence.update_item(character_id, item_id, req).await {
        Ok(i) => {
            state.0.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id });
            (StatusCode::OK, Json(i)).into_response()
        }
        Err(e) => inventory_error(e),
    }
}

async fn delete_item(
    State(state): State<SharedState>,
    Path((character_id, item_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match state.0.persistence.delete_item(character_id, item_id).await {
        Ok(()) => {
            state.0.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id });
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => inventory_error(e),
    }
}

async fn update_currency(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCurrencyRequest>,
) -> impl IntoResponse {
    match state.0.persistence.update_currency(id, req).await {
        Ok(c) => {
            state.0.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id: id });
            (StatusCode::OK, Json(c)).into_response()
        }
        Err(e) => inventory_error(e),
    }
}

// ===========================================================================
// Hechizos
// ===========================================================================

use shared::api_types::spells::{AddSpellRequest, SpellsResponse, UpdateSpellSlotsRequest};

async fn get_spells(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.0.persistence.get_spells(id).await {
        Ok(r) => (StatusCode::OK, Json(r)).into_response(),
        Err(e) => inventory_error(e),
    }
}

async fn add_known_spell(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddSpellRequest>,
) -> impl IntoResponse {
    match state.0.persistence.add_known_spell(id, req).await {
        Ok(s) => {
            state.0.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id: id });
            (StatusCode::CREATED, Json(s)).into_response()
        }
        Err(e) => inventory_error(e),
    }
}

async fn remove_known_spell(
    State(state): State<SharedState>,
    Path((character_id, spell_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match state.0.persistence.remove_known_spell(character_id, spell_id).await {
        Ok(()) => {
            state.0.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id });
            StatusCode::NO_CONTENT.into_response()
        }
        Err(e) => inventory_error(e),
    }
}

async fn toggle_prepared_spell(
    State(state): State<SharedState>,
    Path((character_id, spell_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match state.0.persistence.toggle_prepared_spell(character_id, spell_id).await {
        Ok(prepared) => (
            StatusCode::OK,
            Json(serde_json::json!({ "prepared": prepared })),
        ).into_response(),
        Err(e) => inventory_error(e),
    }
}

async fn update_spell_slots(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSpellSlotsRequest>,
) -> impl IntoResponse {
    match state.0.persistence.update_spell_slots(id, req).await {
        Ok(slots) => (StatusCode::OK, Json(slots)).into_response(),
        Err(e) => inventory_error(e),
    }
}

/// GET /api/vault/items — lista los objetos del vault con dnd_type: item
async fn get_vault_items(State(state): State<SharedState>) -> impl IntoResponse {
    if !state.0.vault.is_configured().await {
        return (StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "Vault no configurado" }))).into_response();
    }
    match state.0.vault.entries_by_kind(crate::vault::frontmatter::DndEntryType::Item).await {
        Ok(entries) => {
            let items: Vec<_> = entries.iter().map(|e| serde_json::json!({
                "path": e.relative_path,
                "name": e.display_name(),
                "description": e.frontmatter.extra.get("description")
                    .and_then(|v| v.as_str()).unwrap_or(""),
                "category": e.frontmatter.extra.get("category")
                    .and_then(|v| v.as_str()).unwrap_or("misc"),
                "weight": e.frontmatter.extra.get("weight").and_then(|v| v.as_f64()),
                "damage": e.frontmatter.extra.get("damage").and_then(|v| v.as_str()),
                "notes": e.frontmatter.extra.get("notes").and_then(|v| v.as_str()).unwrap_or(""),
                "tags": e.frontmatter.tags,
            })).collect();
            (StatusCode::OK, Json(serde_json::json!({ "items": items }))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

fn inventory_error(e: crate::persistence::PersistenceError) -> axum::response::Response {
    use crate::persistence::PersistenceError;
    let status = match &e {
        PersistenceError::NotFound => StatusCode::NOT_FOUND,
        PersistenceError::NoCampaign => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
}

// ===========================================================================
// Campaña
// ===========================================================================

/// GET /api/campaign — resumen de la campaña activa
async fn get_campaign(State(state): State<SharedState>) -> impl IntoResponse {
    match state.0.persistence.current().await {
        Some(c) => (
            StatusCode::OK,
            Json(shared::persistence::CampaignSummary::from(&c)),
        )
            .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "No hay ninguna campaña activa" })),
        )
            .into_response(),
    }
}

/// POST /api/campaign — crea una nueva campaña (sobreescribe la actual)
async fn create_campaign(
    State(state): State<SharedState>,
    Json(req): Json<shared::persistence::CreateCampaignRequest>,
) -> impl IntoResponse {
    match state.0.persistence.create(req.name, req.description).await {
        Ok(c) => (
            StatusCode::CREATED,
            Json(shared::persistence::CampaignSummary::from(&c)),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

/// GET /api/campaigns — lista todas las campañas guardadas
async fn list_campaigns(State(state): State<SharedState>) -> impl IntoResponse {
    match state.0.persistence.list_campaigns().await {
        Ok(list) => (StatusCode::OK, Json(list)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

/// GET /api/campaigns/:filename — activa una campaña concreta
async fn load_campaign_by_file(
    State(state): State<SharedState>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    match state.0.persistence.load_campaign(&filename).await {
        Ok(c) => (
            StatusCode::OK,
            Json(shared::persistence::CampaignSummary::from(&c)),
        ).into_response(),
        Err(crate::persistence::PersistenceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Campaña no encontrada" })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

/// DELETE /api/campaigns/:filename — elimina una campaña del disco
async fn delete_campaign(
    State(state): State<SharedState>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    match state.0.persistence.delete_campaign(&filename).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(crate::persistence::PersistenceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Campaña no encontrada" })),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

// ===========================================================================
// Combate
// ===========================================================================

async fn get_combat_state(State(_state): State<SharedState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

// ===========================================================================
// Lore
// ===========================================================================

async fn get_lore_index(State(state): State<SharedState>) -> impl IntoResponse {
    if !state.0.vault.is_configured().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "Vault no configurado" })),
        )
            .into_response();
    }

    match state
        .0
        .vault
        .entries_by_kind(crate::vault::frontmatter::DndEntryType::Lore)
        .await
    {
        Ok(entries) => {
            let index: Vec<_> = entries
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "path": e.relative_path,
                        "title": e.display_name(),
                        "tags": e.frontmatter.tags,
                    })
                })
                .collect();
            (
                StatusCode::OK,
                Json(serde_json::json!({ "entries": index })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_lore_entry(
    State(state): State<SharedState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    info!("Petición lore entry: {}", path);

    match state.0.vault.parse_entry(&path).await {
        Ok(parsed) => {
            // title: usa name del frontmatter; si no, el nombre del archivo sin extensión
            let title = parsed.frontmatter.name.unwrap_or_else(|| {
                std::path::Path::new(&path)
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.clone())
            });

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "path": path,
                    "title": title,
                    "content": parsed.body,
                    "tags": parsed.frontmatter.tags,
                    "image_url": parsed.frontmatter.image.map(|img| format!("/api/assets/image/{}", img)),
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// ===========================================================================
// Assets — imágenes del vault
// ===========================================================================

async fn get_vault_image(
    State(state): State<SharedState>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match state.0.vault.image_as_base64(&path).await {
        Ok(payload) => {
            // Devolvemos la imagen como data URL en JSON para simplicidad del cliente
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "mime_type": payload.mime_type,
                    "data": payload.data,
                    "path": payload.relative_path,
                })),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
