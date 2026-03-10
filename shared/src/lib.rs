pub mod api_types;
pub mod persistence;
pub mod traits;

// El modelo de dominio (models/) contiene la arquitectura de simulación de DnD —
// estructuras ricas con Box<dyn Trait>, reglas de apilamiento, sistema de dados, etc.
// Todavía no está conectado al sistema de red (no implementa Serialize) porque
// los intercambios con la app Android se hacen a través de api_types/ (DTOs).
// Se suprime dead_code a nivel de módulo hasta que la lógica de simulación
// esté activa y los From<> entre modelos estén implementados.
#[allow(dead_code)]
pub mod models;
