use crate::api_types::catalog::CatalogEntry;
use crate::models::character::Player;
use std::fmt::Debug;

/// Trait que deben implementar todas las razas/especies.
///
/// `catalog_entry()` devuelve la metadata completa que el registry publica en la API.
/// `apply()` aplica los efectos mecánicos al personaje (modificadores, rasgos, etc.).
pub trait Race: Debug + Send + Sync {
    fn id(&self) -> &'static str;
    /// Metadata de catálogo: nombre, descripción, choices de subespecie, rasgos…
    fn catalog_entry(&self) -> CatalogEntry;
    /// Aplica los efectos mecánicos de la raza al personaje.
    fn apply(&self, character: &mut Player);
}
