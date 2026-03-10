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
    pub level: u32,
    pub current_hp: u32,
    pub max_hp: u32,
    pub xp: u64,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub inventory: Vec<InventoryItem>,
    #[serde(default)]
    pub currency: Currency,
    #[serde(default)]
    pub spell_slots: Vec<SpellSlotLevel>,
    #[serde(default)]
    pub known_spells: Vec<Spell>,
    #[serde(default)]
    pub prepared_spells: Vec<Spell>,
    pub updated_at: String,
}

impl SavedCharacter {
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
        // Bug 2 corregido: dado de golpe correcto por clase según PHB 2024.
        // Antes se usaba d8 hardcodeado para todas las clases — sobreestimaba
        // PG de Magos/Brujos y subestimaba Guerreros/Bárbaros.
        let hit_die = hit_die_for_class(&class_id);
        let con_mod = constitution_modifier(attributes.constitution);

        // Nivel 1: máximo del dado de golpe + mod CON (regla PHB 2024)
        let base_hp = (hit_die as i32 + con_mod).max(1) as u32;

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

/// Dado de golpe máximo por clase — PHB 2024.
/// Se usa el valor máximo del dado (no la media) para el nivel 1.
///
/// | Clase      | Dado |
/// |------------|------|
/// | Bárbaro    | d12  |
/// | Guerrero   | d10  |
/// | Paladín    | d10  |
/// | Explorador | d10  |
/// | Bardo      | d8   |
/// | Clérigo    | d8   |
/// | Druida     | d8   |
/// | Monje      | d8   |
/// | Pícaro     | d8   |
/// | Hechicero  | d6   |
/// | Brujo      | d8   |
/// | Mago       | d6   |
pub fn hit_die_for_class(class_id: &str) -> u8 {
    match class_id {
        "barbarian"             => 12,
        "fighter" | "paladin" | "ranger" => 10,
        "bard" | "cleric" | "druid" | "monk" | "rogue" | "warlock" => 8,
        "sorcerer" | "wizard"   => 6,
        // Homebrew u otras clases: d8 como valor razonable por defecto
        _                       => 8,
    }
}

fn constitution_modifier(constitution: u32) -> i32 {
    ((constitution as i32) - 10) / 2
}

// ---------------------------------------------------------------------------
// Campaña guardada
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignFile {
    pub version: u32,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub vault_path: Option<String>,
    #[serde(default)]
    pub characters: HashMap<Uuid, SavedCharacter>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize)]
pub struct CharactersResponse {
    pub characters: Vec<SavedCharacter>,
}
