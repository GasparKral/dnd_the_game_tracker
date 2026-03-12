// Modal de creación de objetos. Actúa como dispatcher:
// según el tipo elegido renderiza el formulario específico.
pub mod armour_form;
pub mod consumable_form;
pub mod misc_form;
//pub mod tool_form;
//pub mod treasure_form;
pub mod weapon_form;

use crate::states::SharedState;
use dioxus::prelude::*;
use shared::api_types::inventory::{InventoryItem, ItemCategory};
use uuid::Uuid;

use armour_form::ArmourForm;
use consumable_form::ConsumableForm;
use misc_form::MiscForm;
//use tool_form::ToolForm;
//use treasure_form::TreasureForm;
use weapon_form::WeaponForm;

// ── Tipo de objeto ────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default, Debug)]
pub enum ItemType {
    Weapon,
    Armour,
    Consumable,
    Tool,
    Treasure,
    #[default]
    Misc,
}

impl ItemType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Weapon => "⚔️ Arma",
            Self::Armour => "🛡️ Armadura",
            Self::Consumable => "🧪 Consumible",
            Self::Tool => "🔧 Herramienta",
            Self::Treasure => "💎 Tesoro",
            Self::Misc => "📦 Misc",
        }
    }

    pub fn vault_category(&self) -> &'static str {
        match self {
            Self::Weapon => "weapon",
            Self::Armour => "armour",
            Self::Consumable => "consumable",
            Self::Tool => "tool",
            Self::Treasure => "treasure",
            Self::Misc => "misc",
        }
    }

    pub fn all() -> &'static [ItemType] {
        &[
            ItemType::Weapon,
            ItemType::Armour,
            ItemType::Consumable,
            ItemType::Tool,
            ItemType::Treasure,
            ItemType::Misc,
        ]
    }
}

// ── Datos base comunes a todos los tipos ─────────────────────────────────────

/// Struct de datos que cada subformulario devuelve al modal padre.
/// Los campos específicos van en `extra`.
#[derive(Clone, Debug, Default)]
pub struct NewItemData {
    pub name: String,
    pub description: String,
    pub weight: Option<f32>,
    pub rarity: String,
    pub source: String,
    pub tags: Vec<String>,
    pub notes: String,
    /// Campos adicionales según el tipo (damage, ac, etc.)
    pub extra: std::collections::HashMap<String, String>,
}

// ── Modal dispatcher ──────────────────────────────────────────────────────────

/// Modal unificado de creación de objetos.
///
/// - Cuando `character_id` es `Some(id)`, además de escribir al vault también
///   añade el objeto al inventario del personaje indicado y llama `on_added`.
/// - Cuando `character_id` es `None`, sólo escribe al vault y llama `on_created`.
#[component]
pub fn CreateItemModal(
    on_close: EventHandler<()>,
    on_created: EventHandler<()>,
    /// Si se proporciona, el objeto se añade también al inventario de este PJ.
    #[props(default)]
    character_id: Option<Uuid>,
    /// Callback extra que se dispara después de añadir al inventario.
    /// Recibe el `InventoryItem` recién creado.
    #[props(default)]
    on_added: Option<EventHandler<InventoryItem>>,
) -> Element {
    let state = use_context::<SharedState>();
    let mut selected_type = use_signal(ItemType::default);
    let mut saving = use_signal(|| false);
    let mut save_error: Signal<Option<String>> = use_signal(|| None);

    // Etiqueta del botón de guardar según el contexto.
    let btn_label: Option<&'static str> = if character_id.is_some() {
        Some("➕ Guardar y añadir al inventario")
    } else {
        None // usa el valor por defecto del SaveButton
    };

    // use_callback produce un Callback<NewItemData> que es Copy,
    // eliminando la necesidad de clonar el handler para cada subform.
    let st = state.clone();
    let on_save: Callback<NewItemData> = use_callback(move |data: NewItemData| {
        let item_type = selected_type.read().clone();
        let s = st.0.clone();
        saving.set(true);
        save_error.set(None);
        spawn(async move {
            match write_item_to_vault(&s, &item_type, &data).await {
                Err(e) => {
                    save_error.set(Some(e));
                    saving.set(false);
                }
                Ok(()) => {
                    // Modo inventario: también persistir en el inventario del PJ.
                    if let Some(cid) = character_id {
                        let category = item_type_to_inventory_category(&item_type);
                        let inv_item = InventoryItem {
                            id: Uuid::new_v4(),
                            name: data.name.clone(),
                            category,
                            description: data.description.clone(),
                            quantity: 1,
                            weight: data.weight,
                            equipped: false,
                            accessory_type: None,
                            stat_bonuses: vec![],
                            notes: data.notes.clone(),
                        };
                        let item_clone = inv_item.clone();
                        match s.persistence.add_item(cid, item_clone).await {
                            Ok(persisted) => {
                                if let Some(handler) = on_added {
                                    handler.call(persisted);
                                }
                            }
                            Err(e) => {
                                // El vault ya se escribió; reportamos el error
                                // de inventario pero no bloqueamos.
                                tracing::warn!("No se pudo añadir al inventario: {e}");
                            }
                        }
                    }
                    on_created.call(());
                }
            }
        });
    });

    rsx! {
        // Backdrop
        div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.82);
                    display:flex; align-items:center; justify-content:center; z-index:100;",

            div {
                style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                        padding:28px; width:640px; max-width:96vw; max-height:90vh;
                        display:flex; flex-direction:column; gap:18px; overflow-y:auto;",

                // ── Header ────────────────────────────────────────────────
                div { style: "display:flex; justify-content:space-between; align-items:center;",
                    h2 { style: "font-size:1.05rem; font-weight:700; color:#fef3c7; margin:0;",
                        if character_id.is_some() { "🎒 Nuevo Objeto (inventario)" }
                        else { "✨ Nuevo Objeto" }
                    }
                    button {
                        style: "padding:4px 12px; font-size:0.68rem; border-radius:8px;
                                cursor:pointer; background:#0c0a09; color:#78716c;
                                border:1px solid #292524;",
                        onclick: move |_| on_close.call(()),
                        "✕ Cerrar"
                    }
                }

                // Aviso de modo inventario
                if character_id.is_some() {
                    div {
                        style: "font-size:0.7rem; color:#86efac; background:#052e16;
                                border:1px solid #166534; border-radius:8px; padding:6px 12px;",
                        "El objeto se guardará en el vault — y se añadirá automáticamente al inventario del personaje."
                    }
                }

                // ── Selector de tipo ──────────────────────────────────────
                div { style: "display:flex; flex-direction:column; gap:6px;",
                    label { style: "font-size:0.68rem; color:#78716c; text-transform:uppercase;
                                    letter-spacing:0.08em;",
                        "Tipo de objeto"
                    }
                    div { style: "display:flex; gap:6px; flex-wrap:wrap;",
                        for t in ItemType::all() {
                            {
                                let t2 = t.clone();
                                let active = *selected_type.read() == *t;
                                let (bg, col, brd) = if active {
                                    ("#451a03","#fbbf24","#92400e")
                                } else {
                                    ("#0c0a09","#a8a29e","#292524")
                                };
                                rsx! {
                                    button {
                                        style: "padding:5px 14px; font-size:0.72rem; border-radius:20px;
                                                background:{bg}; color:{col}; border:1px solid {brd};
                                                cursor:pointer;",
                                        onclick: move |_| selected_type.set(t2.clone()),
                                        "{t.label()}"
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Formulario dinámico según tipo ────────────────────────
                // Callback<T> es Copy — se pasa directamente sin clonar.
                match *selected_type.read() {
                    ItemType::Weapon     => rsx! { WeaponForm     { on_save, save_label: btn_label } },
                    ItemType::Armour     => rsx! { ArmourForm     { on_save, save_label: btn_label } },
                    ItemType::Consumable => rsx! { ConsumableForm { on_save, save_label: btn_label } },
                    ItemType::Misc       => rsx! { MiscForm       { on_save, save_label: btn_label } },
                    _                    => rsx! {},
                }

                // ── Estado guardado ───────────────────────────────────────
                if *saving.read() {
                    p { style: "font-size:0.75rem; color:#fbbf24; text-align:center;",
                        "Guardando…"
                    }
                }
                if let Some(err) = save_error.read().clone() {
                    p { style: "font-size:0.75rem; color:#fca5a5; background:#1c0a0a;
                                border:1px solid #7f1d1d; border-radius:8px; padding:8px 12px;",
                        "⚠️ {err}"
                    }
                }
            }
        }
    }
}

// ── Conversión de ItemType a ItemCategory de inventario ───────────────────────

fn item_type_to_inventory_category(t: &ItemType) -> ItemCategory {
    match t {
        ItemType::Weapon     => ItemCategory::Weapon,
        ItemType::Armour     => ItemCategory::Armour,
        ItemType::Consumable => ItemCategory::Consumable,
        ItemType::Tool       => ItemCategory::Tool,
        ItemType::Treasure   => ItemCategory::Treasure,
        ItemType::Misc       => ItemCategory::Misc,
    }
}

// ── Serialización al vault ────────────────────────────────────────────────────

/// Serializa un campo de texto como escalar YAML correcto.
///
/// - Si el valor contiene saltos de línea → bloque literal `|` (indentado 2 espacios)
/// - Si contiene caracteres especiales → string entre comillas con escape mínimo
/// - En caso contrario → string entre comillas dobles
///
/// Evita que `gray_matter` falle al parsear frontmatter con saltos de línea
/// incrustados dentro de comillas dobles, que no es YAML válido.
fn yaml_scalar(lines: &mut Vec<String>, key: &str, value: &str) {
    if value.contains('\n') {
        // Bloque literal: cada línea indentada con 2 espacios
        lines.push(format!("{key}: |"));
        for line in value.lines() {
            lines.push(format!("  {line}"));
        }
    } else if value.contains('"') || value.contains(':') || value.contains('#') || value.starts_with(' ') {
        // Escapar backslashes y comillas dobles internas
        lines.push(format!("{key}: \"{}\"", value.replace('\\', "\\\\").replace('"', "\\\"")));
    } else {
        lines.push(format!("{key}: \"{value}\""));
    }
}

async fn write_item_to_vault(
    state: &crate::states::AppState,
    item_type: &ItemType,
    data: &NewItemData,
) -> Result<(), String> {
    // Construir el slug desde el nombre
    let slug = data
        .name
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    if slug.is_empty() {
        return Err("El nombre no puede estar vacío.".to_string());
    }

    // Construir el frontmatter YAML
    let mut yaml_lines = vec![
        "---".to_string(),
        "dnd_type: item".to_string(),
        format!("id: {slug}"),
        format!("name: \"{}\"", data.name.replace('"', "\\\"")),
        format!("category: {}", item_type.vault_category()),
        format!(
            "rarity: {}",
            if data.rarity.is_empty() { "common" } else { data.rarity.as_str() }
        ),
        format!(
            "source: \"{}\"",
            if data.source.is_empty() {
                "Homebrew".to_string()
            } else {
                data.source.replace('"', "\\\"")
            }
        ),
        "published: true".to_string(),
    ];

    if let Some(w) = data.weight {
        yaml_lines.push(format!("weight: {w}"));
    }

    if !data.description.is_empty() {
        yaml_scalar(&mut yaml_lines, "description", &data.description);
    }

    // Campos extra específicos del tipo
    for (k, v) in &data.extra {
        if !v.is_empty() {
            yaml_scalar(&mut yaml_lines, k, v);
        }
    }

    // Tags
    if !data.tags.is_empty() {
        yaml_lines.push("tags:".to_string());
        for tag in &data.tags {
            yaml_lines.push(format!("  - {tag}"));
        }
    }

    if !data.notes.is_empty() {
        yaml_scalar(&mut yaml_lines, "notes", &data.notes);
    }

    yaml_lines.push("---".to_string());
    yaml_lines.push(String::new());
    yaml_lines.push(format!("# {}", data.name));
    yaml_lines.push(String::new());
    if !data.description.is_empty() {
        yaml_lines.push(data.description.to_string());
    }

    let content = yaml_lines.join("\n");

    // Subcarpeta dentro del vault
    let subfolder = match item_type {
        ItemType::Weapon => "Items/Weapons",
        ItemType::Armour => "Items/Armour",
        ItemType::Consumable => "Items/Consumables",
        ItemType::Tool => "Items/Tools",
        ItemType::Treasure => "Items/Treasure",
        ItemType::Misc => "Items/Misc",
    };

    state
        .vault
        .write_note(subfolder, &slug, &content)
        .await
        .map_err(|e| e.to_string())
}
