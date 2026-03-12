use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Categorías de proficiencia — PHB 2024
// ---------------------------------------------------------------------------

/// Categoría de una proficiencia según las reglas de D&D 5.5e PHB 2024.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProficiencyCategory {
    /// Habilidades (Atletismo, Sigilo, Arcanos...)
    Skill,
    /// Salvaciones (Fuerza, Destreza, Constitución...)
    SavingThrow,
    /// Armaduras (Ligera, Media, Pesada, Escudo)
    Armor,
    /// Armas (Simples, Marciales, o arma específica)
    Weapon,
    /// Herramientas (Herramientas de ladrón, instrumentos...)
    Tool,
    /// Idiomas
    Language,
}

impl ProficiencyCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Skill => "Habilidad",
            Self::SavingThrow => "Salvación",
            Self::Armor => "Armadura",
            Self::Weapon => "Arma",
            Self::Tool => "Herramienta",
            Self::Language => "Idioma",
        }
    }
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Skill => "🎯",
            Self::SavingThrow => "🛡",
            Self::Armor => "🧥",
            Self::Weapon => "⚔️",
            Self::Tool => "🔧",
            Self::Language => "📜",
        }
    }
}

// ---------------------------------------------------------------------------
// Nivel de maestría (5.5e permite Expertise)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProficiencyLevel {
    /// Proficiencia normal (bono de proficiencia)
    Proficient,
    /// Maestría — dobla el bono de proficiencia (Expertise en 5.5e)
    Expert,
    /// Semiproficiencia — mitad del bono (Bard Jack of All Trades, etc.)
    HalfProficient,
}

impl ProficiencyLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Proficient => "Competente",
            Self::Expert => "Experto",
            Self::HalfProficient => "Semiproficiente",
        }
    }
}

// ---------------------------------------------------------------------------
// Proficiencia individual
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proficiency {
    /// ID único. Ej: "athletics", "thieves_tools", "elvish"
    pub id: String,
    /// Nombre legible. Ej: "Atletismo", "Herramientas de ladrón"
    pub name: String,
    pub category: ProficiencyCategory,
    pub level: ProficiencyLevel,
    /// Fuente que la otorgó. Ej: "Clase: Guerrero", "Raza: Elfo", "Manual"
    #[serde(default)]
    pub source: String,
    /// Notas libres del jugador
    #[serde(default)]
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Requests / Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddProficiencyRequest {
    pub name: String,
    pub category: ProficiencyCategory,
    #[serde(default = "default_proficient")]
    pub level: ProficiencyLevel,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub notes: String,
}

fn default_proficient() -> ProficiencyLevel {
    ProficiencyLevel::Proficient
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProficiencyRequest {
    #[serde(default)]
    pub level: Option<ProficiencyLevel>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProficienciesResponse {
    pub proficiencies: Vec<Proficiency>,
}
