pub mod entry;
pub mod error;
pub mod frontmatter;

use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use regex::Regex;
use tokio::sync::RwLock;
use tracing::{debug, warn};
use walkdir::WalkDir;

use entry::VaultEntry;
use error::VaultError;
use frontmatter::{parse_frontmatter, DndEntryType};

// ---------------------------------------------------------------------------
// Formatos de imagen soportados para envío al cliente
// ---------------------------------------------------------------------------

const SUPPORTED_IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "svg"];

fn mime_type_for(ext: &str) -> Option<&'static str> {
    match ext.to_lowercase().as_str() {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "webp" => Some("image/webp"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// VaultManager
// ---------------------------------------------------------------------------

/// Gestiona el acceso al vault de Obsidian.
///
/// Es `Clone` porque está pensado para vivir dentro de un `Arc<AppState>`.
/// El estado mutable interno se protege con `RwLock`.
#[derive(Debug)]
pub struct VaultManager {
    /// Root del vault. `None` hasta que el DM lo configure.
    root: RwLock<Option<PathBuf>>,
}

impl VaultManager {
    pub fn new() -> Self {
        Self {
            root: RwLock::new(None),
        }
    }

    // -----------------------------------------------------------------------
    // Configuración del vault
    // -----------------------------------------------------------------------

    /// Establece el root del vault validando que la ruta exista y sea un directorio.
    pub async fn open(&self, path: PathBuf) -> Result<(), VaultError> {
        if !path.is_dir() {
            return Err(VaultError::InvalidPath(path));
        }
        *self.root.write().await = Some(path);
        Ok(())
    }

    /// Devuelve el root actual, o error si no está configurado.
    pub async fn root(&self) -> Result<PathBuf, VaultError> {
        self.root
            .read()
            .await
            .clone()
            .ok_or(VaultError::NotConfigured)
    }

    /// Indica si el vault está configurado.
    pub async fn is_configured(&self) -> bool {
        self.root.read().await.is_some()
    }

    // -----------------------------------------------------------------------
    // Escaneo de entradas
    // -----------------------------------------------------------------------

    /// Recorre el vault y devuelve solo las notas `.md` con su frontmatter parseado.
    /// Usado por la API para catálogos y filtros por `dnd_type`.
    pub async fn scan(&self) -> Result<Vec<VaultEntry>, VaultError> {
        let root = self.root().await?;
        self.scan_files(&root).await
    }

    /// Recorre el vault devolviendo **tanto directorios como archivos `.md`**,
    /// ordenados para construir un árbol de navegación en la UI.
    /// Excluye directorios ocultos (`.obsidian`, `.git`, etc.) y el root mismo.
    pub async fn scan_tree(&self) -> Result<Vec<VaultEntry>, VaultError> {
        let root = self.root().await?;
        self.scan_tree_from(&root).await
    }

    /// Filtra las entradas por `dnd_type`.
    pub async fn entries_by_kind(&self, kind: DndEntryType) -> Result<Vec<VaultEntry>, VaultError> {
        let all = self.scan().await?;

        Ok(all
            .into_iter()
            .filter(|e| e.frontmatter.dnd_type.as_ref() == Some(&kind))
            .collect())
    }

    /// Solo archivos .md — para la API y catálogos.
    async fn scan_files(&self, root: &Path) -> Result<Vec<VaultEntry>, VaultError> {
        let mut entries = Vec::new();

        for dir_entry in WalkDir::new(root)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                // Excluir directorios ocultos en toda la ruta
                !e.path()
                    .components()
                    .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
            })
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let path = dir_entry.path().to_path_buf();
            let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();

            match self.read_entry_raw(&path).await {
                Ok(raw) => match parse_frontmatter(&path, &raw) {
                    Ok(parsed) => {
                        entries.push(VaultEntry::new(path, relative, parsed.frontmatter));
                    }
                    Err(e) => warn!("Frontmatter inválido en '{}': {}", path.display(), e),
                },
                Err(e) => warn!("No se pudo leer '{}': {}", path.display(), e),
            }
        }

        debug!("Vault escaneado: {} notas encontradas", entries.len());
        Ok(entries)
    }

    /// Directorios + archivos .md — para el árbol de la UI.
    async fn scan_tree_from(&self, root: &Path) -> Result<Vec<VaultEntry>, VaultError> {
        let mut entries = Vec::new();

        for dir_entry in WalkDir::new(root)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                // Excluir el root mismo y cualquier ruta que pase por un dir oculto
                e.depth() > 0
                    && !e
                        .path()
                        .components()
                        .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
            })
            .filter(|e| {
                // Incluir directorios y archivos .md; excluir otros archivos
                e.file_type().is_dir() || e.path().extension().map_or(false, |ext| ext == "md")
            })
        {
            let path = dir_entry.path().to_path_buf();
            let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();

            let frontmatter = if path.is_file() {
                match self.read_entry_raw(&path).await {
                    Ok(raw) => match parse_frontmatter(&path, &raw) {
                        Ok(parsed) => parsed.frontmatter,
                        Err(e) => {
                            warn!("Frontmatter inválido en '{}': {}", path.display(), e);
                            frontmatter::NoteFrontmatter::default()
                        }
                    },
                    Err(e) => {
                        warn!("No se pudo leer '{}': {}", path.display(), e);
                        continue;
                    }
                }
            } else {
                // Directorios no tienen frontmatter
                frontmatter::NoteFrontmatter::default()
            };

            entries.push(VaultEntry::new(path, relative, frontmatter));
        }

        debug!("Vault tree: {} entradas (dirs + md)", entries.len());
        Ok(entries)
    }

    // -----------------------------------------------------------------------
    // Lectura de notas
    // -----------------------------------------------------------------------

    /// Lee el contenido crudo de una nota dado su path relativo al vault.
    /// Ej: `"Razas/dragon_rider.md"` o `"Razas/dragon_rider"` (añade .md automáticamente).
    pub async fn read_entry(&self, relative: &str) -> Result<String, VaultError> {
        let root = self.root().await?;
        let relative = if relative.ends_with(".md") {
            PathBuf::from(relative)
        } else {
            PathBuf::from(format!("{}.md", relative))
        };
        let full = root.join(&relative);
        self.read_entry_raw(&full).await
    }

    /// Lee y parsea una nota completa (frontmatter + body).
    pub async fn parse_entry(&self, relative: &str) -> Result<frontmatter::ParsedNote, VaultError> {
        let raw = self.read_entry(relative).await?;
        let root = self.root().await?;
        let path = root.join(relative);
        parse_frontmatter(&path, &raw)
    }

    async fn read_entry_raw(&self, full_path: &Path) -> Result<String, VaultError> {
        tokio::fs::read_to_string(full_path)
            .await
            .map_err(|e| VaultError::Io {
                path: full_path.to_path_buf(),
                source: e,
            })
    }

    // -----------------------------------------------------------------------
    // Imágenes
    // -----------------------------------------------------------------------

    /// Resuelve una ruta relativa al vault a su ruta absoluta, verificando que
    /// el archivo existe y tiene un formato soportado.
    pub async fn resolve_image(&self, relative: &str) -> Result<PathBuf, VaultError> {
        let root = self.root().await?;
        let full = root.join(relative);

        let ext = full
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        if !SUPPORTED_IMAGE_EXTENSIONS.contains(&ext.as_str()) {
            return Err(VaultError::UnsupportedImageFormat(ext.to_string()));
        }

        if !full.exists() {
            return Err(VaultError::ImageNotFound(full));
        }

        Ok(full)
    }

    /// Lee una imagen del vault y la devuelve codificada en base64 junto con su MIME type.
    /// Útil para incrustar imágenes en respuestas JSON hacia el cliente móvil.
    pub async fn image_as_base64(&self, relative: &str) -> Result<ImagePayload, VaultError> {
        let full = self.resolve_image(relative).await?;

        let ext = full
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let mime = mime_type_for(&ext)
            .ok_or_else(|| VaultError::UnsupportedImageFormat(ext.to_string()))?;

        let bytes = tokio::fs::read(&full).await.map_err(|e| VaultError::Io {
            path: full.clone(),
            source: e,
        })?;

        let data = BASE64.encode(&bytes);

        Ok(ImagePayload {
            mime_type: mime.to_string(),
            data,
            relative_path: relative.to_string(),
        })
    }

    /// Extrae todas las referencias de imagen `![[...]]` del cuerpo de una nota.
    /// Devuelve las rutas relativas al vault de cada imagen referenciada.
    pub fn extract_wikilink_images(&self, body: &str) -> Vec<String> {
        // Captura: ![[ruta/imagen.png]] o ![[imagen.png|alt text]]
        let re = Regex::new(r"!\[\[([^\]|]+?)(?:\|[^\]]*?)?\]\]").unwrap();
        re.captures_iter(body)
            .filter_map(|cap| {
                let path = cap[1].trim().to_string();
                let ext = Path::new(&path)
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                if SUPPORTED_IMAGE_EXTENSIONS.contains(&ext.as_str()) {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Devuelve todas las imágenes de una entrada: las del frontmatter +
    /// las referencias `![[...]]` del cuerpo.
    pub async fn all_images_of_entry(&self, relative: &str) -> Result<Vec<String>, VaultError> {
        let root = self.root().await?;
        let parsed = self.parse_entry(relative).await?;

        let mut images = Vec::new();

        // Imagen principal del frontmatter
        if let Some(img) = parsed.frontmatter.image {
            images.push(img);
        }

        // Galería del frontmatter
        images.extend(parsed.frontmatter.gallery);

        // Referencias wikilink del cuerpo
        let wikilinks = self.extract_wikilink_images(&parsed.body);
        for link in wikilinks {
            // Intentamos resolver contra el vault para encontrar la ruta real
            // Obsidian permite referenciar solo por nombre de archivo sin subcarpeta
            let candidate = root.join(&link);
            if candidate.exists() {
                images.push(link);
            } else {
                // Búsqueda por nombre de archivo en todo el vault
                if let Some(found) = self.find_asset_by_name(&root, &link) {
                    let rel = found
                        .strip_prefix(&root)
                        .unwrap_or(&found)
                        .to_string_lossy()
                        .to_string();
                    images.push(rel);
                }
            }
        }

        // Deduplicar manteniendo orden
        let mut seen = std::collections::HashSet::new();
        images.retain(|i| seen.insert(i.clone()));

        Ok(images)
    }

    /// Busca un archivo por nombre en el vault (para resolver wikilinks sin ruta completa).
    fn find_asset_by_name(&self, root: &Path, name: &str) -> Option<PathBuf> {
        let target = Path::new(name)
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())?;

        WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| e.file_name().to_string_lossy().to_lowercase() == target)
            .map(|e| e.path().to_path_buf())
    }
}

// ---------------------------------------------------------------------------
// ImagePayload — respuesta serializable para el cliente móvil
// ---------------------------------------------------------------------------

/// Imagen lista para ser enviada al cliente móvil en una respuesta JSON.
#[derive(Debug, serde::Serialize)]
pub struct ImagePayload {
    /// MIME type. Ej: "image/png"
    pub mime_type: String,
    /// Datos de la imagen en base64
    pub data: String,
    /// Ruta relativa original dentro del vault
    pub relative_path: String,
}
