// ═══════════════════════════════════════════════════════════════════════════
// player_inventory.rs — Panel de inventario de un personaje concreto
// ═══════════════════════════════════════════════════════════════════════════

use crate::states::SharedState;
use crate::vault::frontmatter::DndEntryType;
use dioxus::prelude::*;
use shared::api_types::inventory::{
    Currency, InventoryItem, ItemCategory, UpdateCurrencyRequest, UpdateItemRequest,
};
use std::time::Duration;
use uuid::Uuid;

pub const SELECT_STYLE: &str =
    "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
     background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none;
     box-sizing:border-box; appearance:none; cursor:pointer;";

pub const INPUT_STYLE: &str =
    "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
     background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none;
     box-sizing:border-box;";

// ─── Filtros de categoría ────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub enum CategoryFilter {
    All,
    Weapon,
    Armour,
    Consumable,
    Tool,
    Treasure,
    Accessory,
    Misc,
}

impl CategoryFilter {
    pub fn label(&self) -> &'static str {
        match self {
            Self::All        => "Todos",
            Self::Weapon     => "Armas",
            Self::Armour     => "Armaduras",
            Self::Consumable => "Consumibles",
            Self::Tool       => "Herramientas",
            Self::Treasure   => "Tesoros",
            Self::Accessory  => "Accesorios",
            Self::Misc       => "Misc",
        }
    }
    pub fn all_variants() -> &'static [CategoryFilter] {
        &[
            CategoryFilter::All,
            CategoryFilter::Weapon,
            CategoryFilter::Armour,
            CategoryFilter::Consumable,
            CategoryFilter::Tool,
            CategoryFilter::Treasure,
            CategoryFilter::Accessory,
            CategoryFilter::Misc,
        ]
    }
    pub fn matches(&self, cat: &ItemCategory) -> bool {
        match self {
            Self::All        => true,
            Self::Weapon     => *cat == ItemCategory::Weapon,
            Self::Armour     => *cat == ItemCategory::Armour,
            Self::Consumable => *cat == ItemCategory::Consumable,
            Self::Tool       => *cat == ItemCategory::Tool,
            Self::Treasure   => *cat == ItemCategory::Treasure,
            Self::Accessory  => *cat == ItemCategory::Accessory,
            Self::Misc       => *cat == ItemCategory::Misc,
        }
    }
}

// ─── Panel principal de inventario ──────────────────────────────────────────

#[component]
pub fn InventoryPanel(
    character_id: Uuid,
    initial_items: Vec<InventoryItem>,
    initial_currency: Currency,
) -> Element {
    let state = consume_context::<SharedState>().0;

    let mut live_items: Signal<Vec<InventoryItem>> = use_signal(|| initial_items.clone());
    let mut live_currency: Signal<Currency> = use_signal(|| initial_currency.clone());

    // Reset al cambiar de personaje
    {
        let ri = initial_items.clone();
        let rc = initial_currency.clone();
        use_effect(move || {
            live_items.set(ri.clone());
            live_currency.set(rc.clone());
        });
    }

    // Polling 4s
    {
        let sp = state.clone();
        use_future(move || {
            let s = sp.clone();
            async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(4)).await;
                    if let Ok((items, currency)) = s.persistence.get_inventory(character_id).await {
                        live_items.set(items);
                        live_currency.set(currency);
                    }
                }
            }
        });
    }

    let mut cat_filter: Signal<CategoryFilter> = use_signal(|| CategoryFilter::All);
    let mut editing_currency: Signal<bool> = use_signal(|| false);
    let mut show_vault_items: Signal<bool> = use_signal(|| false);
    let mut show_quick_add: Signal<bool> = use_signal(|| false);
    let mut vault_items: Signal<Vec<serde_json::Value>> = use_signal(Vec::new);
    let mut vault_search = use_signal(String::new);
    let mut vault_cat_filter = use_signal(|| "all".to_string());

    let lc0 = initial_currency.clone();
    let mut e_cp = use_signal(|| lc0.copper);
    let mut e_sp = use_signal(|| lc0.silver);
    let mut e_ep = use_signal(|| lc0.electrum);
    let mut e_gp = use_signal(|| lc0.gold);
    let mut e_pp = use_signal(|| lc0.platinum);

    {
        let lc = live_currency.read().clone();
        use_effect(move || {
            e_cp.set(lc.copper);
            e_sp.set(lc.silver);
            e_ep.set(lc.electrum);
            e_gp.set(lc.gold);
            e_pp.set(lc.platinum);
            editing_currency.set(false);
        });
    }

    let sc = state.clone();
    let save_currency = move |_| {
        let req = UpdateCurrencyRequest {
            copper:   Some(*e_cp.read()),
            silver:   Some(*e_sp.read()),
            electrum: Some(*e_ep.read()),
            gold:     Some(*e_gp.read()),
            platinum: Some(*e_pp.read()),
        };
        let s = sc.clone();
        spawn(async move {
            if let Ok(updated) = s.persistence.update_currency(character_id, req).await {
                live_currency.set(updated);
            }
        });
        editing_currency.set(false);
    };

    let cancel_currency = move |_| {
        let cur = live_currency.read().clone();
        e_cp.set(cur.copper);
        e_sp.set(cur.silver);
        e_ep.set(cur.electrum);
        e_gp.set(cur.gold);
        e_pp.set(cur.platinum);
        editing_currency.set(false);
    };

    // ── Snapshots para render ───────────────────────────────────────────────
    // Fix E0597: filtered es un Vec<InventoryItem> completamente owned — se
    // construye leyendo live_items una sola vez y soltando el borrow antes de
    // entrar al rsx!, de modo que los closures de ItemRow pueden capturarlo
    // con lifetime 'static.
    let currency = live_currency.read().clone();
    let cf = cat_filter.read().clone();
    let filtered: Vec<InventoryItem> = live_items
        .read()
        .iter()
        .filter(|i| cf.matches(&i.category))
        .cloned()
        .collect();
    let total_w: f32 = filtered
        .iter()
        .map(|i| i.weight.unwrap_or(0.0) * i.quantity as f32)
        .sum();
    let filtered_count = filtered.len();
    let is_ed_cur = *editing_currency.read();

    let sv_btn = state.clone();

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",

            // ── Monedas ─────────────────────────────────────────────────────
            div { style: "display:flex; align-items:center; justify-content:space-between; gap:8px;",
                CurrencyRow { currency: currency.clone() }
                if !is_ed_cur {
                    button {
                        style: "flex-shrink:0; padding:4px 12px; font-size:0.65rem; border-radius:8px;
                                cursor:pointer; background:#1a1208; color:#f59e0b; border:1px solid #78350f;",
                        onclick: move |_| {
                            let cur = live_currency.read().clone();
                            e_cp.set(cur.copper); e_sp.set(cur.silver);
                            e_ep.set(cur.electrum); e_gp.set(cur.gold); e_pp.set(cur.platinum);
                            editing_currency.set(true);
                        },
                        "✏️ Monedas"
                    }
                }
            }

            if is_ed_cur {
                div {
                    style: "background:#111110; border:1px solid #b45309; border-radius:14px;
                            padding:14px 16px; display:flex; flex-direction:column; gap:10px;",
                    div { style: "display:grid; grid-template-columns:repeat(5,1fr); gap:8px;",
                        CurrencyInput { label: "PC", accent: "#f97316", value: e_cp }
                        CurrencyInput { label: "PP", accent: "#d6d3d1", value: e_sp }
                        CurrencyInput { label: "PE", accent: "#22d3ee", value: e_ep }
                        CurrencyInput { label: "PO", accent: "#fbbf24", value: e_gp }
                        CurrencyInput { label: "Pt", accent: "#c084fc", value: e_pp }
                    }
                    div { style: "display:flex; justify-content:flex-end; gap:8px;",
                        button {
                            style: "padding:5px 14px; font-size:0.7rem; border-radius:8px; cursor:pointer;
                                    background:#1c1917; color:#78716c; border:1px solid #292524;",
                            onclick: cancel_currency,
                            "Cancelar"
                        }
                        button {
                            style: "padding:5px 14px; font-size:0.7rem; font-weight:600; border-radius:8px;
                                    cursor:pointer; background:#92400e; color:#fef3c7; border:1px solid #b45309;",
                            onclick: save_currency,
                            "💾 Guardar"
                        }
                    }
                }
            }

            // ── Filtros ─────────────────────────────────────────────────────
            div { style: "display:flex; gap:4px; flex-wrap:wrap;",
                for f in CategoryFilter::all_variants() {
                    {
                        let is_active = cf == *f;
                        let f2 = f.clone();
                        let bg  = if is_active { "#78350f" } else { "#111110" };
                        let col = if is_active { "#fef3c7" } else { "#78716c" };
                        let brd = if is_active { "#b45309" } else { "#292524" };
                        rsx! {
                            button {
                                style: "padding:3px 10px; font-size:0.62rem; border-radius:6px;
                                        background:{bg}; color:{col}; border:1px solid {brd}; cursor:pointer;",
                                onclick: move |_| cat_filter.set(f2.clone()),
                                "{f.label()}"
                            }
                        }
                    }
                }
            }

            // ── Acciones ────────────────────────────────────────────────────
            div { style: "display:flex; justify-content:flex-end; gap:8px;",
                button {
                    style: "padding:4px 12px; font-size:0.65rem; border-radius:8px; cursor:pointer;
                            background:#1a1208; color:#fbbf24; border:1px solid #78350f;",
                    onclick: move |_| show_quick_add.set(true),
                    "✏️ Objeto rápido"
                }
                button {
                    style: "padding:4px 12px; font-size:0.65rem; border-radius:8px; cursor:pointer;
                            background:#071a0e; color:#34d399; border:1px solid #065f46;",
                    onclick: move |_| {
                        let sv = sv_btn.clone();
                        show_vault_items.set(true);
                        spawn(async move {
                            if let Ok(entries) = sv.vault.entries_by_kind(DndEntryType::Item).await {
                                let list = entries.iter().map(|e| {
                                    let raw_bonuses = e.frontmatter.extra.get("stat_bonuses");
                                    let stat_bonuses_val = match raw_bonuses {
                                        None => serde_json::Value::Array(vec![]),
                                        Some(serde_json::Value::Array(arr)) => serde_json::Value::Array(arr.clone()),
                                        Some(serde_json::Value::String(s)) => {
                                            serde_json::from_str(s).unwrap_or(serde_json::Value::Array(vec![]))
                                        }
                                        Some(other) => other.clone(),
                                    };
                                    serde_json::json!({
                                        "name": e.display_name(),
                                        "category": e.frontmatter.extra.get("category")
                                            .and_then(|v| v.as_str()).unwrap_or("misc"),
                                        "description": e.frontmatter.extra.get("description")
                                            .and_then(|v| v.as_str()).unwrap_or(""),
                                        "weight": e.frontmatter.extra.get("weight"),
                                        "notes": e.frontmatter.extra.get("notes")
                                            .and_then(|v| v.as_str()).unwrap_or(""),
                                        "stat_bonuses": stat_bonuses_val,
                                    })
                                }).collect();
                                vault_items.set(list);
                            }
                        });
                    },
                    "📜 Vault"
                }
            }

            // ── Tabla de items ───────────────────────────────────────────────
            if filtered.is_empty() {
                div {
                    style: "display:flex; align-items:center; justify-content:center;
                            height:60px; font-size:0.78rem; color:#44403c;
                            border:1px dashed #292524; border-radius:12px;",
                    "Sin objetos en esta categoría."
                }
            } else {
                div { style: "border:1px solid #292524; border-radius:14px; overflow:hidden;",
                    // Cabecera
                    div {
                        style: "display:grid; grid-template-columns:5fr 3fr 2fr 2fr 2fr;
                                padding:7px 16px; background:#111110; border-bottom:1px solid #1c1917;
                                font-size:0.62rem; color:#57534e; text-transform:uppercase; letter-spacing:0.09em;",
                        div { "Objeto" }
                        div { "Tipo" }
                        div { style: "text-align:center;", "Cant." }
                        div { style: "text-align:center;", "Equip." }
                        div { style: "text-align:right;", "Peso" }
                    }
                    // Fix E0507 + E0597: iteramos sobre un Vec owned de snapshots.
                    // Para cada fila pre-clonamos state en dos Arc independientes
                    // (su_upd para on_update, su_del para on_delete) — así ninguna
                    // closure intenta mover el mismo Arc dos veces dentro del mismo
                    // FnMut del loop.
                    for item in filtered {
                        {
                            let iid      = item.id;
                            let item_own = item.clone();
                            let su_upd   = state.clone();
                            let su_del   = state.clone();
                            rsx! {
                                ItemRow {
                                    item: item_own,
                                    on_update: move |req: UpdateItemRequest| {
                                        let s = su_upd.clone();
                                        spawn(async move {
                                            if let Ok(updated) = s.persistence
                                                .update_item(character_id, iid, req).await
                                            {
                                                let mut v = live_items.write();
                                                if let Some(pos) = v.iter().position(|i| i.id == iid) {
                                                    v[pos] = updated;
                                                }
                                            }
                                        });
                                    },
                                    on_delete: move |id: Uuid| {
                                        let s = su_del.clone();
                                        spawn(async move {
                                            let _ = s.persistence.delete_item(character_id, id).await;
                                            live_items.write().retain(|i| i.id != id);
                                        });
                                    },
                                }
                            }
                        }
                    }
                }
                p { style: "font-size:0.68rem; color:#44403c; text-align:right; margin:0;",
                    "{filtered_count} objeto(s) · {total_w:.1} lb."
                }
            }
        }

        if *show_quick_add.read() {
            QuickAddItemModal {
                character_id,
                on_close: move || show_quick_add.set(false),
                on_added: move |item: InventoryItem| {
                    live_items.write().push(item);
                    show_quick_add.set(false);
                },
            }
        }

        if *show_vault_items.read() {
            VaultItemsModal {
                character_id,
                items: vault_items.read().clone(),
                search: vault_search,
                cat_filter: vault_cat_filter,
                on_close: move || {
                    show_vault_items.set(false);
                    vault_search.set(String::new());
                    vault_cat_filter.set("all".to_string());
                    vault_items.set(vec![]);
                },
                on_added: move |item: InventoryItem| {
                    live_items.write().push(item);
                },
            }
        }
    }
}

// ─── Modal: Crear objeto rápido ──────────────────────────────────────────────

#[component]
fn QuickAddItemModal(
    character_id: Uuid,
    on_close: EventHandler<()>,
    on_added: EventHandler<InventoryItem>,
) -> Element {
    let state = consume_context::<SharedState>().0;

    let mut q_name        = use_signal(String::new);
    let mut q_desc        = use_signal(String::new);
    let mut q_category    = use_signal(|| "misc".to_string());
    let mut q_quantity: Signal<u32>  = use_signal(|| 1);
    let mut q_weight      = use_signal(String::new);
    let mut q_notes       = use_signal(String::new);
    let mut q_equipped: Signal<bool> = use_signal(|| false);

    let save = move |_| {
        let name = q_name.read().trim().to_string();
        if name.is_empty() { return; }

        let cat    = category_from_str(&q_category.read());
        let weight = q_weight.read().parse::<f32>().ok();
        let item   = InventoryItem {
            id: uuid::Uuid::new_v4(),
            name: name.clone(),
            category: cat,
            description: q_desc.read().clone(),
            quantity: *q_quantity.read(),
            weight,
            equipped: *q_equipped.read(),
            accessory_type: None,
            stat_bonuses: vec![],
            notes: q_notes.read().clone(),
        };

        let s     = state.clone();
        let item2 = item.clone();
        spawn(async move {
            let _ = s.persistence.add_item(character_id, item2).await;
        });
        on_added.call(item);
    };

    rsx! {
        div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.78);
                    display:flex; align-items:center; justify-content:center; z-index:60;",
            div {
                style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                        padding:24px; width:460px; max-width:95vw; max-height:88vh;
                        overflow-y:auto; display:flex; flex-direction:column; gap:14px;",

                h3 { style: "font-size:1rem; font-weight:700; color:#fef3c7; margin:0;",
                    "✏️ Objeto rápido" }
                p  { style: "font-size:0.72rem; color:#78716c; margin:0;",
                    "Añade un objeto al inventario sin necesidad de crearlo en el vault." }

                QuickField { label: "Nombre *",
                    input { style: INPUT_STYLE, placeholder: "Nombre del objeto",
                        value: "{q_name}", oninput: move |e| q_name.set(e.value()) }
                }

                div { style: "display:grid; grid-template-columns:1fr 1fr; gap:8px;",
                    QuickField { label: "Categoría",
                        select {
                            style: SELECT_STYLE,
                            oninput: move |e| q_category.set(e.value()),
                            option { value: "misc",       "📦 Misc" }
                            option { value: "weapon",     "⚔️ Arma" }
                            option { value: "armour",     "🛡️ Armadura" }
                            option { value: "consumable", "🧪 Consumible" }
                            option { value: "tool",       "🔧 Herramienta" }
                            option { value: "treasure",   "💎 Tesoro" }
                            option { value: "accessory",  "💍 Accesorio" }
                        }
                    }
                    QuickField { label: "Cantidad",
                        input {
                            r#type: "number", min: "1", style: INPUT_STYLE,
                            value: "{q_quantity}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u32>() { q_quantity.set(v.max(1)); }
                            }
                        }
                    }
                }

                div { style: "display:grid; grid-template-columns:1fr 1fr; gap:8px;",
                    QuickField { label: "Peso (lb)",
                        input { r#type: "number", style: INPUT_STYLE, placeholder: "0.0",
                            value: "{q_weight}", oninput: move |e| q_weight.set(e.value()) }
                    }
                    QuickField { label: "Equipado",
                        div { style: "display:flex; align-items:center; gap:8px; padding:9px 0;",
                            input {
                                r#type: "checkbox", checked: "{q_equipped}",
                                onchange: move |e| q_equipped.set(e.checked()),
                            }
                            span { style: "font-size:0.78rem; color:#a8a29e;", "Equipado ahora" }
                        }
                    }
                }

                QuickField { label: "Descripción",
                    textarea {
                        style: "width:100%; padding:8px 10px; font-size:0.78rem; border-radius:8px;
                                background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                outline:none; resize:none; min-height:60px; font-family:inherit;
                                box-sizing:border-box;",
                        value: "{q_desc}", oninput: move |e| q_desc.set(e.value())
                    }
                }

                QuickField { label: "Notas",
                    textarea {
                        style: "width:100%; padding:8px 10px; font-size:0.78rem; border-radius:8px;
                                background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                outline:none; resize:none; min-height:44px; font-family:inherit;
                                box-sizing:border-box;",
                        value: "{q_notes}", oninput: move |e| q_notes.set(e.value())
                    }
                }

                div { style: "display:flex; justify-content:flex-end; gap:8px;",
                    button {
                        style: "padding:7px 16px; font-size:0.72rem; border-radius:8px; cursor:pointer;
                                background:#1c1917; color:#78716c; border:1px solid #292524;",
                        onclick: move |_| on_close.call(()), "Cancelar"
                    }
                    button {
                        style: "padding:7px 16px; font-size:0.72rem; font-weight:600; border-radius:8px;
                                cursor:pointer; background:#92400e; color:#fef3c7; border:1px solid #b45309;",
                        onclick: save, "➕ Añadir"
                    }
                }
            }
        }
    }
}

// ─── Modal: Items del vault ──────────────────────────────────────────────────

#[component]
fn VaultItemsModal(
    character_id: Uuid,
    items: Vec<serde_json::Value>,
    mut search: Signal<String>,
    mut cat_filter: Signal<String>,
    on_close: EventHandler<()>,
    on_added: EventHandler<InventoryItem>,
) -> Element {
    let state = consume_context::<SharedState>().0;

    let search_lower = search.read().to_lowercase();
    let cat_f = cat_filter.read().clone();

    let filtered: Vec<_> = items.iter().filter(|item| {
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
        let cat  = item.get("category").and_then(|v| v.as_str()).unwrap_or("misc");
        let name_ok = search_lower.is_empty() || name.contains(&search_lower);
        let cat_ok  = cat_f == "all" || cat == cat_f;
        name_ok && cat_ok
    }).collect();

    rsx! {
        div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.82);
                    display:flex; align-items:center; justify-content:center; z-index:60;",
            div {
                style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                        padding:24px; width:600px; max-width:95vw; max-height:82vh;
                        display:flex; flex-direction:column; gap:12px;",

                div { style: "display:flex; justify-content:space-between; align-items:center;",
                    h3 { style: "font-size:1rem; font-weight:700; color:#fef3c7; margin:0;",
                        "📜 Items del Vault" }
                    button {
                        style: "padding:4px 12px; font-size:0.7rem; border-radius:8px; cursor:pointer;
                                background:#1c1917; color:#78716c; border:1px solid #292524;",
                        onclick: move |_| on_close.call(()), "✕ Cerrar"
                    }
                }

                input {
                    style: "padding:7px 12px; font-size:0.78rem; border-radius:9px;
                            background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                            outline:none; width:100%; box-sizing:border-box;",
                    placeholder: "🔍 Buscar por nombre…",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }

                select {
                    style: SELECT_STYLE,
                    oninput: move |e| cat_filter.set(e.value()),
                    option { value: "all",        "Todos" }
                    option { value: "weapon",     "⚔️ Armas" }
                    option { value: "armour",     "🛡️ Armaduras" }
                    option { value: "consumable", "🧪 Consumibles" }
                    option { value: "tool",       "🔧 Herramientas" }
                    option { value: "treasure",   "💎 Tesoros" }
                    option { value: "misc",       "📦 Misc" }
                }

                p { style: "font-size:0.66rem; color:#57534e; margin:0;",
                    "{filtered.len()} resultado(s)" }

                if items.is_empty() {
                    p { style: "color:#44403c; font-size:0.8rem; text-align:center; padding:24px 0;",
                        "No hay items con dnd_type:item en el vault, o el vault no está configurado." }
                } else if filtered.is_empty() {
                    p { style: "color:#44403c; font-size:0.8rem; text-align:center; padding:24px 0;",
                        "Ningún objeto coincide con los filtros." }
                } else {
                    div { style: "overflow-y:auto; max-height:360px; display:flex; flex-direction:column; gap:6px; padding-right:4px;",
                        for vi_item in filtered {
                            {
                                let name    = vi_item.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let cat_str = vi_item.get("category").and_then(|v| v.as_str()).unwrap_or("misc").to_string();
                                let desc    = vi_item.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let notes   = vi_item.get("notes").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let weight  = vi_item.get("weight").and_then(|v| v.as_f64()).map(|w| w as f32);

                                let (cat_label, cat_col) = match cat_str.as_str() {
                                    "weapon"     => ("⚔️ Arma",       "#fca5a5"),
                                    "armour"     => ("🛡️ Armadura",   "#93c5fd"),
                                    "consumable" => ("🧪 Consumible", "#86efac"),
                                    "tool"       => ("🔧 Herramienta","#fde68a"),
                                    "treasure"   => ("💎 Tesoro",     "#e9d5ff"),
                                    _            => ("📦 Misc",       "#a8a29e"),
                                };

                                let sa      = state.clone();
                                let name2   = name.clone();
                                let cat_str2 = cat_str.clone();
                                let desc2   = desc.clone();
                                let notes2  = notes.clone();

                                rsx! {
                                    div {
                                        style: "display:flex; justify-content:space-between; align-items:center;
                                                padding:9px 14px; background:#111110; border-radius:10px;
                                                border:1px solid #1c1917; gap:10px;",
                                        div { style: "flex:1; min-width:0; display:flex; flex-direction:column; gap:2px;",
                                            div { style: "display:flex; align-items:center; gap:8px;",
                                                span { style: "font-size:0.8rem; font-weight:600; color:#fef3c7;
                                                               white-space:nowrap; overflow:hidden; text-overflow:ellipsis;",
                                                    "{name}" }
                                                span { style: "font-size:0.6rem; color:{cat_col}; white-space:nowrap;",
                                                    "{cat_label}" }
                                            }
                                            if !desc.is_empty() {
                                                p { style: "font-size:0.68rem; color:#78716c; margin:0;
                                                            white-space:nowrap; overflow:hidden; text-overflow:ellipsis;
                                                            max-width:380px;",
                                                    "{desc}" }
                                            }
                                        }
                                        button {
                                            style: "padding:5px 12px; font-size:0.65rem; border-radius:8px;
                                                    cursor:pointer; border:1px solid #166534;
                                                    background:#052e16; color:#34d399; white-space:nowrap; flex-shrink:0;",
                                            onclick: move |_| {
                                                let cat = category_from_str(&cat_str2);
                                                let item = InventoryItem {
                                                    id: uuid::Uuid::new_v4(),
                                                    name: name2.clone(),
                                                    category: cat,
                                                    description: desc2.clone(),
                                                    quantity: 1,
                                                    weight,
                                                    equipped: false,
                                                    accessory_type: None,
                                                    stat_bonuses: vec![],
                                                    notes: notes2.clone(),
                                                };
                                                let s     = sa.clone();
                                                let item2 = item.clone();
                                                spawn(async move {
                                                    let _ = s.persistence.add_item(character_id, item2).await;
                                                });
                                                on_added.call(item);
                                            },
                                            "＋ Añadir"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Fila de item con edición inline ────────────────────────────────────────

#[component]
fn ItemRow(
    item: InventoryItem,
    on_update: EventHandler<UpdateItemRequest>,
    on_delete: EventHandler<Uuid>,
) -> Element {
    let mut expanded = use_signal(|| false);
    let mut editing  = use_signal(|| false);
    let mut e_qty:   Signal<u32>    = use_signal(|| item.quantity);
    let mut e_eq:    Signal<bool>   = use_signal(|| item.equipped);
    let mut e_notes: Signal<String> = use_signal(|| item.notes.clone());

    let total_w  = item.weight.unwrap_or(0.0) * item.quantity as f32;
    let cat_lbl  = item.category.label();
    let eq_col   = if item.equipped { "#34d399" } else { "#57534e" };
    let iid      = item.id;

    rsx! {
        div { style: "border-bottom:1px solid #1c1917;",

            // Fila principal
            div {
                style: "display:grid; grid-template-columns:5fr 3fr 2fr 2fr 2fr;
                        padding:9px 16px; align-items:center; cursor:pointer;",
                onclick: move |_| {
                    let v = *expanded.read();
                    expanded.set(!v);
                },

                div { style: "display:flex; align-items:center; gap:8px;",
                    span { style: "font-size:0.83rem; font-weight:500; color:#d6d3d1;
                                   overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                        "{item.name}"
                    }
                    button {
                        style: "padding:1px 7px; font-size:0.58rem; border-radius:5px; cursor:pointer;
                                background:#1a1208; color:#f59e0b; border:1px solid #78350f; flex-shrink:0;",
                        onclick: move |e| {
                            e.stop_propagation();
                            // Fix E0502: leer el valor primero, luego mutar —
                            // evita tener un borrow inmutable y uno mutable activos
                            // al mismo tiempo sobre la misma Signal.
                            let current = *editing.read();
                            editing.set(!current);
                        },
                        "✏️"
                    }
                    button {
                        style: "padding:1px 7px; font-size:0.58rem; border-radius:5px; cursor:pointer;
                                background:#200a0a; color:#f87171; border:1px solid #7f1d1d; flex-shrink:0;",
                        onclick: move |e| { e.stop_propagation(); on_delete.call(iid); },
                        "🗑"
                    }
                }

                div {
                    span {
                        style: "font-size:0.63rem; background:#1c1917; color:#78716c;
                                border-radius:6px; padding:3px 7px; border:1px solid #292524;",
                        "{cat_lbl}"
                    }
                }
                div { style: "font-size:0.78rem; color:#78716c; text-align:center;", "×{item.quantity}" }
                div { style: "font-size:0.72rem; color:{eq_col}; text-align:center;",
                    if item.equipped { "✔" } else { "—" }
                }
                div { style: "font-size:0.72rem; color:#57534e; text-align:right;", "{total_w:.1} lb" }
            }

            // Panel edición inline
            if *editing.read() {
                div {
                    style: "padding:10px 16px 12px; background:#111110; border-top:1px solid #1c1917;
                            display:flex; flex-direction:column; gap:10px;",
                    div { style: "display:grid; grid-template-columns:1fr 1fr 3fr; gap:10px; align-items:end;",
                        div { style: "display:flex; flex-direction:column; gap:4px;",
                            label { style: "font-size:0.6rem; color:#78716c;", "Cantidad" }
                            input {
                                r#type: "number", min: "0",
                                style: "padding:5px 8px; font-size:0.8rem; border-radius:7px;
                                        background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                        outline:none; box-sizing:border-box; width:100%;",
                                value: "{e_qty}",
                                oninput: move |ev| {
                                    if let Ok(v) = ev.value().parse::<u32>() { e_qty.set(v); }
                                }
                            }
                        }
                        div { style: "display:flex; flex-direction:column; gap:4px;",
                            label { style: "font-size:0.6rem; color:#78716c;", "Equipado" }
                            div { style: "display:flex; align-items:center; gap:6px; padding:6px 0;",
                                input {
                                    r#type: "checkbox", checked: "{e_eq}",
                                    onchange: move |ev| e_eq.set(ev.checked()),
                                }
                                span { style: "font-size:0.75rem; color:#a8a29e;",
                                    if *e_eq.read() { "Sí" } else { "No" }
                                }
                            }
                        }
                        div { style: "display:flex; flex-direction:column; gap:4px;",
                            label { style: "font-size:0.6rem; color:#78716c;", "Notas" }
                            input {
                                style: "padding:5px 8px; font-size:0.78rem; border-radius:7px;
                                        background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                        outline:none; box-sizing:border-box; width:100%;",
                                value: "{e_notes}",
                                oninput: move |ev| e_notes.set(ev.value())
                            }
                        }
                    }
                    div { style: "display:flex; justify-content:flex-end; gap:8px;",
                        button {
                            style: "padding:5px 12px; font-size:0.7rem; border-radius:7px; cursor:pointer;
                                    background:#1c1917; color:#78716c; border:1px solid #292524;",
                            onclick: move |_| {
                                e_qty.set(item.quantity);
                                e_eq.set(item.equipped);
                                e_notes.set(item.notes.clone());
                                editing.set(false);
                            },
                            "Cancelar"
                        }
                        button {
                            style: "padding:5px 12px; font-size:0.7rem; font-weight:600; border-radius:7px;
                                    cursor:pointer; background:#92400e; color:#fef3c7; border:1px solid #b45309;",
                            onclick: move |_| {
                                on_update.call(UpdateItemRequest {
                                    quantity: Some(*e_qty.read()),
                                    equipped: Some(*e_eq.read()),
                                    notes:    Some(e_notes.read().clone()),
                                });
                                editing.set(false);
                            },
                            "💾 Guardar"
                        }
                    }
                }
            }

            // Descripción expandida
            if *expanded.read() && !*editing.read() && !item.description.is_empty() {
                div {
                    style: "padding:8px 20px 12px; font-size:0.75rem; color:#a8a29e;
                            background:#111110; border-top:1px solid #1c1917;
                            white-space:pre-wrap; line-height:1.6;",
                    "{item.description}"
                }
            }
        }
    }
}

// ─── Widgets de monedas ──────────────────────────────────────────────────────

#[component]
pub fn CurrencyRow(currency: Currency) -> Element {
    rsx! {
        div { style: "display:flex; gap:8px; flex-wrap:wrap;",
            CurrencyChip { label: "PC", amount: currency.copper,   accent: "#f97316", bg: "#1a0e07", border: "#7c2d12" }
            CurrencyChip { label: "PP", amount: currency.silver,   accent: "#d6d3d1", bg: "#1a1917", border: "#44403c" }
            CurrencyChip { label: "PE", amount: currency.electrum, accent: "#22d3ee", bg: "#071a1f", border: "#0e7490" }
            CurrencyChip { label: "PO", amount: currency.gold,     accent: "#fbbf24", bg: "#1a1306", border: "#92400e" }
            CurrencyChip { label: "Pt", amount: currency.platinum, accent: "#c084fc", bg: "#130a1f", border: "#7e22ce" }
        }
    }
}

#[component]
fn CurrencyChip(
    label: &'static str,
    amount: u32,
    accent: &'static str,
    bg: &'static str,
    border: &'static str,
) -> Element {
    rsx! {
        div {
            style: "background:{bg}; border:1px solid {border}; border-radius:12px;
                    padding:7px 14px; display:flex; align-items:center; gap:8px;",
            span { style: "font-size:0.68rem; font-weight:700; color:{accent};", "{label}" }
            span { style: "font-size:0.92rem; font-weight:600; color:#e7e5e4;", "{amount}" }
        }
    }
}

#[component]
fn CurrencyInput(label: &'static str, accent: &'static str, mut value: Signal<u32>) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; align-items:center; gap:5px;",
            span { style: "font-size:0.65rem; font-weight:700; color:{accent};", "{label}" }
            input {
                r#type: "number", min: "0",
                style: "width:100%; background:#1c1917; border:1px solid #44403c;
                        border-radius:8px; padding:5px 4px; text-align:center;
                        font-size:0.95rem; font-weight:700; color:#e7e5e4;
                        outline:none; box-sizing:border-box;",
                value: "{value}",
                oninput: move |e| { if let Ok(v) = e.value().parse::<u32>() { value.set(v); } },
            }
        }
    }
}

// ─── Helpers de formulario ───────────────────────────────────────────────────

#[component]
fn QuickField(label: &'static str, children: Element) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:4px;",
            label { style: "font-size:0.65rem; color:#78716c; text-transform:uppercase; letter-spacing:0.07em;",
                "{label}" }
            { children }
        }
    }
}

// ─── Conversión de string a ItemCategory ─────────────────────────────────────

pub fn category_from_str(s: &str) -> ItemCategory {
    match s {
        "weapon"     => ItemCategory::Weapon,
        "armour"     => ItemCategory::Armour,
        "consumable" => ItemCategory::Consumable,
        "tool"       => ItemCategory::Tool,
        "treasure"   => ItemCategory::Treasure,
        "accessory"  => ItemCategory::Accessory,
        _            => ItemCategory::Misc,
    }
}
