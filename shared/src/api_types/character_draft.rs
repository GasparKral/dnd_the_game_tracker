use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Atributos
// ---------------------------------------------------------------------------

/// Los seis atributos base de DnD 5.5e
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributesDto {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl AttributesDto {
    /// Suma total — útil para validar point-buy (27 puntos estándar)
    pub fn point_buy_cost(&self) -> u32 {
        [
            self.strength,
            self.dexterity,
            self.constitution,
            self.intelligence,
            self.wisdom,
            self.charisma,
        ]
        .iter()
        .map(|&v| cost_for_score(v))
        .sum()
    }
}

/// Coste en puntos de point-buy según la tabla de 5.5e
fn cost_for_score(score: u32) -> u32 {
    match score {
        8 => 0,
        9 => 1,
        10 => 2,
        11 => 3,
        12 => 4,
        13 => 5,
        14 => 7,
        15 => 9,
        _ => u32::MAX, // valor inválido
    }
}

// ---------------------------------------------------------------------------
// Paso del wizard
// ---------------------------------------------------------------------------

/// En qué paso del wizard de creación se encuentra el draft.
/// El servidor lo actualiza en cada respuesta para que el cliente sepa qué mostrar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CreationStep {
    #[default]
    Name,
    Race,
    Class,
    Attributes,
    Background,
    Feats,
    Review,
    Complete,
}

impl CreationStep {
    pub fn next(&self) -> Self {
        match self {
            Self::Name => Self::Race,
            Self::Race => Self::Class,
            Self::Class => Self::Attributes,
            Self::Attributes => Self::Background,
            Self::Background => Self::Feats,
            Self::Feats => Self::Review,
            Self::Review => Self::Complete,
            Self::Complete => Self::Complete,
        }
    }

    pub fn is_complete(&self) -> bool {
        *self == Self::Complete
    }
}

// ---------------------------------------------------------------------------
// Draft del personaje — estado parcial durante la creación
// ---------------------------------------------------------------------------

/// Estado del personaje en construcción.
/// Se actualiza paso a paso desde el cliente y se persiste en el servidor.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterDraft {
    /// ID del draft — asignado por el servidor al crear
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<Uuid>,

    /// Paso actual del wizard
    pub step: CreationStep,

    // --- Datos del personaje ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// ID de la raza elegida. Ej: "elf", "dragon_rider"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_id: Option<String>,

    /// ID de la clase elegida. Ej: "fighter", "wizard"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<String>,

    /// ID del trasfondo elegido. Ej: "acolyte", "soldier"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_id: Option<String>,

    /// Atributos asignados (point-buy o método estándar)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDto>,

    /// IDs de los dones elegidos (puede ser vacío hasta el paso Feats)
    #[serde(default)]
    pub feat_ids: Vec<String>,

    /// Respuestas a los choices de raza/clase/trasfondo
    /// La clave es `"<entry_id>.<choice_id>"`. Ej: `"elf.subrace"` → `"high_elf"`
    #[serde(default)]
    pub choices: HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Requests del cliente
// ---------------------------------------------------------------------------

/// Inicia un nuevo draft. El servidor responde con el draft_id asignado.
#[derive(Debug, Deserialize)]
pub struct CreateDraftRequest {
    /// Nombre del jugador (no del personaje) — para asociar al WS
    pub player_name: String,
}

/// Avanza el wizard con los datos del paso actual.
/// El servidor valida, actualiza el draft y devuelve el siguiente paso.
#[derive(Debug, Deserialize)]
pub struct UpdateDraftRequest {
    pub draft_id: Uuid,
    pub step: CreationStep,

    /// Nombre del jugador — requerido solo en el último paso para el auto-save
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_name: Option<String>,

    // Campos opcionales — solo se envían en el paso correspondiente
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub race_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<AttributesDto>,

    #[serde(default)]
    pub feat_ids: Vec<String>,

    #[serde(default)]
    pub choices: HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Respuestas del servidor
// ---------------------------------------------------------------------------

/// Respuesta a POST /character/draft/create y PUT /character/draft/:id
#[derive(Debug, Serialize)]
pub struct DraftResponse {
    pub draft: CharacterDraft,
    /// Errores de validación del paso actual, si los hay
    #[serde(default)]
    pub errors: Vec<String>,
    /// true si el draft fue finalizado y el personaje ya existe
    pub finalized: bool,
}

/// Respuesta a GET /character/draft/:id
#[derive(Debug, Serialize)]
pub struct DraftStatusResponse {
    pub draft: CharacterDraft,
    pub is_complete: bool,
}
