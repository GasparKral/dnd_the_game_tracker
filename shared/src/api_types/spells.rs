use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Escuela de magia
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    #[default]
    Unknown,
}

impl SpellSchool {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Abjuration    => "Abjuración",
            Self::Conjuration   => "Conjuración",
            Self::Divination    => "Adivinación",
            Self::Enchantment   => "Encantamiento",
            Self::Evocation     => "Evocación",
            Self::Illusion      => "Ilusión",
            Self::Necromancy    => "Nigromancia",
            Self::Transmutation => "Transmutación",
            Self::Unknown       => "Desconocida",
        }
    }
}

// ---------------------------------------------------------------------------
// Componentes de conjuro
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SpellComponents {
    #[serde(default)]
    pub verbal: bool,
    #[serde(default)]
    pub somatic: bool,
    /// Material — None si no necesita, Some("") si necesita pero sin especificar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
}

// ---------------------------------------------------------------------------
// Conjuro conocido / preparado
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spell {
    pub id: Uuid,
    pub name: String,
    /// 0 = truco, 1–9 = nivel del conjuro
    pub level: u8,
    pub school: SpellSchool,
    /// Tiempo de lanzamiento. Ej: "1 acción", "1 acción adicional", "1 minuto"
    #[serde(default)]
    pub casting_time: String,
    /// Alcance. Ej: "18 m", "Toque", "Personal", "Vista"
    #[serde(default)]
    pub range: String,
    /// Duración. Ej: "Instantánea", "Concentración, hasta 1 minuto"
    #[serde(default)]
    pub duration: String,
    pub components: SpellComponents,
    /// Descripción completa del conjuro
    #[serde(default)]
    pub description: String,
    /// Daño o curación. Ej: "8d6 fuego", "2d8+4 radiante"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage: Option<String>,
    /// Salvación requerida. Ej: "DES", "SAB"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saving_throw: Option<String>,
    /// Notas adicionales del jugador
    #[serde(default)]
    pub notes: String,
    /// Requiere concentración
    #[serde(default)]
    pub concentration: bool,
    /// Es un ritual
    #[serde(default)]
    pub ritual: bool,
}

impl Spell {
    pub fn new(name: impl Into<String>, level: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            level,
            school: SpellSchool::Unknown,
            casting_time: String::new(),
            range: String::new(),
            duration: String::new(),
            components: SpellComponents::default(),
            description: String::new(),
            damage: None,
            saving_throw: None,
            notes: String::new(),
            concentration: false,
            ritual: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Espacio de hechizo por nivel
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SpellSlotLevel {
    /// Nivel del espacio (1–9)
    pub level: u8,
    /// Total que tiene el personaje en ese nivel
    pub total: u8,
    /// Los que quedan disponibles (se gasta al lanzar, se recupera con descanso largo)
    pub remaining: u8,
}

// ---------------------------------------------------------------------------
// Requests / Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct AddSpellRequest {
    pub name: String,
    pub level: u8,
    #[serde(default)]
    pub school: SpellSchool,
    #[serde(default)]
    pub casting_time: String,
    #[serde(default)]
    pub range: String,
    #[serde(default)]
    pub duration: String,
    #[serde(default)]
    pub components: SpellComponents,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub damage: Option<String>,
    #[serde(default)]
    pub saving_throw: Option<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub concentration: bool,
    #[serde(default)]
    pub ritual: bool,
    /// Si true, añadir a hechizos preparados además de conocidos
    #[serde(default)]
    pub prepared: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpellSlotsRequest {
    /// Lista completa de niveles con sus totales y restantes
    pub slots: Vec<SpellSlotLevel>,
}

#[derive(Debug, Serialize)]
pub struct SpellsResponse {
    pub known_spells: Vec<Spell>,
    pub prepared_spells: Vec<Spell>,
    pub spell_slots: Vec<SpellSlotLevel>,
}
