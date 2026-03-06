use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("El vault no existe o no es un directorio: {0}")]
    InvalidPath(PathBuf),

    #[error("Error de I/O en '{path}': {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Error al parsear el frontmatter YAML de '{path}': {message}")]
    FrontmatterParse { path: PathBuf, message: String },

    #[error("El vault no está configurado")]
    NotConfigured,

    #[error("Imagen no encontrada: {0}")]
    ImageNotFound(PathBuf),

    #[error("Formato de imagen no soportado: {0}")]
    UnsupportedImageFormat(String),

    #[error("Error al codificar imagen en base64: {0}")]
    ImageEncoding(String),
}
