use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::api_types::character_draft::AttributesDto;
use crate::api_types::inventory::{Currency, InventoryItem};
use crate::api_types::spells::{Spell, SpellSlotLevel};

// ---------------------------------------------------------------------------
// Personaje guardado — draft finalizado listo para jugar
// ---------------------------------------------------------------------------

/// Snapshot completo de un personaje en el momento en que finaliza el wizard.
/// Es lo que se persiste en disco y se carga en futuras sesiones.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavedCharacter {
    pub id: Uuid,
    pub player_name: String,
    pub name: String,
    pub race_id: String,
    pub class_id: String,
    pub background_id: String,
    pub attributes: AttributesDto,
    #[serde(default)]
    pub feat_ids: Vec<String>,
    #[serde(default)]
    pub choices: HashMap<String, serde_json::Value>,
    /// Nivel actual (empieza en 1)
    pub level: u32,
    /// Puntos de golpe actuales
    pub current_hp: u32,
    /// Puntos de golpe máximos (calculados al crear)
    pub max_hp: u32,
    /// Puntos de experiencia
    pub xp: u64,
    /// Notas libres del jugador
    #[serde(default)]
    pub notes: String,
    /// Objetos en el inventario
    #[serde(default)]
    pub inventory: Vec<InventoryItem>,
    /// Monedas del personaje
    #[serde(default)]
    pub currency: Currency,
    /// Espacios de hechizo por nivel (1–9)
    #[serde(default)]
    pub spell_slots: Vec<SpellSlotLevel>,
    /// Hechizos conocidos (todos los que el personaje ha aprendido)
    #[serde(default)]
    pub known_spells: Vec<Spell>,
    /// Hechizos preparados (subconjunto de conocidos, activos para la jornada)
    #[serde(default)]
    pub prepared_spells: Vec<Spell>,
    /// Timestamp ISO 8601 de la última modificación
    pub updated_at: String,
}

impl SavedCharacter {
    /// Construye un SavedCharacter desde un draft finalizado.
    pub fn from_finalized_draft(
        player_name: String,
        draft_id: Uuid,
        name: String,
        race_id: String,
        class_id: String,
        background_id: String,
        attributes: AttributesDto,
        feat_ids: Vec<String>,
        choices: HashMap<String, serde_json::Value>,
    ) -> Self {
        // HP inicial = dado de golpe de clase + modificador de CON
        // Usamos el mínimo razonable (dado medio + CON) hasta tener la clase real
        let con_mod = constitution_modifier(attributes.constitution);
        let base_hp = (8 + con_mod).max(1) as u32; // d8 por defecto

        let now = chrono::Utc::now().to_rfc3339();

        Self {
            id: draft_id,
            player_name,
            name,
            race_id,
            class_id,
            background_id,
            attributes,
            feat_ids,
            choices,
            level: 1,
            current_hp: base_hp,
            max_hp: base_hp,
            xp: 0,
            notes: String::new(),
            inventory: Vec::new(),
            currency: Currency::default(),
            spell_slots: Vec::new(),
            known_spells: Vec::new(),
            prepared_spells: Vec::new(),
            updated_at: now,
        }
    }
}

fn constitution_modifier(constitution: u32) -> i32 {
    ((constitution as i32) - 10) / 2
}

// ---------------------------------------------------------------------------
// Campaña guardada
// ---------------------------------------------------------------------------

/// Estado completo de una campaña — se serializa a un único archivo JSON.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignFile {
    pub version: u32,
    pub name: String,
    pub description: String,
    /// Ruta al vault de Obsidian asociado (puede estar vacía)
    #[serde(default)]
    pub vault_path: Option<String>,
    /// Personajes de los jugadores, indexados por id
    #[serde(default)]
    pub characters: HashMap<Uuid, SavedCharacter>,
    /// Timestamp ISO 8601 de la última modificación
    pub updated_at: String,
}

impl CampaignFile {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            version: 1,
            name: name.into(),
            description: description.into(),
            vault_path: None,
            characters: HashMap::new(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
}

// ---------------------------------------------------------------------------
// Requests / Responses de la API de campaña
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CampaignSummary {
    pub name: String,
    pub description: String,
    pub vault_path: Option<String>,
    pub character_count: usize,
    pub updated_at: String,
}

impl From<&CampaignFile> for CampaignSummary {
    fn from(c: &CampaignFile) -> Self {
        Self {
            name: c.name.clone(),
            description: c.description.clone(),
            vault_path: c.vault_path.clone(),
            character_count: c.characters.len(),
            updated_at: c.updated_at.clone(),
        }
    }
}

/// Respuesta a GET /characters y GET /characters?player=X
#[derive(Debug, Serialize)]
pub struct CharactersResponse {
    pub characters: Vec<SavedCharacter>,
}
