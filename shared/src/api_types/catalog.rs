use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Opciones de selección
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    /// ID interno. Ej: "high_elf"
    pub id: String,
    /// Texto legible. Ej: "Alto Elfo"
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ---------------------------------------------------------------------------
// Schema de choices — describe qué decisiones debe tomar el jugador
// ---------------------------------------------------------------------------

/// Describe un campo de decisión que el jugador debe completar en el wizard.
/// Kotlin renderiza este schema sin conocer el dominio DnD.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ChoiceSchema {
    /// Elige exactamente una opción de una lista
    SingleSelect {
        id: String,
        label: String,
        options: Vec<SelectOption>,
    },

    /// Elige entre `min` y `max` opciones de una lista
    MultiSelect {
        id: String,
        label: String,
        min: u8,
        max: u8,
        options: Vec<SelectOption>,
    },

    /// Distribuye `points` puntos entre los campos dados (point-buy de atributos)
    PointBuy {
        id: String,
        label: String,
        points: u32,
        fields: Vec<PointBuyField>,
    },

    /// Campo de texto libre (nombre del personaje, backstory, etc.)
    TextInput {
        id: String,
        label: String,
        max_length: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        placeholder: Option<String>,
    },

    /// Valor numérico dentro de un rango (edad, peso...)
    NumberInput {
        id: String,
        label: String,
        min: i32,
        max: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        default: Option<i32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointBuyField {
    /// ID del campo. Ej: "str", "dex", "con"
    pub id: String,
    /// Etiqueta legible. Ej: "Fuerza"
    pub label: String,
    pub min: u32,
    pub max: u32,
}

// ---------------------------------------------------------------------------
// Entrada del catálogo
// ---------------------------------------------------------------------------

/// Entrada de un catálogo (raza, clase, trasfondo, don...).
/// El cliente móvil recibe esta estructura y renderiza el wizard dinámicamente.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    /// ID único. Ej: "elf", "dragon_rider", "fighter"
    pub id: String,

    /// Nombre legible. Ej: "Elfo", "Jinete de Dragón"
    pub name: String,

    /// Origen. Ej: "PHB2024", "Homebrew", "UA2023"
    pub source: String,

    /// Descripción corta para mostrar en la lista
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Descripción larga / lore (Markdown permitido)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lore: Option<String>,

    /// URL relativa a la imagen principal. Ej: "/api/assets/image/races/elf.png"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    /// Decisiones que el jugador debe tomar al elegir esta entrada.
    /// El cliente debe responder los marcados en `required_choices` antes de avanzar.
    #[serde(default)]
    pub choices: Vec<ChoiceSchema>,

    /// IDs de los choices obligatorios (subset de `choices`).
    #[serde(default)]
    pub required_choices: Vec<String>,

    /// Lista de rasgos para mostrar en preview (solo nombres)
    #[serde(default)]
    pub traits_preview: Vec<String>,

    /// Rasgos mecánicos detallados (nombre + descripción)
    #[serde(default)]
    pub traits_detail: Vec<TraitDetail>,

    /// Velocidad base en metros
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_m: Option<u32>,

    /// Tamaño ("Small", "Medium"...)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// Rasgo mecánico con nombre y descripción corta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitDetail {
    pub name: String,
    pub description: String,
}

impl TraitDetail {
    pub fn new(name: impl Into<String>, desc: impl Into<String>) -> Self {
        Self { name: name.into(), description: desc.into() }
    }
}

impl SelectOption {
    pub fn new(id: impl Into<String>, label: impl Into<String>, desc: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into(), description: Some(desc.into()) }
    }
    pub fn bare(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self { id: id.into(), label: label.into(), description: None }
    }
}

impl CatalogEntry {
    pub fn minimal(id: impl Into<String>, name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            id: id.into(), name: name.into(), source: source.into(),
            description: None, lore: None, image_url: None,
            choices: vec![], required_choices: vec![],
            traits_preview: vec![], traits_detail: vec![],
            speed_m: None, size: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Respuestas de catálogo agrupadas por tipo
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogResponse {
    pub entries: Vec<CatalogEntry>,
}
