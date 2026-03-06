use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::frontmatter::{DndEntryType, NoteFrontmatter};

// ---------------------------------------------------------------------------
// VaultEntry — representa una nota dentro del vault
// ---------------------------------------------------------------------------

/// Metadatos de una nota del vault, sin el cuerpo completo.
/// Se usa para construir índices y listados sin leer todos los archivos enteros.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VaultEntry {
    /// Ruta absoluta al archivo .md
    pub path: PathBuf,

    /// Ruta relativa al root del vault. Ej: "Razas/dragon_rider.md"
    pub relative_path: PathBuf,

    /// Nombre del archivo sin extensión. Ej: "dragon_rider"
    pub slug: String,

    /// Frontmatter parseado de la nota
    pub frontmatter: NoteFrontmatter,
}

impl VaultEntry {
    pub(crate) fn new(path: PathBuf, relative_path: PathBuf, frontmatter: NoteFrontmatter) -> Self {
        let slug = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Self {
            path,
            relative_path,
            slug,
            frontmatter,
        }
    }

    /// Nombre legible: usa `frontmatter.name` si está presente, si no el slug.
    pub fn display_name(&self) -> &str {
        self.frontmatter.name.as_deref().unwrap_or(&self.slug)
    }

    /// Devuelve el tipo DnD de la entrada, si está definido.
    pub fn dnd_type(&self) -> Option<&DndEntryType> {
        self.frontmatter.dnd_type.as_ref()
    }

    /// Indica si la entrada tiene imagen principal definida en el frontmatter.
    pub fn has_image(&self) -> bool {
        self.frontmatter.image.is_some()
    }

    /// Resuelve la ruta absoluta de la imagen principal contra el root del vault.
    pub fn resolve_image(&self, vault_root: &Path) -> Option<PathBuf> {
        self.frontmatter
            .image
            .as_ref()
            .map(|rel| vault_root.join(rel))
    }

    /// Resuelve las rutas absolutas de todas las imágenes de la galería.
    pub fn resolve_gallery(&self, vault_root: &Path) -> Vec<PathBuf> {
        self.frontmatter
            .gallery
            .iter()
            .map(|rel| vault_root.join(rel))
            .collect()
    }

    /// Todas las imágenes (principal + galería) resueltas, en orden.
    pub fn all_images(&self, vault_root: &Path) -> Vec<PathBuf> {
        let mut images = Vec::new();
        if let Some(main) = self.resolve_image(vault_root) {
            images.push(main);
        }
        images.extend(self.resolve_gallery(vault_root));
        images
    }
}
