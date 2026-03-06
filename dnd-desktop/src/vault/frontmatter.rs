use std::path::Path;

use gray_matter::{engine::YAML, Matter, ParsedEntityStruct};
use serde::{Deserialize, Serialize};

use super::error::VaultError;

// ---------------------------------------------------------------------------
// Tipo de entrada DnD — campo `dnd_type` en el frontmatter
// ---------------------------------------------------------------------------

/// Discrimina para qué parte del sistema está destinada la nota.
/// El campo en YAML es `dnd_type` (snake_case).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DndEntryType {
    Race,
    Class,
    Background,
    Feat,
    Item,
    Spell,
    Npc,
    Location,
    Faction,
    Lore,
    /// Cualquier valor desconocido se preserva como string
    #[serde(other)]
    Unknown,
}

// ---------------------------------------------------------------------------
// Frontmatter completo de una nota
// ---------------------------------------------------------------------------

/// Cabecera YAML de una nota del vault.
/// Todos los campos son opcionales — una nota sin frontmatter es válida (lore puro).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NoteFrontmatter {
    // --- Clasificación ---
    /// Tipo de entrada DnD. Ausente → nota de lore genérica.
    pub dnd_type: Option<DndEntryType>,

    /// Identificador único de la entidad (slug). Ej: "dragon_rider".
    pub id: Option<String>,

    /// Nombre legible. Ej: "Jinete de Dragón".
    pub name: Option<String>,

    /// Origen de la regla. Ej: "PHB2024", "Homebrew", "UA2023".
    pub source: Option<String>,

    /// Indica si la entrada está lista para usarse en partida.
    #[serde(default)]
    pub published: bool,

    // --- Imágenes ---
    /// Imagen principal — ruta relativa al vault. Ej: "Assets/Images/races/dragon_rider.png"
    pub image: Option<String>,

    /// Imágenes adicionales — rutas relativas al vault.
    #[serde(default)]
    pub gallery: Vec<String>,

    // --- Tags de Obsidian ---
    /// Tags libres. Ej: ["combate", "magia", "acuático"]
    #[serde(default)]
    pub tags: Vec<String>,

    // --- Campos extra no mapeados ---
    /// Todo lo demás que haya en el frontmatter queda aquí como JSON crudo,
    /// útil para los campos específicos de cada dnd_type (choices, traits, etc.)
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Resultado del parsing
// ---------------------------------------------------------------------------

/// Resultado de parsear una nota: frontmatter separado del cuerpo.
#[derive(Debug, Clone)]
pub struct ParsedNote {
    pub frontmatter: NoteFrontmatter,
    /// Cuerpo markdown sin la cabecera YAML.
    pub body: String,
}

// ---------------------------------------------------------------------------
// Función de parsing
// ---------------------------------------------------------------------------

/// Parsea el frontmatter YAML de un archivo markdown.
/// Usa `parse_with_struct` de gray_matter para deserializar directamente al tipo.
/// Si el archivo no tiene frontmatter devuelve `NoteFrontmatter::default()` con
/// el contenido completo como body.
pub fn parse_frontmatter(path: &Path, raw: &str) -> Result<ParsedNote, VaultError> {
    let matter = Matter::<YAML>::new();

    // parse_with_struct devuelve Option<ParsedEntityStruct<T>> — None si no hay frontmatter.
    let result: Option<ParsedEntityStruct<NoteFrontmatter>> = matter.parse_with_struct(raw);

    match result {
        Some(parsed) => Ok(ParsedNote {
            frontmatter: parsed.data,
            body: parsed.content,
        }),
        None => {
            // Sin frontmatter: cuerpo completo, valores por defecto
            Ok(ParsedNote {
                frontmatter: NoteFrontmatter::default(),
                body: raw.to_string(),
            })
        }
    }
}
