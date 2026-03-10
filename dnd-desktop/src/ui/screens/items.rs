// ═══════════════════════════════════════════════════════════════════════════
// items.rs — Pantalla "Objetos del Vault"
// ═══════════════════════════════════════════════════════════════════════════

use crate::states::SharedState;
use crate::vault::frontmatter::DndEntryType;
use dioxus::prelude::*;

use super::items_form::CreateItemModal;

const SELECT_STYLE: &str =
    "padding:8px 14px; font-size:0.82rem; border-radius:10px;
     background:#1c1917; border:1px solid #292524; color:#e7e5e4;
     outline:none; appearance:none; cursor:pointer;";

// ─── Filtro de categoría ─────────────────────────────────────────────────────

#[derive(Clone, PartialEq, Default)]
enum CatFilter {
    #[default]
    All,
    Weapon,
    Armour,
    Consumable,
    Tool,
    Treasure,
    Misc,
}

impl CatFilter {
    fn label(&self) -> &'static str {
        match self {
            Self::All        => "Todos",
            Self::Weapon     => "Armas",
            Self::Armour     => "Armaduras",
            Self::Consumable => "Consumibles",
            Self::Tool       => "Herramientas",
            Self::Treasure   => "Tesoros",
            Self::Misc       => "Misc",
        }
    }
    fn matches(&self, cat: &str) -> bool {
        match self {
            Self::All        => true,
            Self::Weapon     => cat == "weapon",
            Self::Armour     => cat == "armour",
            Self::Consumable => cat == "consumable",
            Self::Tool       => cat == "tool",
            Self::Treasure   => cat == "treasure",
            Self::Misc       => cat == "misc",
        }
    }
    fn all() -> &'static [CatFilter] {
        &[
            CatFilter::All, CatFilter::Weapon, CatFilter::Armour,
            CatFilter::Consumable, CatFilter::Tool, CatFilter::Treasure, CatFilter::Misc,
        ]
    }
}

// ─── Pantalla principal ──────────────────────────────────────────────────────

#[component]
pub fn ItemsScreen() -> Element {
    let state = use_context::<SharedState>();

    let mut items: Signal<Vec<serde_json::Value>> = use_signal(Vec::new);
    let mut loading    = use_signal(|| true);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    let mut search      = use_signal(String::new);
    let mut cat_filter  = use_signal(CatFilter::default);
    let mut show_create = use_signal(|| false);
    let mut editing_item: Signal<Option<serde_json::Value>> = use_signal(|| None);

    // Cargar al montar
    let st = state.clone();
    use_effect(move || {
        let inner = st.0.clone();
        spawn(async move {
            reload_items(inner, &mut items, &mut loading, &mut error_msg).await;
        });
    });

    // Fix E0596: use_callback produce un Callback que es Copy — se puede pasar
    // directamente a on_created / on_saved sin necesidad de .clone() ni mut.
    // Un closure suelto (let reload = move || {...}) no es Clone, pero
    // Callback<()> sí lo es porque Dioxus lo almacena en el árbol de hooks.
    let st2 = state.clone();
    let reload = use_callback(move |()| {
        let inner = st2.0.clone();
        show_create.set(false);
        editing_item.set(None);
        spawn(async move {
            reload_items(inner, &mut items, &mut loading, &mut error_msg).await;
        });
    });

    // Filtrado
    let search_val = search.read().to_lowercase();
    let cat_val    = cat_filter.read().clone();
    let all_items  = items.read().clone();
    let filtered: Vec<serde_json::Value> = all_items.into_iter().filter(|i| {
        let name = i.get("name").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
        let cat  = i.get("category").and_then(|v| v.as_str()).unwrap_or("misc");
        (search_val.is_empty() || name.contains(&search_val)) && cat_val.matches(cat)
    }).collect();

    rsx! {
        div { style: "min-height:100vh; padding:32px 48px; background:#0c0a09; color:#e7e5e4;",

            // ── Cabecera ─────────────────────────────────────────────────
            div { style: "display:flex; justify-content:space-between; align-items:center; margin-bottom:24px;",
                div {
                    h1 { style: "font-size:1.4rem; font-weight:700; color:#fef3c7; margin:0;",
                        "⚔️ Objetos del Vault" }
                    p  { style: "font-size:0.78rem; color:#78716c; margin:4px 0 0;",
                        "{filtered.len()} objeto(s) encontrado(s)" }
                }
                button {
                    style: "padding:8px 18px; font-size:0.78rem; font-weight:600;
                            border-radius:10px; cursor:pointer; border:1px solid #92400e;
                            background:#451a03; color:#fbbf24;",
                    onclick: move |_| show_create.set(true),
                    "+ Nuevo Objeto"
                }
            }

            // ── Búsqueda + filtros ────────────────────────────────────────
            div { style: "display:flex; flex-direction:column; gap:10px; margin-bottom:20px;",
                input {
                    style: "padding:8px 14px; font-size:0.82rem; border-radius:10px;
                            background:#1c1917; border:1px solid #292524; color:#e7e5e4;
                            width:320px; outline:none;",
                    placeholder: "🔍 Buscar por nombre…",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }
                div { style: "display:flex; gap:6px; flex-wrap:wrap;",
                    for filter in CatFilter::all() {
                        {
                            let f2 = filter.clone();
                            let active = *cat_filter.read() == *filter;
                            let (bg, col, brd) = if active {
                                ("#451a03","#fbbf24","#92400e")
                            } else {
                                ("#1c1917","#a8a29e","#292524")
                            };
                            rsx! {
                                button {
                                    style: "padding:4px 12px; font-size:0.68rem; border-radius:20px;
                                            background:{bg}; color:{col}; border:1px solid {brd}; cursor:pointer;",
                                    onclick: move |_| cat_filter.set(f2.clone()),
                                    "{filter.label()}"
                                }
                            }
                        }
                    }
                }
            }

            // ── Cuerpo ────────────────────────────────────────────────────
            if *loading.read() {
                div { style: "text-align:center; color:#78716c; padding:60px 0;",
                    "Cargando items del vault…" }
            } else if let Some(err) = error_msg.read().clone() {
                div { style: "background:#1c0a0a; border:1px solid #7f1d1d; border-radius:12px;
                              padding:16px 20px; color:#fca5a5; font-size:0.82rem;",
                    "⚠️ Error: {err}" }
            } else if filtered.is_empty() {
                div { style: "display:flex; flex-direction:column; align-items:center;
                              justify-content:center; gap:12px; padding:80px 0;",
                    span { style: "font-size:2.5rem;", "📦" }
                    p { style: "color:#57534e; font-size:0.9rem; margin:0;",
                        "No hay objetos que coincidan con los filtros." }
                }
            } else {
                div { style: "border:1px solid #292524; border-radius:14px; overflow:hidden;",
                    div {
                        style: "display:grid; grid-template-columns:3fr 2fr 1.5fr 1fr 1.5fr 2fr;
                                padding:8px 16px; background:#111110; border-bottom:1px solid #1c1917;
                                font-size:0.6rem; color:#57534e; text-transform:uppercase; letter-spacing:0.1em;",
                        div { "Nombre" }
                        div { "Categoría" }
                        div { "Daño / CA" }
                        div { style: "text-align:center;", "Peso" }
                        div { "Fuente" }
                        div { style: "text-align:right;", "Acciones" }
                    }
                    for item in filtered.iter() {
                        {
                            let item_for_edit = item.clone();
                            rsx! {
                                ItemRow {
                                    item: item.clone(),
                                    on_edit: move || editing_item.set(Some(item_for_edit.clone())),
                                }
                            }
                        }
                    }
                }
            }
        }

        // Callback es Copy — se pasa directamente sin .clone() ni mut.
        if *show_create.read() {
            CreateItemModal {
                on_close: move || show_create.set(false),
                on_created: move || reload.call(()),
            }
        }

        if let Some(item) = editing_item.read().clone() {
            EditItemModal {
                item,
                on_close: move || editing_item.set(None),
                on_saved: move || reload.call(()),
            }
        }
    }
}

// ─── Helper: cargar/recargar lista ───────────────────────────────────────────

async fn reload_items(
    inner: std::sync::Arc<crate::states::AppState>,
    items: &mut Signal<Vec<serde_json::Value>>,
    loading: &mut Signal<bool>,
    error_msg: &mut Signal<Option<String>>,
) {
    loading.set(true);
    match inner.vault.entries_by_kind(DndEntryType::Item).await {
        Ok(entries) => {
            let list = entries.iter().map(|e| serde_json::json!({
                "name":        e.display_name(),
                "slug":        e.slug,
                "subfolder":   e.relative_path.parent()
                                 .map(|p| p.to_string_lossy().to_string())
                                 .unwrap_or_default(),
                "category":    e.frontmatter.extra.get("category")
                                 .and_then(|v| v.as_str()).unwrap_or("misc"),
                "description": e.frontmatter.extra.get("description")
                                 .and_then(|v| v.as_str()).unwrap_or(""),
                "weight":      e.frontmatter.extra.get("weight").and_then(|v| v.as_f64()),
                "damage":      e.frontmatter.extra.get("damage")
                                 .and_then(|v| v.as_str()).unwrap_or(""),
                "rarity":      e.frontmatter.extra.get("rarity")
                                 .and_then(|v| v.as_str()).unwrap_or("common"),
                "source":      e.frontmatter.source.as_deref().unwrap_or("Homebrew"),
                "notes":       e.frontmatter.extra.get("notes")
                                 .and_then(|v| v.as_str()).unwrap_or(""),
                "tags":        e.frontmatter.tags,
            })).collect();
            items.set(list);
            error_msg.set(None);
        }
        Err(e) => error_msg.set(Some(e.to_string())),
    }
    loading.set(false);
}

// ─── Fila de la tabla ────────────────────────────────────────────────────────

#[component]
fn ItemRow(item: serde_json::Value, on_edit: EventHandler<()>) -> Element {
    let name   = str_field(&item, "name");
    let cat    = str_field(&item, "category");
    let damage = str_field(&item, "damage");
    let source = str_field(&item, "source");
    let desc   = str_field(&item, "description");
    let weight = item.get("weight").and_then(|v| v.as_f64());

    let mut expanded = use_signal(|| false);

    let (cat_label, cat_color) = cat_style(&cat);
    let weight_str = weight.map(|w| format!("{w:.1} lb")).unwrap_or_else(|| "—".to_string());
    let dmg_str    = if damage.is_empty() { "—".to_string() } else { damage };

    rsx! {
        div { style: "border-bottom:1px solid #1c1917;",
            div {
                style: "display:grid; grid-template-columns:3fr 2fr 1.5fr 1fr 1.5fr 2fr;
                        padding:10px 16px; font-size:0.78rem; color:#d6d3d1; align-items:center;
                        cursor:pointer;",
                onclick: move |_| {
                    let v = *expanded.read();
                    expanded.set(!v);
                },

                div { style: "font-weight:600; color:#fef3c7;", "{name}" }
                div { style: "color:{cat_color}; font-size:0.72rem;", "{cat_label}" }
                div { style: "color:#a8a29e; font-size:0.72rem;", "{dmg_str}" }
                div { style: "text-align:center; color:#78716c; font-size:0.7rem;", "{weight_str}" }
                div { style: "color:#57534e; font-size:0.68rem;", "{source}" }

                div { style: "display:flex; justify-content:flex-end;",
                    button {
                        style: "padding:3px 10px; font-size:0.65rem; border-radius:7px; cursor:pointer;
                                background:#1a1208; color:#f59e0b; border:1px solid #78350f;",
                        onclick: move |e| { e.stop_propagation(); on_edit.call(()); },
                        "✏️ Editar"
                    }
                }
            }
            if *expanded.read() && !desc.is_empty() {
                div {
                    style: "padding:8px 20px 12px; font-size:0.75rem; color:#a8a29e;
                            background:#111110; border-top:1px solid #1c1917;
                            white-space:pre-wrap; line-height:1.6;",
                    "{desc}"
                }
            }
        }
    }
}

// ─── Modal: editar objeto y serializar al vault ──────────────────────────────

#[component]
fn EditItemModal(
    item: serde_json::Value,
    on_close: EventHandler<()>,
    on_saved: EventHandler<()>,
) -> Element {
    let state = use_context::<SharedState>();

    let mut e_name   = use_signal(|| str_field(&item, "name"));
    let mut e_desc   = use_signal(|| str_field(&item, "description"));
    let mut e_notes  = use_signal(|| str_field(&item, "notes"));
    let mut e_damage = use_signal(|| str_field(&item, "damage"));
    let mut e_weight = use_signal(|| {
        item.get("weight").and_then(|v| v.as_f64())
            .map(|w| format!("{w}"))
            .unwrap_or_default()
    });
    let mut e_rarity = use_signal(|| str_field_or(&item, "rarity", "common"));
    let mut e_source = use_signal(|| str_field_or(&item, "source", "Homebrew"));
    let mut e_cat    = use_signal(|| str_field_or(&item, "category", "misc"));

    let slug      = str_field(&item, "slug");
    let subfolder = str_field(&item, "subfolder");

    let mut saving   = use_signal(|| false);
    let mut save_err: Signal<Option<String>> = use_signal(|| None);

    let st = state.clone();
    let save = move |_| {
        saving.set(true);
        save_err.set(None);

        let name   = e_name.read().clone();
        let desc   = e_desc.read().clone();
        let notes  = e_notes.read().clone();
        let damage = e_damage.read().clone();
        let weight = e_weight.read().parse::<f32>().ok();
        let rarity = e_rarity.read().clone();
        let source = e_source.read().clone();
        let cat    = e_cat.read().clone();
        let slug2  = slug.clone();
        let sub2   = subfolder.clone();
        let s      = st.0.clone();

        spawn(async move {
            let mut yaml = vec![
                "---".to_string(),
                "dnd_type: item".to_string(),
                format!("id: {slug2}"),
                format!("name: \"{}\"", name.replace('"', "\\\"")),
                format!("category: {cat}"),
                format!("rarity: {rarity}"),
                format!("source: \"{}\"", source.replace('"', "\\\"")),
                "published: true".to_string(),
            ];
            if let Some(w) = weight { yaml.push(format!("weight: {w}")); }
            if !desc.is_empty()   { yaml.push(format!("description: \"{}\"", desc.replace('"', "\\\""))); }
            if !damage.is_empty() { yaml.push(format!("damage: \"{}\"", damage.replace('"', "\\\""))); }
            if !notes.is_empty()  { yaml.push(format!("notes: \"{}\"", notes.replace('"', "\\\""))); }
            yaml.push("---".to_string());
            yaml.push(String::new());
            yaml.push(format!("# {name}"));
            if !desc.is_empty() { yaml.push(String::new()); yaml.push(desc.clone()); }
            let content = yaml.join("\n");

            match s.vault.write_note(&sub2, &slug2, &content).await {
                Ok(()) => on_saved.call(()),
                Err(e) => {
                    save_err.set(Some(e.to_string()));
                    saving.set(false);
                }
            }
        });
    };

    const FIELD_INPUT: &str =
        "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
         background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none; box-sizing:border-box;";
    const FIELD_SELECT: &str =
        "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
         background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none;
         appearance:none; cursor:pointer; box-sizing:border-box;";

    rsx! {
        div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.82);
                    display:flex; align-items:center; justify-content:center; z-index:100;",
            div {
                style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                        padding:28px; width:600px; max-width:96vw; max-height:90vh;
                        display:flex; flex-direction:column; gap:16px; overflow-y:auto;",

                div { style: "display:flex; justify-content:space-between; align-items:center;",
                    h2 { style: "font-size:1.05rem; font-weight:700; color:#fef3c7; margin:0;",
                        "✏️ Editar Objeto" }
                    button {
                        style: "padding:4px 12px; font-size:0.68rem; border-radius:8px; cursor:pointer;
                                background:#0c0a09; color:#78716c; border:1px solid #292524;",
                        onclick: move |_| on_close.call(()), "✕ Cerrar"
                    }
                }

                EditField { label: "Nombre",
                    input { style: FIELD_INPUT, value: "{e_name}",
                        oninput: move |e| e_name.set(e.value()) }
                }

                div { style: "display:grid; grid-template-columns:1fr 1fr; gap:10px;",
                    EditField { label: "Categoría",
                        select {
                            style: FIELD_SELECT,
                            oninput: move |e| e_cat.set(e.value()),
                            option { value: "weapon",     selected: e_cat.read().as_str() == "weapon",     "⚔️ Arma" }
                            option { value: "armour",     selected: e_cat.read().as_str() == "armour",     "🛡️ Armadura" }
                            option { value: "consumable", selected: e_cat.read().as_str() == "consumable", "🧪 Consumible" }
                            option { value: "tool",       selected: e_cat.read().as_str() == "tool",       "🔧 Herramienta" }
                            option { value: "treasure",   selected: e_cat.read().as_str() == "treasure",   "💎 Tesoro" }
                            option { value: "misc",       selected: e_cat.read().as_str() == "misc",       "📦 Misc" }
                        }
                    }
                    EditField { label: "Rareza",
                        select {
                            style: FIELD_SELECT,
                            oninput: move |e| e_rarity.set(e.value()),
                            option { value: "common",    selected: e_rarity.read().as_str() == "common",    "Common" }
                            option { value: "uncommon",  selected: e_rarity.read().as_str() == "uncommon",  "Uncommon" }
                            option { value: "rare",      selected: e_rarity.read().as_str() == "rare",      "Rare" }
                            option { value: "very_rare", selected: e_rarity.read().as_str() == "very_rare", "Very Rare" }
                            option { value: "legendary", selected: e_rarity.read().as_str() == "legendary", "Legendary" }
                            option { value: "artifact",  selected: e_rarity.read().as_str() == "artifact",  "Artifact" }
                        }
                    }
                }

                div { style: "display:grid; grid-template-columns:1fr 1fr 1fr; gap:10px;",
                    EditField { label: "Daño",
                        input { style: FIELD_INPUT, value: "{e_damage}", placeholder: "1d8 slashing",
                            oninput: move |e| e_damage.set(e.value()) }
                    }
                    EditField { label: "Peso (lb)",
                        input { r#type: "number", style: FIELD_INPUT, value: "{e_weight}",
                            oninput: move |e| e_weight.set(e.value()) }
                    }
                    EditField { label: "Fuente",
                        input { style: FIELD_INPUT, value: "{e_source}",
                            oninput: move |e| e_source.set(e.value()) }
                    }
                }

                EditField { label: "Descripción",
                    textarea {
                        style: "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
                                background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                outline:none; resize:vertical; min-height:80px;
                                font-family:inherit; box-sizing:border-box;",
                        value: "{e_desc}", oninput: move |e| e_desc.set(e.value())
                    }
                }

                EditField { label: "Notas",
                    textarea {
                        style: "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
                                background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                outline:none; resize:vertical; min-height:50px;
                                font-family:inherit; box-sizing:border-box;",
                        value: "{e_notes}", oninput: move |e| e_notes.set(e.value())
                    }
                }

                if *saving.read() {
                    p { style: "font-size:0.75rem; color:#fbbf24;", "Guardando en vault…" }
                }
                if let Some(err) = save_err.read().clone() {
                    p { style: "font-size:0.75rem; color:#fca5a5; background:#1c0a0a;
                                border:1px solid #7f1d1d; border-radius:8px; padding:8px 12px;",
                        "⚠️ {err}" }
                }

                div { style: "display:flex; justify-content:flex-end; gap:10px;",
                    button {
                        style: "padding:8px 18px; font-size:0.78rem; border-radius:10px; cursor:pointer;
                                background:#1c1917; color:#78716c; border:1px solid #292524;",
                        onclick: move |_| on_close.call(()), "Cancelar"
                    }
                    button {
                        style: "padding:8px 20px; font-size:0.78rem; font-weight:600; border-radius:10px;
                                cursor:pointer; background:#065f46; color:#34d399; border:1px solid #166534;",
                        onclick: save, "💾 Guardar en vault"
                    }
                }
            }
        }
    }
}

// ─── Widgets locales ─────────────────────────────────────────────────────────

#[component]
fn EditField(label: &'static str, children: Element) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:4px;",
            label { style: "font-size:0.65rem; color:#78716c; text-transform:uppercase; letter-spacing:0.07em;",
                "{label}" }
            { children }
        }
    }
}

// ─── Helpers de extracción de JSON ───────────────────────────────────────────

fn str_field(item: &serde_json::Value, key: &str) -> String {
    item.get(key).and_then(|v| v.as_str()).unwrap_or("").to_string()
}

fn str_field_or(item: &serde_json::Value, key: &str, default: &str) -> String {
    let v = item.get(key).and_then(|v| v.as_str()).unwrap_or("");
    if v.is_empty() { default.to_string() } else { v.to_string() }
}

fn cat_style(cat: &str) -> (&'static str, &'static str) {
    match cat {
        "weapon"     => ("⚔️ Arma",       "#fca5a5"),
        "armour"     => ("🛡️ Armadura",   "#93c5fd"),
        "consumable" => ("🧪 Consumible", "#86efac"),
        "tool"       => ("🔧 Herramienta","#fde68a"),
        "treasure"   => ("💎 Tesoro",     "#e9d5ff"),
        _            => ("📦 Misc",       "#a8a29e"),
    }
}
