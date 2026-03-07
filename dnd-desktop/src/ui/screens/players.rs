use crate::states::SharedState;
use chrono;
use dioxus::prelude::*;
use shared::api_types::inventory::{Currency, InventoryItem, ItemCategory, UpdateCurrencyRequest};
use shared::api_types::spells::{Spell, SpellSchool, SpellSlotLevel};
use shared::persistence::SavedCharacter;
use std::time::Duration;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

fn modifier(score: u32) -> i32 {
    (score as i32 - 10) / 2
}

fn fmt_mod(m: i32) -> String {
    if m >= 0 {
        format!("+{}", m)
    } else {
        m.to_string()
    }
}

fn class_label(id: &str) -> &'static str {
    match id {
        "barbarian" => "Bárbaro",
        "bard" => "Bardo",
        "cleric" => "Clérigo",
        "druid" => "Druida",
        "fighter" => "Guerrero",
        "monk" => "Monje",
        "paladin" => "Paladín",
        "ranger" => "Explorador",
        "rogue" => "Pícaro",
        "sorcerer" => "Hechicero",
        "warlock" => "Brujo",
        "wizard" => "Mago",
        _ => "Desconocido",
    }
}

fn race_label(id: &str) -> &'static str {
    match id {
        "human" => "Humano",
        "elf" => "Elfo",
        "dwarf" => "Enano",
        _ => "Desconocido",
    }
}

fn hp_color_bar(current: u32, max: u32) -> &'static str {
    if max == 0 {
        return "#57534e";
    }
    let pct = current as f32 / max as f32;
    if pct > 0.6 {
        "#10b981"
    } else if pct > 0.3 {
        "#f59e0b"
    } else {
        "#ef4444"
    }
}

fn hp_color_text(current: u32, max: u32) -> &'static str {
    if max == 0 {
        return "#57534e";
    }
    let pct = current as f32 / max as f32;
    if pct > 0.6 {
        "#10b981"
    } else if pct > 0.3 {
        "#f59e0b"
    } else {
        "#ef4444"
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PANTALLA PRINCIPAL — Master-Detail layout
// ═══════════════════════════════════════════════════════════════════════════

#[component]
pub fn Players() -> Element {
    let state = consume_context::<SharedState>().0;

    // Signal compartido: qué personaje está seleccionado
    let mut selected_id: Signal<Option<Uuid>> = use_signal(|| None);

    let mut characters_res = use_resource(move || {
        let state = state.clone();
        async move { state.persistence.all_characters().await.unwrap_or_default() }
    });

    rsx! {
        div {
            style: "display:flex; height:100vh; width:100%; overflow:hidden;
                    background:#0c0a09; color:#e7e5e4;",

            // ── SIDEBAR ───────────────────────────────────────────────────
            div {
                style: "width:272px; flex-shrink:0; display:flex; flex-direction:column;
                        border-right:1px solid #1c1917; background:#0c0a09;",

                div {
                    style: "padding:24px 20px 16px; border-bottom:1px solid #1c1917;",
                    h1 { style: "font-size:1rem; font-weight:700; color:#fef3c7;
                                 letter-spacing:0.06em; margin:0;", "JUGADORES" }
                    p { style: "font-size:0.68rem; color:#44403c; margin:4px 0 0;",
                        "Personajes de la campaña" }
                }

                div {
                    style: "flex:1; overflow-y:auto; padding:8px;",
                    match characters_res.read().as_ref() {
                        None => rsx! {
                            div { style: "display:flex; align-items:center; justify-content:center;
                                          height:120px; color:#44403c; font-size:0.75rem;",
                                "Cargando…" }
                        },
                        Some(list) if list.is_empty() => rsx! {
                            div { style: "display:flex; flex-direction:column; align-items:center;
                                          justify-content:center; height:160px; gap:12px; color:#44403c;",
                                span { style: "font-size:2.5rem; opacity:0.2;", "⚔" }
                                p { style: "font-size:0.72rem; text-align:center; margin:0;",
                                    "No hay personajes todavía." }
                            }
                        },
                        Some(list) => rsx! {
                            div { style: "display:flex; flex-direction:column; gap:3px;",
                                for ch in list.iter() {
                                    {
                                        let ch_id   = ch.id;
                                        let ch_c    = ch.clone();
                                        let is_sel  = selected_id.read().map_or(false, |s| s == ch_id);
                                        let mut sid = selected_id;
                                        rsx! {
                                            SidebarCard {
                                                character: ch_c,
                                                selected: is_sel,
                                                onclick: move |_| sid.set(Some(ch_id)),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── PANEL DERECHO ─────────────────────────────────────────────
            div {
                style: "flex:1; overflow-y:auto; background:#111110;",
                match *selected_id.read() {
                    None => rsx! { EmptyDetail {} },
                    Some(uid) => {
                        let maybe_ch = characters_res.read().as_ref()
                            .and_then(|list| list.iter().find(|c| c.id == uid).cloned());
                        match maybe_ch {
                            None => rsx! { EmptyDetail {} },
                            Some(ch) => rsx! {
                                CharacterDetail {
                                    character: ch,
                                    // Pasamos selected_id como Signal para que el efecto interno
                                    // pueda rastrearlo y resetear los campos al cambiar de personaje
                                    selected_id: selected_id,
                                    on_save: move |updated: SavedCharacter| {
                                        characters_res.restart();
                                        selected_id.set(Some(updated.id));
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

// ── Placeholder ────────────────────────────────────────────────────────────

#[component]
fn EmptyDetail() -> Element {
    rsx! {
        div {
            style: "display:flex; flex-direction:column; align-items:center; justify-content:center;
                    height:100%; gap:16px; color:#44403c; user-select:none;",
            span { style: "font-size:4rem; opacity:0.12;", "📜" }
            p { style: "font-size:0.82rem; letter-spacing:0.06em; margin:0;",
                "Selecciona un personaje para ver su ficha" }
        }
    }
}

// ── Tarjeta sidebar ────────────────────────────────────────────────────────

#[component]
fn SidebarCard(
    character: SavedCharacter,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let hp_pct = if character.max_hp > 0 {
        (character.current_hp as f32 / character.max_hp as f32 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let bar_col = hp_color_bar(character.current_hp, character.max_hp);
    let hp_col = hp_color_text(character.current_hp, character.max_hp);
    let bg = if selected { "#1c1917" } else { "transparent" };
    let border = if selected { "#b45309" } else { "transparent" };
    let name_col = if selected { "#fde68a" } else { "#d6d3d1" };

    rsx! {
        div {
            style: "background:{bg}; border:1px solid {border}; border-radius:14px;
                    padding:11px 12px; cursor:pointer; transition:background 0.12s, border-color 0.12s;",
            onclick: move |e| onclick.call(e),

            div { style: "display:flex; align-items:center; gap:10px;",
                // Avatar
                div {
                    style: "width:34px; height:34px; border-radius:50%;
                            background:#292524; border:1px solid #44403c;
                            display:flex; align-items:center; justify-content:center;
                            font-size:0.85rem; font-weight:700; color:#fbbf24;
                            flex-shrink:0; user-select:none;",
                    { character.name.chars().next().unwrap_or('?').to_string() }
                }
                div { style: "flex:1; min-width:0;",
                    p { style: "font-size:0.83rem; font-weight:600; color:{name_col};
                                overflow:hidden; text-overflow:ellipsis; white-space:nowrap;
                                margin:0; line-height:1.3;",
                        "{character.name}"
                    }
                    p { style: "font-size:0.68rem; color:#57534e; margin:1px 0 0;
                                overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                        "{race_label(&character.race_id)} · {class_label(&character.class_id)}"
                    }
                }
                div {
                    style: "flex-shrink:0; font-size:0.62rem; font-weight:700; color:#d97706;
                            background:#1c1208; border:1px solid #78350f; border-radius:7px; padding:2px 6px;",
                    "Nv{character.level}"
                }
            }

            div { style: "margin-top:9px;",
                div { style: "display:flex; justify-content:space-between;
                              font-size:0.67rem; margin-bottom:3px;",
                    span { style: "color:#44403c;", "PG" }
                    span { style: "color:{hp_col}; font-weight:500;",
                        "{character.current_hp}/{character.max_hp}" }
                }
                div { style: "height:4px; border-radius:99px; background:#1c1917; overflow:hidden;",
                    div { style: "height:100%; border-radius:99px; width:{hp_pct}%;
                                  background:{bar_col}; transition:width 0.3s;" }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PANEL DETALLE — recibe selected_id como Signal para poder rastrearlo
// ═══════════════════════════════════════════════════════════════════════════

#[component]
fn CharacterDetail(
    character: SavedCharacter,
    selected_id: Signal<Option<Uuid>>, // ← Signal rastreable por use_effect
    on_save: EventHandler<SavedCharacter>,
) -> Element {
    let state = consume_context::<SharedState>().0;
    let mut editing = use_signal(|| false);

    let mut d_name = use_signal(|| character.name.clone());
    let mut d_level = use_signal(|| character.level);
    let mut d_current_hp = use_signal(|| character.current_hp);
    let mut d_max_hp = use_signal(|| character.max_hp);
    let mut d_xp = use_signal(|| character.xp);
    let mut d_notes = use_signal(|| character.notes.clone());
    let mut d_str = use_signal(|| character.attributes.strength);
    let mut d_dex = use_signal(|| character.attributes.dexterity);
    let mut d_con = use_signal(|| character.attributes.constitution);
    let mut d_int = use_signal(|| character.attributes.intelligence);
    let mut d_wis = use_signal(|| character.attributes.wisdom);
    let mut d_cha = use_signal(|| character.attributes.charisma);

    // ── FIX: use_effect lee selected_id (Signal) para que Dioxus lo rastree.
    // Cuando cambia la selección, recargamos el personaje desde el estado y
    // actualizamos todos los signals de edición.
    let state_eff = state.clone();
    use_effect(move || {
        // Leer selected_id lo suscribe a sus cambios
        let uid = match *selected_id.read() {
            Some(u) => u,
            None => return,
        };

        let state_inner = state_eff.clone();
        spawn(async move {
            if let Ok(Some(ch)) = state_inner.persistence.get_character(uid).await {
                d_name.set(ch.name);
                d_level.set(ch.level);
                d_current_hp.set(ch.current_hp);
                d_max_hp.set(ch.max_hp);
                d_xp.set(ch.xp);
                d_notes.set(ch.notes);
                d_str.set(ch.attributes.strength);
                d_dex.set(ch.attributes.dexterity);
                d_con.set(ch.attributes.constitution);
                d_int.set(ch.attributes.intelligence);
                d_wis.set(ch.attributes.wisdom);
                d_cha.set(ch.attributes.charisma);
                editing.set(false);
            }
        });
    });

    // ── Guardar ───────────────────────────────────────────────────────────
    let char_for_save = character.clone();
    let save = move |_| {
        let mut u = char_for_save.clone();
        u.name = d_name.read().clone();
        u.level = *d_level.read();
        u.current_hp = *d_current_hp.read();
        u.max_hp = *d_max_hp.read();
        u.xp = *d_xp.read();
        u.notes = d_notes.read().clone();
        u.attributes.strength = *d_str.read();
        u.attributes.dexterity = *d_dex.read();
        u.attributes.constitution = *d_con.read();
        u.attributes.intelligence = *d_int.read();
        u.attributes.wisdom = *d_wis.read();
        u.attributes.charisma = *d_cha.read();
        u.updated_at = chrono::Utc::now().to_rfc3339();
        let s = state.clone();
        let u2 = u.clone();
        spawn(async move {
            let _ = s.persistence.upsert_character(u2).await;
        });
        on_save.call(u);
        editing.set(false);
    };

    // ── Cancelar ──────────────────────────────────────────────────────────
    let char_for_cancel = character.clone();
    let cancel = move |_| {
        d_name.set(char_for_cancel.name.clone());
        d_level.set(char_for_cancel.level);
        d_current_hp.set(char_for_cancel.current_hp);
        d_max_hp.set(char_for_cancel.max_hp);
        d_xp.set(char_for_cancel.xp);
        d_notes.set(char_for_cancel.notes.clone());
        d_str.set(char_for_cancel.attributes.strength);
        d_dex.set(char_for_cancel.attributes.dexterity);
        d_con.set(char_for_cancel.attributes.constitution);
        d_int.set(char_for_cancel.attributes.intelligence);
        d_wis.set(char_for_cancel.attributes.wisdom);
        d_cha.set(char_for_cancel.attributes.charisma);
        editing.set(false);
    };

    let is_ed = *editing.read();
    let card_border = if is_ed { "#b45309" } else { "#292524" };
    let card_glow = if is_ed { "0 0 0 1px #92400e" } else { "none" };

    let hp_pct = if *d_max_hp.read() > 0 {
        (*d_current_hp.read() as f32 / *d_max_hp.read() as f32 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let bar_col = hp_color_bar(*d_current_hp.read(), *d_max_hp.read());
    let hp_tcol = hp_color_text(*d_current_hp.read(), *d_max_hp.read());

    rsx! {
        div {
            style: "padding:28px 32px; max-width:700px; margin:0 auto;
                    display:flex; flex-direction:column; gap:18px;",

            // ── Cabecera ──────────────────────────────────────────────────
            div {
                style: "display:flex; align-items:flex-start; justify-content:space-between; gap:16px;",
                div {
                    h2 { style: "font-size:1.4rem; font-weight:700; color:#fef3c7;
                                  letter-spacing:0.02em; margin:0; line-height:1.25;",
                        { d_name.read().clone() }
                    }
                    p { style: "font-size:0.7rem; color:#57534e; margin:5px 0 0;",
                        "{race_label(&character.race_id)} · {class_label(&character.class_id)} · {character.player_name}"
                    }
                }
                div { style: "display:flex; gap:8px; flex-shrink:0; margin-top:2px;",
                    if is_ed {
                        button {
                            style: "padding:6px 14px; font-size:0.72rem; border-radius:10px;
                                    background:#1c1917; color:#78716c; border:1px solid #292524; cursor:pointer;",
                            onclick: cancel, "Cancelar"
                        }
                        button {
                            style: "padding:6px 16px; font-size:0.72rem; border-radius:10px; font-weight:600;
                                    background:#92400e; color:#fef3c7; border:1px solid #b45309; cursor:pointer;",
                            onclick: save, "💾 Guardar"
                        }
                    } else {
                        button {
                            style: "padding:6px 14px; font-size:0.72rem; border-radius:10px;
                                    background:#1c1917; color:#d6d3d1; border:1px solid #292524; cursor:pointer;",
                            onclick: move |_| editing.set(true), "✏️ Editar ficha"
                        }
                    }
                }
            }

            // ── CARD 1: Ficha de personaje ────────────────────────────────
            div {
                style: "background:#1c1917; border:1px solid {card_border}; border-radius:20px;
                        overflow:hidden; box-shadow:{card_glow}; transition:border-color 0.25s, box-shadow 0.25s;",

                // Franja dorada superior
                div { style: "height:3px; background:linear-gradient(90deg,#78350f 0%,#f59e0b 50%,#78350f 100%);" }

                div { style: "padding:22px; display:flex; flex-direction:column; gap:22px;",

                    // Identidad
                    div { style: "display:flex; align-items:flex-start; gap:16px;",
                        // Avatar
                        div {
                            style: "width:60px; height:60px; border-radius:50%;
                                    background:#292524; border:2px solid #44403c;
                                    display:flex; align-items:center; justify-content:center;
                                    font-size:1.6rem; font-weight:700; color:#fbbf24;
                                    flex-shrink:0; user-select:none;",
                            { d_name.read().chars().next().unwrap_or('?').to_string() }
                        }
                        div { style: "flex:1; padding-top:2px; display:flex; flex-direction:column; gap:5px;",
                            if is_ed {
                                input {
                                    style: "width:100%; background:#111110; border:1px solid #b45309;
                                            border-radius:10px; padding:7px 12px; font-size:0.95rem;
                                            font-weight:700; color:#fef3c7; outline:none; box-sizing:border-box;",
                                    value: "{d_name}",
                                    oninput: move |e| d_name.set(e.value()),
                                }
                            } else {
                                h1 { style: "font-size:1.15rem; font-weight:700; color:#fef3c7; margin:0;",
                                    "{d_name}" }
                            }
                            p { style: "font-size:0.72rem; color:#78716c; margin:0;",
                                "{race_label(&character.race_id)} · {class_label(&character.class_id)}" }
                            p { style: "font-size:0.68rem; color:#44403c; margin:0;",
                                "Jugador: {character.player_name}" }
                        }
                        // Badge nivel
                        div {
                            style: "flex-shrink:0; background:#1a1208; border:1px solid #78350f;
                                    border-radius:14px; padding:7px 14px; text-align:center; min-width:52px;",
                            p { style: "font-size:0.55rem; color:#92400e; margin:0;
                                        letter-spacing:0.1em; text-transform:uppercase;", "Nivel" }
                            if is_ed {
                                input {
                                    r#type: "number", min: "1", max: "20",
                                    style: "width:100%; background:transparent; border:none;
                                            text-align:center; font-size:1.4rem; font-weight:800;
                                            color:#fbbf24; outline:none; padding:0; box-sizing:border-box;",
                                    value: "{d_level}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u32>() { d_level.set(v.clamp(1,20)); }
                                    },
                                }
                            } else {
                                p { style: "font-size:1.5rem; font-weight:800; color:#fbbf24; margin:0; line-height:1.1;",
                                    "{d_level}" }
                            }
                        }
                    }

                    SectionDivider { label: "ESTADÍSTICAS" }

                    // Stats grid (3 cajas)
                    div {
                        style: "display:grid; grid-template-columns:repeat(3,1fr); gap:10px;",
                        StatBox { label: "PG Actuales", editing: is_ed,
                            display: d_current_hp.read().to_string(), accent: "#10b981",
                            children: rsx! { NumInputU32 { value: d_current_hp, min: 0, max: 99999 } } }
                        StatBox { label: "PG Máximos", editing: is_ed,
                            display: d_max_hp.read().to_string(), accent: "#78716c",
                            children: rsx! { NumInputU32 { value: d_max_hp, min: 1, max: 99999 } } }
                        StatBox { label: "Experiencia", editing: is_ed,
                            display: d_xp.read().to_string(), accent: "#f59e0b",
                            children: rsx! { NumInputU64 { value: d_xp } } }
                    }

                    // Barra HP
                    div { style: "display:flex; flex-direction:column; gap:7px;",
                        div { style: "display:flex; justify-content:space-between; font-size:0.7rem;",
                            span { style: "color:#57534e; letter-spacing:0.08em; text-transform:uppercase;",
                                "Puntos de Golpe" }
                            span { style: "color:{hp_tcol}; font-weight:600;",
                                "{d_current_hp} / {d_max_hp}" }
                        }
                        div {
                            style: "height:10px; border-radius:99px; background:#111110;
                                    overflow:hidden; border:1px solid #1c1917;",
                            div { style: "height:100%; border-radius:99px; width:{hp_pct}%;
                                          background:{bar_col}; transition:width 0.4s ease;" }
                        }
                    }

                    SectionDivider { label: "ATRIBUTOS" }

                    div {
                        style: "display:grid; grid-template-columns:repeat(6,1fr); gap:8px;",
                        AttrWidget { name: "FUE", value: d_str, editing: is_ed }
                        AttrWidget { name: "DES", value: d_dex, editing: is_ed }
                        AttrWidget { name: "CON", value: d_con, editing: is_ed }
                        AttrWidget { name: "INT", value: d_int, editing: is_ed }
                        AttrWidget { name: "SAB", value: d_wis, editing: is_ed }
                        AttrWidget { name: "CAR", value: d_cha, editing: is_ed }
                    }

                    SectionDivider { label: "NOTAS" }

                    if is_ed {
                        textarea {
                            style: "width:100%; background:#111110; border:1px solid #b45309;
                                    border-radius:12px; padding:12px 14px; font-size:0.83rem;
                                    color:#d6d3d1; outline:none; resize:none; min-height:90px;
                                    box-sizing:border-box; font-family:inherit;",
                            placeholder: "Notas sobre el personaje…",
                            value: "{d_notes}",
                            oninput: move |e| d_notes.set(e.value()),
                        }
                    } else if d_notes.read().is_empty() {
                        p { style: "font-size:0.78rem; color:#44403c; font-style:italic; margin:0;",
                            "Sin notas." }
                    } else {
                        p { style: "font-size:0.83rem; color:#a8a29e; white-space:pre-wrap;
                                    line-height:1.65; margin:0;", "{d_notes}" }
                    }
                }
            }

            // ── CARD 2: Inventario + Hechizos ────────────────────────────
            InventoryCard {
                character_id: character.id,
                initial_items: character.inventory.clone(),
                initial_currency: character.currency.clone(),
                initial_spell_slots: character.spell_slots.clone(),
                initial_known_spells: character.known_spells.clone(),
                initial_prepared_spells: character.prepared_spells.clone(),
            }
            div { style: "height:24px;" }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CARD INVENTARIO + HECHIZOS — tabs, filtro categóría, vault items, hechizos
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, PartialEq)]
enum InventoryTab {
    Items,
    Spells,
}

#[derive(Clone, PartialEq)]
enum CategoryFilter {
    All,
    Weapon,
    Armour,
    Consumable,
    Tool,
    Treasure,
    Misc,
}

impl CategoryFilter {
    fn label(&self) -> &'static str {
        match self {
            Self::All => "Todos",
            Self::Weapon => "Armas",
            Self::Armour => "Armaduras",
            Self::Consumable => "Consumibles",
            Self::Tool => "Herramientas",
            Self::Treasure => "Tesoros",
            Self::Misc => "Misc",
        }
    }
    fn matches(&self, cat: &ItemCategory) -> bool {
        match self {
            Self::All => true,
            Self::Weapon => *cat == ItemCategory::Weapon,
            Self::Armour => *cat == ItemCategory::Armour,
            Self::Consumable => *cat == ItemCategory::Consumable,
            Self::Tool => *cat == ItemCategory::Tool,
            Self::Treasure => *cat == ItemCategory::Treasure,
            Self::Misc => *cat == ItemCategory::Misc,
        }
    }
}

#[component]
fn InventoryCard(
    character_id: Uuid,
    initial_items: Vec<InventoryItem>,
    initial_currency: Currency,
    initial_spell_slots: Vec<SpellSlotLevel>,
    initial_known_spells: Vec<Spell>,
    initial_prepared_spells: Vec<Spell>,
) -> Element {
    let state = consume_context::<SharedState>().0;

    // ── Estado vivo sincronizado por polling ────────────────────────────────────
    let mut live_items: Signal<Vec<InventoryItem>> = use_signal(|| initial_items.clone());
    let mut live_currency: Signal<Currency> = use_signal(|| initial_currency.clone());
    let mut live_slots: Signal<Vec<SpellSlotLevel>> = use_signal(|| initial_spell_slots.clone());
    let mut live_known: Signal<Vec<Spell>> = use_signal(|| initial_known_spells.clone());
    let mut live_prepared: Signal<Vec<Spell>> = use_signal(|| initial_prepared_spells.clone());

    // Resetear al cambiar de personaje
    let ri = initial_items.clone();
    let rc = initial_currency.clone();
    let rs = initial_spell_slots.clone();
    let rk = initial_known_spells.clone();
    let rp = initial_prepared_spells.clone();
    use_effect(move || {
        live_items.set(ri.clone());
        live_currency.set(rc.clone());
        live_slots.set(rs.clone());
        live_known.set(rk.clone());
        live_prepared.set(rp.clone());
    });

    // Polling 4s
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
                if let Ok(sr) = s.persistence.get_spells(character_id).await {
                    live_slots.set(sr.spell_slots);
                    live_known.set(sr.known_spells);
                    live_prepared.set(sr.prepared_spells);
                }
            }
        }
    });

    // ── UI state ───────────────────────────────────────────────────────────────
    let mut active_tab: Signal<InventoryTab> = use_signal(|| InventoryTab::Items);
    let mut cat_filter: Signal<CategoryFilter> = use_signal(|| CategoryFilter::All);
    let mut editing_currency: Signal<bool> = use_signal(|| false);
    let mut show_vault_items: Signal<bool> = use_signal(|| false);
    // vault items — se cargan al abrir el panel
    let mut vault_items: Signal<Vec<serde_json::Value>> = use_signal(|| vec![]);
    // hechizos
    let mut show_add_spell: Signal<bool> = use_signal(|| false);
    let mut spell_name = use_signal(|| String::new());
    let mut spell_level: Signal<u8> = use_signal(|| 0);
    let mut spell_school = use_signal(|| String::from("unknown"));
    let mut spell_cast = use_signal(|| String::new());
    let mut spell_range = use_signal(|| String::new());
    let mut spell_dur = use_signal(|| String::new());
    let mut spell_desc = use_signal(|| String::new());
    let mut spell_dmg = use_signal(|| String::new());
    let mut spell_conc: Signal<bool> = use_signal(|| false);
    let mut spell_ritual: Signal<bool> = use_signal(|| false);
    let mut spell_prepared: Signal<bool> = use_signal(|| false);

    // Editor monedas
    let lc0 = initial_currency.clone();
    let mut e_cp = use_signal(|| lc0.copper);
    let mut e_sp = use_signal(|| lc0.silver);
    let mut e_ep = use_signal(|| lc0.electrum);
    let mut e_gp = use_signal(|| lc0.gold);
    let mut e_pp = use_signal(|| lc0.platinum);
    let lc = live_currency.read().clone();
    use_effect(move || {
        e_cp.set(lc.copper);
        e_sp.set(lc.silver);
        e_ep.set(lc.electrum);
        e_gp.set(lc.gold);
        e_pp.set(lc.platinum);
        editing_currency.set(false);
    });

    let ss = state.clone();
    let save_currency = move |_| {
        let req = UpdateCurrencyRequest {
            copper: Some(*e_cp.read()),
            silver: Some(*e_sp.read()),
            electrum: Some(*e_ep.read()),
            gold: Some(*e_gp.read()),
            platinum: Some(*e_pp.read()),
        };
        let s = ss.clone();
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

    // Pre-clones de state para los closures dentro del rsx! que lo necesitan
    // (state es Arc, no Copy — cada closure move necesita su propia copia)
    let state_vault_btn   = state.clone(); // botón "Añadir desde vault"
    let state_vault_modal = state.clone(); // modal items del vault (sv2)
    let state_spells      = state.clone(); // closures on_toggle / on_remove

    // Guardar hechizo nuevo
    let sa = state.clone();
    let save_spell = move |_| {
        use shared::api_types::spells::{AddSpellRequest, SpellComponents};
        let school_str = spell_school.read().clone();
        let school = match school_str.as_str() {
            "abjuration" => SpellSchool::Abjuration,
            "conjuration" => SpellSchool::Conjuration,
            "divination" => SpellSchool::Divination,
            "enchantment" => SpellSchool::Enchantment,
            "evocation" => SpellSchool::Evocation,
            "illusion" => SpellSchool::Illusion,
            "necromancy" => SpellSchool::Necromancy,
            "transmutation" => SpellSchool::Transmutation,
            _ => SpellSchool::Unknown,
        };
        let req = AddSpellRequest {
            name: spell_name.read().clone(),
            level: *spell_level.read(),
            school,
            casting_time: spell_cast.read().clone(),
            range: spell_range.read().clone(),
            duration: spell_dur.read().clone(),
            components: SpellComponents::default(),
            description: spell_desc.read().clone(),
            damage: if spell_dmg.read().is_empty() {
                None
            } else {
                Some(spell_dmg.read().clone())
            },
            saving_throw: None,
            notes: String::new(),
            concentration: *spell_conc.read(),
            ritual: *spell_ritual.read(),
            prepared: *spell_prepared.read(),
        };
        let is_prep = *spell_prepared.read();
        let s = sa.clone();
        spawn(async move {
            if let Ok(spell) = s.persistence.add_known_spell(character_id, req).await {
                live_known.write().push(spell.clone());
                if is_prep {
                    live_prepared.write().push(spell);
                }
            }
        });
        show_add_spell.set(false);
        spell_name.set(String::new());
        spell_level.set(0);
    };

    // Snapshot para render
    let items = live_items.read().clone();
    let currency = live_currency.read().clone();
    let slots = live_slots.read().clone();
    let known = live_known.read().clone();
    let prepared = live_prepared.read().clone();
    let cf = cat_filter.read().clone();
    let filtered: Vec<_> = items
        .iter()
        .filter(|i| cf.matches(&i.category))
        .cloned()
        .collect();
    let total_w: f32 = filtered
        .iter()
        .map(|i| i.weight.unwrap_or(0.0) * i.quantity as f32)
        .sum();
    let is_ed_cur = *editing_currency.read();
    let tab = active_tab.read().clone();

    rsx! {
        div {
            style: "background:#1c1917; border:1px solid #292524; border-radius:20px; overflow:hidden;",
            div { style: "height:3px; background:linear-gradient(90deg,#1c1917 0%,#44403c 50%,#1c1917 100%);" }
            div { style: "padding:20px; display:flex; flex-direction:column; gap:14px;",

                // ── Tabs: Inventario | Hechizos ────────────────────────────────────────
                div { style: "display:flex; gap:4px; border-bottom:1px solid #292524; padding-bottom:10px;",
                    for (lbl, tval) in [("Inventario", InventoryTab::Items), ("Hechizos", InventoryTab::Spells)] {
                        {
                            let is_active = tab == tval;
                            let tval2 = tval.clone();
                            let bg  = if is_active { "#292524" } else { "transparent" };
                            let col = if is_active { "#fef3c7" } else { "#78716c" };
                            let brd = if is_active { "#78350f" } else { "transparent" };
                            rsx! {
                                button {
                                    style: "padding:5px 16px; font-size:0.72rem; border-radius:8px;
                                            background:{bg}; color:{col}; border:1px solid {brd}; cursor:pointer;",
                                    onclick: move |_| active_tab.set(tval2.clone()),
                                    "{lbl}"
                                }
                            }
                        }
                    }
                }

                // ────────────────────────────────────────────────────────────────────────
                if tab == InventoryTab::Items {

                    // ── Monedas ──────────────────────────────────────────────────
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
                                }, "✏️ Monedas"
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
                                    onclick: cancel_currency, "Cancelar"
                                }
                                button {
                                    style: "padding:5px 14px; font-size:0.7rem; font-weight:600; border-radius:8px;
                                            cursor:pointer; background:#92400e; color:#fef3c7; border:1px solid #b45309;",
                                    onclick: save_currency, "💾 Guardar"
                                }
                            }
                        }
                    }

                    // ── Filtros de categoría ─────────────────────────────────────
                    div { style: "display:flex; gap:4px; flex-wrap:wrap;",
                        for f in [
                            CategoryFilter::All, CategoryFilter::Weapon, CategoryFilter::Armour,
                            CategoryFilter::Consumable, CategoryFilter::Tool,
                            CategoryFilter::Treasure, CategoryFilter::Misc,
                        ] {
                            {
                                let is_active = cf == f;
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

                    // ── Botón añadir desde vault ────────────────────────────────
                    div { style: "display:flex; justify-content:flex-end;",
                        button {
                            style: "padding:4px 12px; font-size:0.65rem; border-radius:8px; cursor:pointer;
                                    background:#071a0e; color:#34d399; border:1px solid #065f46;",
                            onclick: move |_| {
                                let sv = state_vault_btn.clone();
                                show_vault_items.set(true);
                                spawn(async move {
                                    // Llamar al vault manager para obtener items
                                    if let Ok(entries) = sv.vault.entries_by_kind(
                                        crate::vault::frontmatter::DndEntryType::Item
                                    ).await {
                                        vault_items.set(entries.iter().map(|e| serde_json::json!({
                                            "name": e.display_name(),
                                            "category": e.frontmatter.extra.get("category")
                                                .and_then(|v| v.as_str()).unwrap_or("misc"),
                                            "description": e.frontmatter.extra.get("description")
                                                .and_then(|v| v.as_str()).unwrap_or(""),
                                            "weight": e.frontmatter.extra.get("weight"),
                                            "notes": e.frontmatter.extra.get("notes")
                                                .and_then(|v| v.as_str()).unwrap_or(""),
                                        })).collect());
                                    }
                                });
                            },
                            "📜 Añadir desde vault"
                        }
                    }

                    // ── Tabla de items filtrados ──────────────────────────────
                    if filtered.is_empty() {
                        div {
                            style: "display:flex; align-items:center; justify-content:center;
                                    height:60px; font-size:0.78rem; color:#44403c;
                                    border:1px dashed #292524; border-radius:12px;",
                            "Sin objetos en esta categoría."
                        }
                    } else {
                        div {
                            style: "border:1px solid #292524; border-radius:14px; overflow:hidden;",
                            div {
                                style: "display:grid; grid-template-columns:5fr 3fr 2fr 2fr;
                                        padding:7px 16px; background:#111110; border-bottom:1px solid #1c1917;
                                        font-size:0.62rem; color:#57534e; text-transform:uppercase; letter-spacing:0.09em;",
                                div { "Objeto" } div { "Tipo" }
                                div { style: "text-align:center;", "Cant." }
                                div { style: "text-align:right;", "Peso" }
                            }
                            for item in filtered.iter() { ItemRow { item: item.clone() } }
                        }
                        p { style: "font-size:0.68rem; color:#44403c; text-align:right; margin:0;",
                            "{filtered.len()} objeto(s) · {total_w:.1} lb." }
                    }

                } else {
                    // ───────────────────────────────────────────────────────────────────────
                    // TAB HECHIZOS
                    // ───────────────────────────────────────────────────────────────────────

                    // ── Espacios de hechizo ───────────────────────────────────
                    SpellSlotsPanel {
                        character_id: character_id,
                        slots: slots.clone(),
                        on_update: move |new_slots| live_slots.set(new_slots),
                    }

                    // ── Hechizos preparados ────────────────────────────────
                    SectionDivider { label: "PREPARADOS" }
                    if prepared.is_empty() {
                        p { style: "font-size:0.75rem; color:#44403c; text-align:center; margin:0;",
                            "Ningún hechizo preparado." }
                    } else {
                        div { style: "display:flex; flex-direction:column; gap:4px;",
                            for spell in prepared.iter() {
                                SpellRowDm {
                                    spell: spell.clone(),
                                    is_prepared: true,
                                    character_id: character_id,
                                    on_toggle: move |_id| {},
                                    on_remove: move |_id| {},
                                }
                            }
                        }
                    }

                    // ── Hechizos conocidos ─────────────────────────────────
                    div { style: "display:flex; align-items:center; justify-content:space-between;",
                        SectionDivider { label: "CONOCIDOS" }
                        button {
                            style: "margin-left:12px; flex-shrink:0; padding:4px 12px; font-size:0.65rem;
                                    border-radius:8px; cursor:pointer; background:#071a0e; color:#34d399;
                                    border:1px solid #065f46;",
                            onclick: move |_| show_add_spell.set(true),
                            "+ Hechizo"
                        }
                    }

                    if known.is_empty() {
                        p { style: "font-size:0.75rem; color:#44403c; text-align:center; margin:0;",
                            "Sin hechizos conocidos." }
                    } else {
                        div { style: "display:flex; flex-direction:column; gap:4px;",
                            for spell in known.iter() {
                                {
                                    let is_prep = prepared.iter().any(|p| p.id == spell.id);
                                    let sid = spell.id;
                                    let sc  = state_spells.clone();
                                    let sc2 = state_spells.clone();
                                    rsx! {
                                        SpellRowDm {
                                            spell: spell.clone(),
                                            is_prepared: is_prep,
                                            character_id: character_id,
                                            on_toggle: move |_| {
                                            let s = sc.clone();
                                            spawn(async move {
                                            if let Ok(_) = s.persistence.toggle_prepared_spell(character_id, sid).await {
                                            if let Ok(sr) = s.persistence.get_spells(character_id).await {
                                            live_prepared.set(sr.prepared_spells);
                                            }
                                            }
                                            });
                                            },
                                            on_remove: move |_| {
                                                let s = sc2.clone();
                                                spawn(async move {
                                                    let _ = s.persistence.remove_known_spell(character_id, sid).await;
                                                    if let Ok(sr) = s.persistence.get_spells(character_id).await {
                                                        live_known.set(sr.known_spells);
                                                        live_prepared.set(sr.prepared_spells);
                                                    }
                                                });
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ── Modal: Añadir hechizo ────────────────────────────────────────────
        if *show_add_spell.read() {
            div {
                style: "position:fixed; inset:0; background:rgba(0,0,0,0.75);
                        display:flex; align-items:center; justify-content:center; z-index:50;",
                div {
                    style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                            padding:24px; width:480px; max-width:95vw; display:flex;
                            flex-direction:column; gap:14px;",
                    h3 { style: "font-size:1rem; font-weight:700; color:#fef3c7; margin:0;",
                        "Nuevo Hechizo" }

                    // Nombre + nivel
                    div { style: "display:grid; grid-template-columns:3fr 1fr; gap:8px;",
                        SpellInput { label: "Nombre", value: spell_name }
                        div { style: "display:flex; flex-direction:column; gap:4px;",
                            label { style: "font-size:0.62rem; color:#78716c;", "Nivel (0=truco)" }
                            input {
                                r#type: "number", min: "0", max: "9",
                                style: "background:#111110; border:1px solid #44403c; border-radius:8px;
                                        padding:7px 8px; color:#e7e5e4; font-size:0.85rem; outline:none;
                                        width:100%; box-sizing:border-box;",
                                value: "{spell_level}",
                                oninput: move |e| {
                                    if let Ok(v) = e.value().parse::<u8>() { spell_level.set(v.min(9)); }
                                }
                            }
                        }
                    }

                    // Escuela
                    div { style: "display:flex; flex-direction:column; gap:4px;",
                        label { style: "font-size:0.62rem; color:#78716c;", "Escuela" }
                        select {
                            style: "background:#111110; border:1px solid #44403c; border-radius:8px;
                                    padding:7px 8px; color:#e7e5e4; font-size:0.85rem; outline:none;",
                            oninput: move |e| spell_school.set(e.value()),
                            option { value: "abjuration",    "Abjuración" }
                            option { value: "conjuration",   "Conjuración" }
                            option { value: "divination",    "Adivinación" }
                            option { value: "enchantment",   "Encantamiento" }
                            option { value: "evocation",     "Evocación" }
                            option { value: "illusion",      "Ilusión" }
                            option { value: "necromancy",    "Nigromancia" }
                            option { value: "transmutation", "Transmutación" }
                            option { value: "unknown",       selected: true, "Desconocida" }
                        }
                    }

                    // Tiempo / alcance / duración
                    div { style: "display:grid; grid-template-columns:repeat(3,1fr); gap:8px;",
                        SpellInput { label: "Tiempo lanzamiento", value: spell_cast }
                        SpellInput { label: "Alcance",           value: spell_range }
                        SpellInput { label: "Duración",          value: spell_dur }
                    }

                    // Daño
                    SpellInput { label: "Daño (ej: 8d6 fuego)", value: spell_dmg }

                    // Descripción
                    div { style: "display:flex; flex-direction:column; gap:4px;",
                        label { style: "font-size:0.62rem; color:#78716c;", "Descripción" }
                        textarea {
                            style: "background:#111110; border:1px solid #44403c; border-radius:8px;
                                    padding:8px 10px; color:#e7e5e4; font-size:0.82rem; outline:none;
                                    resize:none; min-height:72px; font-family:inherit;",
                            value: "{spell_desc}",
                            oninput: move |e| spell_desc.set(e.value())
                        }
                    }

                    // Flags
                    div { style: "display:flex; gap:16px;",
                        CheckFlag { label: "Concentración", value: spell_conc }
                        CheckFlag { label: "Ritual",         value: spell_ritual }
                        CheckFlag { label: "Preparado",      value: spell_prepared }
                    }

                    // Acciones
                    div { style: "display:flex; justify-content:flex-end; gap:8px;",
                        button {
                            style: "padding:7px 16px; font-size:0.72rem; border-radius:8px; cursor:pointer;
                                    background:#1c1917; color:#78716c; border:1px solid #292524;",
                            onclick: move |_| show_add_spell.set(false), "Cancelar"
                        }
                        button {
                            style: "padding:7px 16px; font-size:0.72rem; font-weight:600; border-radius:8px;
                                    cursor:pointer; background:#1e3a5f; color:#93c5fd; border:1px solid #1d4ed8;",
                            onclick: save_spell, "✨ Añadir hechizo"
                        }
                    }
                }
            }
        }

        // ── Modal: Items del vault ─────────────────────────────────────────
        if *show_vault_items.read() {
            {
                let vi = vault_items.read().clone();
                let sv2 = state_vault_modal.clone();
                rsx! {
                    div {
                        style: "position:fixed; inset:0; background:rgba(0,0,0,0.75);
                                display:flex; align-items:center; justify-content:center; z-index:50;",
                        div {
                            style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                                    padding:24px; width:560px; max-width:95vw; max-height:80vh;
                                    display:flex; flex-direction:column; gap:14px;",
                            div { style: "display:flex; justify-content:space-between; align-items:center;",
                                h3 { style: "font-size:1rem; font-weight:700; color:#fef3c7; margin:0;",
                                    "📜 Items del Vault" }
                                button {
                                    style: "padding:4px 12px; font-size:0.7rem; border-radius:8px; cursor:pointer;
                                            background:#1c1917; color:#78716c; border:1px solid #292524;",
                                    onclick: move |_| show_vault_items.set(false), "Cerrar"
                                }
                            }
                            if vi.is_empty() {
                                p { style: "color:#44403c; font-size:0.8rem; text-align:center;",
                                    "No hay items con dnd_type:item en el vault, o el vault no está configurado." }
                            } else {
                                div { style: "overflow-y:auto; display:flex; flex-direction:column; gap:6px;",
                                    for vi_item in vi.iter() {
                                        {
                                            let name = vi_item.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                            let desc = vi_item.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                            let cat_str = vi_item.get("category").and_then(|v| v.as_str()).unwrap_or("misc").to_string();
                                            let notes = vi_item.get("notes").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                            let name2 = name.clone();
                                            let cat2  = cat_str.clone();
                                            let desc2 = desc.clone();
                                            let notes2 = notes.clone();
                                            let s3 = sv2.clone();
                                            rsx! {
                                                div {
                                                    style: "display:flex; align-items:center; justify-content:space-between;
                                                            background:#111110; border:1px solid #292524; border-radius:10px;
                                                            padding:10px 14px; gap:12px;",
                                                    div { style: "flex:1; min-width:0;",
                                                        p { style: "font-size:0.83rem; font-weight:600; color:#d6d3d1; margin:0;",
                                                            "{name}" }
                                                        if !desc.is_empty() {
                                                            p { style: "font-size:0.7rem; color:#57534e; margin:2px 0 0;
                                                                        overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                                                                "{desc}" }
                                                        }
                                                    }
                                                    button {
                                                        style: "flex-shrink:0; padding:4px 10px; font-size:0.65rem; border-radius:7px;
                                                                cursor:pointer; background:#1a2e0a; color:#86efac; border:1px solid #166534;",
                                                        onclick: move |_| {
                                                            use shared::api_types::inventory::{AddItemRequest, ItemCategory};
                                                            let cat = match cat2.as_str() {
                                                                "weapon"     => ItemCategory::Weapon,
                                                                "armour"     => ItemCategory::Armour,
                                                                "consumable" => ItemCategory::Consumable,
                                                                "tool"       => ItemCategory::Tool,
                                                                "treasure"   => ItemCategory::Treasure,
                                                                _            => ItemCategory::Misc,
                                                            };
                                                            let item = shared::api_types::inventory::InventoryItem::new(
                                                                name2.clone(), cat, desc2.clone(), 1);
                                                            let item = shared::api_types::inventory::InventoryItem {
                                                                notes: notes2.clone(), ..item
                                                            };
                                                            let s = s3.clone();
                                                            spawn(async move {
                                                                if let Ok(_) = s.persistence.add_item(character_id, item).await {
                                                                    if let Ok((items, _)) = s.persistence.get_inventory(character_id).await {
                                                                        live_items.set(items);
                                                                    }
                                                                }
                                                            });
                                                            show_vault_items.set(false);
                                                        },
                                                        "+ Al inventario"
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
        }
    }
}

// ── Input numérico de moneda para el editor del DM ──────────────────────────────

#[component]
fn CurrencyInput(label: &'static str, accent: &'static str, mut value: Signal<u32>) -> Element {
    rsx! {
        div {
            style: "display:flex; flex-direction:column; align-items:center; gap:5px;",
            span { style: "font-size:0.65rem; font-weight:700; color:{accent};", "{label}" }
            input {
                r#type: "number", min: "0",
                style: "width:100%; background:#1c1917; border:1px solid #44403c;
                        border-radius:8px; padding:5px 4px; text-align:center;
                        font-size:0.95rem; font-weight:700; color:#e7e5e4;
                        outline:none; box-sizing:border-box;",
                value: "{value}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<u32>() { value.set(v); }
                },
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Widgets
// ═══════════════════════════════════════════════════════════════════════════

#[component]
fn SectionDivider(label: &'static str) -> Element {
    rsx! {
        div { style: "display:flex; align-items:center; gap:10px;",
            div { style: "flex:1; height:1px; background:#292524;" }
            span { style: "font-size:0.6rem; color:#57534e; letter-spacing:0.14em; user-select:none;",
                "✦  {label}  ✦" }
            div { style: "flex:1; height:1px; background:#292524;" }
        }
    }
}

#[component]
fn StatBox(
    label: &'static str,
    editing: bool,
    display: String,
    accent: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div {
            style: "background:#111110; border:1px solid #292524; border-radius:14px;
                    padding:13px 10px; display:flex; flex-direction:column; align-items:center; gap:7px;",
            p { style: "font-size:0.6rem; color:#57534e; text-transform:uppercase;
                         letter-spacing:0.09em; text-align:center; margin:0;", "{label}" }
            if editing { { children } } else {
                p { style: "font-size:1.7rem; font-weight:800; color:{accent}; margin:0; line-height:1;",
                    "{display}" }
            }
        }
    }
}

#[component]
fn NumInputU32(mut value: Signal<u32>, min: u32, max: u32) -> Element {
    rsx! {
        input {
            r#type: "number", min: "{min}", max: "{max}",
            style: "width:100%; background:#1c1917; border:1px solid #b45309; border-radius:8px;
                    padding:6px 4px; text-align:center; font-size:1.1rem; font-weight:700;
                    color:#fef3c7; outline:none; box-sizing:border-box;",
            value: "{value}",
            oninput: move |e| {
                if let Ok(v) = e.value().parse::<u32>() { value.set(v.clamp(min, max)); }
            },
        }
    }
}

#[component]
fn NumInputU64(mut value: Signal<u64>) -> Element {
    rsx! {
        input {
            r#type: "number", min: "0",
            style: "width:100%; background:#1c1917; border:1px solid #b45309; border-radius:8px;
                    padding:6px 4px; text-align:center; font-size:1.1rem; font-weight:700;
                    color:#fef3c7; outline:none; box-sizing:border-box;",
            value: "{value}",
            oninput: move |e| {
                if let Ok(v) = e.value().parse::<u64>() { value.set(v); }
            },
        }
    }
}

#[component]
fn AttrWidget(name: &'static str, mut value: Signal<u32>, editing: bool) -> Element {
    let m = modifier(*value.read());
    let ms = fmt_mod(m);
    let mod_color = if m >= 0 { "#10b981" } else { "#ef4444" };
    let border = if editing { "#b45309" } else { "#292524" };

    rsx! {
        div {
            style: "background:#111110; border:1px solid {border}; border-radius:14px;
                    display:flex; flex-direction:column; align-items:center;
                    padding:12px 4px 10px; gap:6px; transition:border-color 0.2s;",
            p { style: "font-size:0.58rem; color:#57534e; text-transform:uppercase;
                         letter-spacing:0.1em; font-weight:600; margin:0;", "{name}" }
            if editing {
                input {
                    r#type: "number", min: "1", max: "30",
                    style: "width:100%; background:#1c1917; border:1px solid #b45309; border-radius:8px;
                            padding:4px 2px; text-align:center; font-size:1.1rem; font-weight:700;
                            color:#fef3c7; outline:none; box-sizing:border-box;",
                    value: "{value}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u32>() { value.set(v.clamp(1, 30)); }
                    },
                }
            } else {
                div {
                    style: "width:42px; height:42px; border-radius:50%;
                            background:#1c1917; border:2px solid #44403c;
                            display:flex; align-items:center; justify-content:center;",
                    span { style: "font-size:1.15rem; font-weight:800; color:#e7e5e4;", "{value}" }
                }
            }
            div {
                style: "width:26px; height:26px; border-radius:50%;
                        background:#0c0a09; border:1px solid #1c1917;
                        display:flex; align-items:center; justify-content:center;",
                span { style: "font-size:0.62rem; font-weight:700; color:{mod_color};", "{ms}" }
            }
        }
    }
}

#[component]
fn CurrencyRow(currency: Currency) -> Element {
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
fn ItemRow(item: InventoryItem) -> Element {
    let total_w = item.weight.unwrap_or(0.0) * item.quantity as f32;
    let category = item.category.label();
    rsx! {
        div {
            style: "display:grid; grid-template-columns:5fr 3fr 2fr 2fr;
                    padding:9px 16px; border-bottom:1px solid #1c1917; align-items:center;",
            div {
                style: "font-size:0.83rem; font-weight:500; color:#d6d3d1;
                        overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                "{item.name}"
            }
            div {
                span {
                    style: "font-size:0.63rem; background:#1c1917; color:#78716c;
                            border-radius:6px; padding:3px 7px; border:1px solid #292524;",
                    "{category}"
                }
            }
            div { style: "font-size:0.78rem; color:#78716c; text-align:center;", "×{item.quantity}" }
            div { style: "font-size:0.72rem; color:#57534e; text-align:right;", "{total_w:.1} lb" }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HECHIZOS — Widgets
// ═══════════════════════════════════════════════════════════════════════════

/// Panel de espacios de hechizo — muestra Nv1–9 con contador editable.
#[component]
fn SpellSlotsPanel(
    character_id: Uuid,
    slots: Vec<SpellSlotLevel>,
    on_update: EventHandler<Vec<SpellSlotLevel>>,
) -> Element {
    let state = consume_context::<SharedState>().0;
    // Trabajamos sobre una copia editable
    let mut local: Signal<Vec<SpellSlotLevel>> = use_signal(|| slots.clone());
    use_effect(move || {
        local.set(slots.clone());
    });

    let save_slots = {
        let s = state.clone();
        move |_| {
            use shared::api_types::spells::UpdateSpellSlotsRequest;
            let req = UpdateSpellSlotsRequest {
                slots: local.read().clone(),
            };
            let s2 = s.clone();
            let slots_snap = local.read().clone();
            spawn(async move {
                let _ = s2.persistence.update_spell_slots(character_id, req).await;
            });
            on_update.call(slots_snap);
        }
    };

    rsx! {
        div {
            style: "background:#111110; border:1px solid #292524; border-radius:14px; padding:14px 16px;",
            div { style: "display:flex; align-items:center; justify-content:space-between; margin-bottom:10px;",
                span { style: "font-size:0.65rem; color:#78716c; letter-spacing:0.1em;", "ESPACIOS DE HECHIZO" }
                button {
                    style: "padding:3px 10px; font-size:0.62rem; border-radius:6px; cursor:pointer;
                            background:#1a1208; color:#f59e0b; border:1px solid #78350f;",
                    onclick: save_slots, "💾 Guardar espacios"
                }
            }
            // Grid de niveles 1–9
            div { style: "display:grid; grid-template-columns:repeat(9,1fr); gap:6px;",
                for lvl_idx in 0..9u8 {
                    {
                        let lvl_num = lvl_idx + 1;
                        // Buscamos el nivel o creamos uno vacío
                        let slot = local.read().iter()
                            .find(|s| s.level == lvl_num)
                            .cloned()
                            .unwrap_or(SpellSlotLevel { level: lvl_num, total: 0, remaining: 0 });
                        rsx! {
                            div {
                                style: "display:flex; flex-direction:column; align-items:center; gap:4px;",
                                span { style: "font-size:0.58rem; color:#57534e;", "Nv{lvl_num}" }
                                // Total
                                input {
                                    r#type: "number", min: "0", max: "9",
                                    style: "width:100%; background:#1c1917; border:1px solid #44403c;
                                            border-radius:6px; padding:4px 2px; text-align:center;
                                            font-size:0.85rem; color:#e7e5e4; outline:none;
                                            box-sizing:border-box;",
                                    title: "Total",
                                    value: "{slot.total}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u8>() {
                                            let mut s = local.write();
                                            if let Some(existing) = s.iter_mut().find(|x| x.level == lvl_num) {
                                                existing.total = v;
                                                if existing.remaining > v { existing.remaining = v; }
                                            } else {
                                                s.push(SpellSlotLevel { level: lvl_num, total: v, remaining: v.min(v) });
                                            }
                                        }
                                    }
                                }
                                // Restantes
                                input {
                                    r#type: "number", min: "0", max: "{slot.total}",
                                    style: "width:100%; background:#1c1917; border:1px solid #065f46;
                                            border-radius:6px; padding:4px 2px; text-align:center;
                                            font-size:0.85rem; color:#34d399; outline:none;
                                            box-sizing:border-box;",
                                    title: "Restantes",
                                    value: "{slot.remaining}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u8>() {
                                            let mut s = local.write();
                                            if let Some(existing) = s.iter_mut().find(|x| x.level == lvl_num) {
                                                existing.remaining = v.min(existing.total);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            p { style: "font-size:0.6rem; color:#44403c; margin:8px 0 0;",
                "Fila superior: total · Fila inferior: restantes (verde)" }
        }
    }
}

/// Fila de un hechizo en el panel DM (con toggle preparado + eliminar).
#[component]
fn SpellRowDm(
    spell: Spell,
    is_prepared: bool,
    character_id: Uuid,
    on_toggle: EventHandler<Uuid>,
    on_remove: EventHandler<Uuid>,
) -> Element {
    let mut expanded = use_signal(|| false);
    let level_str = if spell.level == 0 {
        "Truco".to_string()
    } else {
        format!("Nv{}", spell.level)
    };
    let school_str = spell.school.label();
    let prep_bg = if is_prepared { "#1a2e0a" } else { "#111110" };
    let prep_col = if is_prepared { "#86efac" } else { "#57534e" };
    let prep_brd = if is_prepared { "#166534" } else { "#292524" };
    let conc_badge = if spell.concentration { " ◇C" } else { "" };
    let rit_badge = if spell.ritual { " ◆R" } else { "" };
    let sid = spell.id;

    rsx! {
        div {
            style: "background:#111110; border:1px solid #292524; border-radius:10px; overflow:hidden;",

            // ─ Cabecera ─────────────────────────────────────────────────────
            div {
                style: "display:flex; align-items:center; gap:8px; padding:9px 12px; cursor:pointer;",
                onclick: move |_| { let v = *expanded.read(); expanded.set(!v); },

                // Badge nivel
                span {
                    style: "flex-shrink:0; font-size:0.6rem; font-weight:700; color:#a78bfa;
                            background:#1e1030; border:1px solid #4c1d95; border-radius:6px;
                            padding:2px 6px;",
                    "{level_str}"
                }
                // Nombre
                span {
                    style: "flex:1; font-size:0.83rem; font-weight:500; color:#d6d3d1;
                            overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                    "{spell.name}{conc_badge}{rit_badge}"
                }
                // Escuela
                span { style: "font-size:0.62rem; color:#78716c; flex-shrink:0;", "{school_str}" }

                // Toggle preparado
                button {
                    style: "flex-shrink:0; padding:2px 8px; font-size:0.6rem; border-radius:6px;
                            background:{prep_bg}; color:{prep_col}; border:1px solid {prep_brd};
                            cursor:pointer;",
                    onclick: move |e| { e.stop_propagation(); on_toggle.call(sid); },
                    if is_prepared { "✔ Prep" } else { "Preparar" }
                }
                // Eliminar
                button {
                    style: "flex-shrink:0; padding:2px 7px; font-size:0.6rem; border-radius:6px;
                            background:#200a0a; color:#f87171; border:1px solid #7f1d1d;
                            cursor:pointer;",
                    onclick: move |e| { e.stop_propagation(); on_remove.call(sid); },
                    "×"
                }
                span { style: "font-size:0.65rem; color:#57534e;",
                    if *expanded.read() { "▴" } else { "▾" }
                }
            }

            // ─ Detalle expandible ─────────────────────────────────────────
            if *expanded.read() {
                div {
                    style: "padding:10px 14px; border-top:1px solid #1c1917;
                            display:flex; flex-direction:column; gap:7px;",

                    // Meta
                    div { style: "display:flex; flex-wrap:wrap; gap:10px;",
                        if !spell.casting_time.is_empty() {
                            SpellMeta { label: "⏱ Tiempo", value: spell.casting_time.clone() }
                        }
                        if !spell.range.is_empty() {
                            SpellMeta { label: "🎯 Alcance", value: spell.range.clone() }
                        }
                        if !spell.duration.is_empty() {
                            SpellMeta { label: "⌛ Duración", value: spell.duration.clone() }
                        }
                        if let Some(dmg) = &spell.damage {
                            SpellMeta { label: "⚔️ Daño", value: dmg.clone() }
                        }
                        if let Some(sv) = &spell.saving_throw {
                            SpellMeta { label: "🛡 Salvación", value: sv.clone() }
                        }
                    }

                    if !spell.description.is_empty() {
                        p { style: "font-size:0.78rem; color:#a8a29e; line-height:1.55;
                                    white-space:pre-wrap; margin:0;",
                            "{spell.description}" }
                    }
                    if !spell.notes.is_empty() {
                        p { style: "font-size:0.72rem; color:#78716c; font-style:italic; margin:0;",
                            "Notas: {spell.notes}" }
                    }
                }
            }
        }
    }
}

#[component]
fn SpellMeta(label: &'static str, value: String) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:1px;",
            span { style: "font-size:0.58rem; color:#57534e;", "{label}" }
            span { style: "font-size:0.75rem; color:#d6d3d1; font-weight:500;", "{value}" }
        }
    }
}

#[component]
fn SpellInput(label: &'static str, mut value: Signal<String>) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:4px;",
            label { style: "font-size:0.62rem; color:#78716c;", "{label}" }
            input {
                style: "background:#111110; border:1px solid #44403c; border-radius:8px;
                        padding:7px 8px; color:#e7e5e4; font-size:0.85rem; outline:none;
                        width:100%; box-sizing:border-box;",
                value: "{value}",
                oninput: move |e| value.set(e.value())
            }
        }
    }
}

#[component]
fn CheckFlag(label: &'static str, mut value: Signal<bool>) -> Element {
    let checked = *value.read();
    let col = if checked { "#fbbf24" } else { "#57534e" };
    rsx! {
        button {
            style: "display:flex; align-items:center; gap:6px; background:none; border:none;
                    cursor:pointer; color:{col}; font-size:0.75rem; padding:0;",
            onclick: move |_| { let v = *value.read(); value.set(!v); },
            {
            let check_bg = if checked { "#92400e" } else { "transparent" };
            rsx! {
            span { style: "width:14px; height:14px; border-radius:4px; border:1px solid {col};
                           background:{check_bg};
                           display:flex; align-items:center; justify-content:center;",
                if checked { span { style: "font-size:0.6rem; color:#fef3c7;", "✓" } }
            }
            } // rsx!
            } // let check_bg
            "{label}"
        }
    }
}
