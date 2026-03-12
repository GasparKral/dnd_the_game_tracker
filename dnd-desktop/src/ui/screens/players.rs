// ═══════════════════════════════════════════════════════════════════════════
// players.rs — Pantalla de personajes del DM
//
// Arquitectura tras refactorización:
//  • Players          — layout master-detail con sidebar
//  • CharacterDetail  — ficha editable (stats, atributos, notas)
//  • InventoryCard    — wrapper que une InventoryPanel + SpellsPanel con tabs
//
// Los módulos player_inventory.rs y player_spells.rs contienen toda la
// lógica de inventario y hechizos respectivamente. Cada panel recibe
// character_id y sus datos iniciales — el polling es INTERNO a cada panel
// y usa ese character_id para no mezclar datos entre personajes.
// ═══════════════════════════════════════════════════════════════════════════

use crate::states::SharedState;
use crate::ui::screens::player_inventory::InventoryPanel;
use crate::ui::screens::player_spells::{SectionDivider, SpellsPanel};
use chrono;
use dioxus::prelude::*;
use shared::persistence::SavedCharacter;
use uuid::Uuid;

// ─── Helpers ────────────────────────────────────────────────────────────────

fn modifier(score: u32) -> i32 { (score as i32 - 10) / 2 }

fn fmt_mod(m: i32) -> String {
    if m >= 0 { format!("+{}", m) } else { m.to_string() }
}

fn class_label(id: &str) -> &'static str {
    match id {
        "barbarian" => "Bárbaro",
        "bard"      => "Bardo",
        "cleric"    => "Clérigo",
        "druid"     => "Druida",
        "fighter"   => "Guerrero",
        "monk"      => "Monje",
        "paladin"   => "Paladín",
        "ranger"    => "Explorador",
        "rogue"     => "Pícaro",
        "sorcerer"  => "Hechicero",
        "warlock"   => "Brujo",
        "wizard"    => "Mago",
        _           => "Desconocido",
    }
}

fn race_label(id: &str) -> &'static str {
    match id {
        "human" => "Humano",
        "elf"   => "Elfo",
        "dwarf" => "Enano",
        _       => "Desconocido",
    }
}

/// Bono de competencia según nivel (PHB 2024)
fn proficiency_bonus(level: u32) -> i32 {
    match level {
        1..=4  => 2,
        5..=8  => 3,
        9..=12 => 4,
        13..=16 => 5,
        _ => 6,
    }
}

/// Tabla de habilidades: (id, nombre en español, atributo base)
/// Atributo: 0=FUE, 1=DES, 2=CON, 3=INT, 4=SAB, 5=CAR
fn skill_table() -> &'static [(&'static str, &'static str, usize)] {
    &[
        ("athletics",       "Atletismo",      0), // FUE
        ("acrobatics",      "Acrobacias",     1), // DES
        ("sleight_of_hand", "Juego de Manos", 1), // DES
        ("stealth",         "Sigilo",         1), // DES
        ("arcana",          "Arcanos",        3), // INT
        ("history",         "Historia",       3), // INT
        ("investigation",   "Investigación",  3), // INT
        ("nature",          "Naturaleza",     3), // INT
        ("religion",        "Religión",       3), // INT
        ("animal_handling", "Trato Animales", 4), // SAB
        ("insight",         "Perspicacia",    4), // SAB
        ("medicine",        "Medicina",       4), // SAB
        ("perception",      "Percepción",     4), // SAB
        ("survival",        "Supervivencia",  4), // SAB
        ("deception",       "Engaño",         5), // CAR
        ("intimidation",    "Intimidación",   5), // CAR
        ("performance",     "Interpretación", 5), // CAR
        ("persuasion",      "Persuasión",     5), // CAR
    ]
}

fn hp_bar_color(current: u32, max: u32) -> &'static str {
    if max == 0 { return "#57534e"; }
    match (current as f32 / max as f32 * 10.0) as u32 {
        7..=10 => "#10b981",
        4..=6  => "#f59e0b",
        _      => "#ef4444",
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PANTALLA PRINCIPAL
// ═══════════════════════════════════════════════════════════════════════════

#[component]
pub fn Players() -> Element {
    let state = consume_context::<SharedState>().0;
    let mut selected_id: Signal<Option<Uuid>> = use_signal(|| None);

    let mut characters_res = use_resource(move || {
        let s = state.clone();
        async move { s.persistence.all_characters().await.unwrap_or_default() }
    });

    rsx! {
        div {
            style: "display:flex; height:100vh; width:100%; overflow:hidden;
                    background:#0c0a09; color:#e7e5e4;",

            // ── Sidebar ───────────────────────────────────────────────────
            div {
                style: "width:272px; flex-shrink:0; display:flex; flex-direction:column;
                        border-right:1px solid #1c1917; background:#0c0a09;",

                div { style: "padding:24px 20px 16px; border-bottom:1px solid #1c1917;",
                    h1 { style: "font-size:1rem; font-weight:700; color:#fef3c7;
                                 letter-spacing:0.06em; margin:0;", "JUGADORES" }
                    p { style: "font-size:0.68rem; color:#44403c; margin:4px 0 0;",
                        "Personajes de la campaña" }
                }

                div { style: "flex:1; overflow-y:auto; padding:8px;",
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
                                        let ch_id  = ch.id;
                                        let ch_c   = ch.clone();
                                        let is_sel = selected_id.read().map_or(false, |s| s == ch_id);
                                        rsx! {
                                            SidebarCard {
                                                character: ch_c,
                                                selected: is_sel,
                                                onclick: move |_| selected_id.set(Some(ch_id)),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Panel derecho ─────────────────────────────────────────────
            div { style: "flex:1; overflow-y:auto; background:#111110;",
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
                                    selected_id,
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

// ─── Placeholder ─────────────────────────────────────────────────────────────

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

// ─── Tarjeta sidebar ─────────────────────────────────────────────────────────

#[component]
fn SidebarCard(
    character: SavedCharacter,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let hp_pct = if character.max_hp > 0 {
        (character.current_hp as f32 / character.max_hp as f32 * 100.0).clamp(0.0, 100.0)
    } else { 0.0 };
    let bar_col = hp_bar_color(character.current_hp, character.max_hp);
    let bg      = if selected { "#1c1917" } else { "transparent" };
    let border  = if selected { "#b45309" } else { "transparent" };
    let name_col = if selected { "#fde68a" } else { "#d6d3d1" };

    rsx! {
        div {
            style: "background:{bg}; border:1px solid {border}; border-radius:14px;
                    padding:11px 12px; cursor:pointer;",
            onclick: move |e| onclick.call(e),

            div { style: "display:flex; align-items:center; gap:10px;",
                div {
                    style: "width:34px; height:34px; border-radius:50%;
                            background:#292524; border:1px solid #44403c;
                            display:flex; align-items:center; justify-content:center;
                            font-size:0.85rem; font-weight:700; color:#fbbf24; flex-shrink:0;",
                    { character.name.chars().next().unwrap_or('?').to_string() }
                }
                div { style: "flex:1; min-width:0;",
                    p { style: "font-size:0.83rem; font-weight:600; color:{name_col};
                                overflow:hidden; text-overflow:ellipsis; white-space:nowrap; margin:0;",
                        "{character.name}" }
                    p { style: "font-size:0.68rem; color:#57534e; margin:1px 0 0;
                                overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                        "{race_label(&character.race_id)} · {class_label(&character.class_id)}" }
                }
                div {
                    style: "flex-shrink:0; font-size:0.62rem; font-weight:700; color:#d97706;
                            background:#1c1208; border:1px solid #78350f; border-radius:7px; padding:2px 6px;",
                    "Nv{character.level}"
                }
            }
            div { style: "margin-top:9px;",
                div { style: "display:flex; justify-content:space-between; font-size:0.67rem; margin-bottom:3px;",
                    span { style: "color:#44403c;", "PG" }
                    span { style: "color:{bar_col}; font-weight:500;",
                        "{character.current_hp}/{character.max_hp}" }
                }
                div { style: "height:4px; border-radius:99px; background:#1c1917; overflow:hidden;",
                    div { style: "height:100%; border-radius:99px; width:{hp_pct}%;
                                  background:{bar_col};" }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PANEL DETALLE
// ═══════════════════════════════════════════════════════════════════════════

#[component]
fn CharacterDetail(
    character: SavedCharacter,
    selected_id: Signal<Option<Uuid>>,
    on_save: EventHandler<SavedCharacter>,
) -> Element {
    let state = consume_context::<SharedState>().0;
    let mut editing = use_signal(|| false);

    // Señales de edición
    let mut d_name       = use_signal(|| character.name.clone());
    let mut d_level      = use_signal(|| character.level);
    let mut d_current_hp = use_signal(|| character.current_hp);
    let mut d_max_hp     = use_signal(|| character.max_hp);
    let mut d_xp         = use_signal(|| character.xp);
    let mut d_notes      = use_signal(|| character.notes.clone());
    let mut d_temp_hp    = use_signal(|| character.temp_hp);
    let mut d_str        = use_signal(|| character.attributes.strength);
    let mut d_dex        = use_signal(|| character.attributes.dexterity);
    let mut d_con        = use_signal(|| character.attributes.constitution);
    let mut d_int        = use_signal(|| character.attributes.intelligence);
    let mut d_wis        = use_signal(|| character.attributes.wisdom);
    let mut d_cha        = use_signal(|| character.attributes.charisma);

    // Resetear señales cuando cambia el personaje seleccionado
    {
        let se = state.clone();
        use_effect(move || {
            let uid = match *selected_id.read() {
                Some(u) => u,
                None => return,
            };
            let s = se.clone();
            spawn(async move {
                if let Ok(Some(ch)) = s.persistence.get_character(uid).await {
                    d_name.set(ch.name);
                    d_level.set(ch.level);
                    d_current_hp.set(ch.current_hp);
                    d_max_hp.set(ch.max_hp);
                    d_temp_hp.set(ch.temp_hp);
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
    }

    // Guardar
    let char_save = character.clone();
    let ss = state.clone();
    let save = move |_| {
        let mut u = char_save.clone();
        u.name                    = d_name.read().clone();
        u.level                   = *d_level.read();
        u.current_hp              = *d_current_hp.read();
        u.max_hp                  = *d_max_hp.read();
        u.temp_hp                 = *d_temp_hp.read();
        u.xp                      = *d_xp.read();
        u.notes                   = d_notes.read().clone();
        u.attributes.strength     = *d_str.read();
        u.attributes.dexterity    = *d_dex.read();
        u.attributes.constitution = *d_con.read();
        u.attributes.intelligence = *d_int.read();
        u.attributes.wisdom       = *d_wis.read();
        u.attributes.charisma     = *d_cha.read();
        u.updated_at              = chrono::Utc::now().to_rfc3339();
        let s = ss.clone();
        let u2 = u.clone();
        spawn(async move { let _ = s.persistence.upsert_character(u2).await; });
        on_save.call(u);
        editing.set(false);
    };

    // Cancelar
    let char_cancel = character.clone();
    let cancel = move |_| {
        d_name.set(char_cancel.name.clone());
        d_level.set(char_cancel.level);
        d_current_hp.set(char_cancel.current_hp);
        d_max_hp.set(char_cancel.max_hp);
        d_temp_hp.set(char_cancel.temp_hp);
        d_xp.set(char_cancel.xp);
        d_notes.set(char_cancel.notes.clone());
        d_str.set(char_cancel.attributes.strength);
        d_dex.set(char_cancel.attributes.dexterity);
        d_con.set(char_cancel.attributes.constitution);
        d_int.set(char_cancel.attributes.intelligence);
        d_wis.set(char_cancel.attributes.wisdom);
        d_cha.set(char_cancel.attributes.charisma);
        editing.set(false);
    };

    let is_ed = *editing.read();
    let hp_pct = if *d_max_hp.read() > 0 {
        (*d_current_hp.read() as f32 / *d_max_hp.read() as f32 * 100.0).clamp(0.0, 100.0)
    } else { 0.0 };
    let bar_col  = hp_bar_color(*d_current_hp.read(), *d_max_hp.read());
    let card_brd = if is_ed { "#b45309" } else { "#292524" };
    let card_glow = if is_ed { "0 0 0 1px #92400e" } else { "none" };

    rsx! {
        div {
            style: "padding:28px 32px; max-width:700px; margin:0 auto;
                    display:flex; flex-direction:column; gap:18px;",

            // ── Cabecera ──────────────────────────────────────────────────
            div { style: "display:flex; align-items:flex-start; justify-content:space-between; gap:16px;",
                div {
                    h2 { style: "font-size:1.4rem; font-weight:700; color:#fef3c7;
                                  letter-spacing:0.02em; margin:0;",
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

            // ── Card: Ficha principal ─────────────────────────────────────
            div {
                style: "background:#1c1917; border:1px solid {card_brd}; border-radius:20px;
                        overflow:hidden; box-shadow:{card_glow};",
                div { style: "height:3px; background:linear-gradient(90deg,#78350f 0%,#f59e0b 50%,#78350f 100%);" }
                div { style: "padding:22px; display:flex; flex-direction:column; gap:22px;",

                    // Identidad
                    div { style: "display:flex; align-items:flex-start; gap:16px;",
                        div {
                            style: "width:60px; height:60px; border-radius:50%;
                                    background:#292524; border:2px solid #44403c;
                                    display:flex; align-items:center; justify-content:center;
                                    font-size:1.6rem; font-weight:700; color:#fbbf24; flex-shrink:0;",
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
                                    border-radius:14px; padding:7px 14px; text-align:center;",
                            p { style: "font-size:0.55rem; color:#92400e; margin:0; letter-spacing:0.1em;", "NIVEL" }
                            if is_ed {
                                input {
                                    r#type: "number", min: "1", max: "20",
                                    style: "width:48px; background:transparent; border:none; text-align:center;
                                            font-size:1.4rem; font-weight:800; color:#fbbf24; outline:none; padding:0;",
                                    value: "{d_level}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u32>() { d_level.set(v.clamp(1,20)); }
                                    },
                                }
                            } else {
                                p { style: "font-size:1.5rem; font-weight:800; color:#fbbf24; margin:0;",
                                    "{d_level}" }
                            }
                        }
                    }

                    SectionDivider { label: "ESTADÍSTICAS" }

                    div { style: "display:grid; grid-template-columns:repeat(4,1fr); gap:10px;",
                        StatBox { label: "PG Actuales", editing: is_ed, display: d_current_hp.read().to_string(), accent: "#10b981",
                            children: rsx! { NumInput32 { value: d_current_hp, min: 0, max: 99999 } } }
                        StatBox { label: "PG Máximos",  editing: is_ed, display: d_max_hp.read().to_string(),     accent: "#78716c",
                            children: rsx! { NumInput32 { value: d_max_hp, min: 1, max: 99999 } } }
                        StatBox { label: "PG Temporal", editing: is_ed, display: d_temp_hp.read().to_string(),    accent: "#818cf8",
                            children: rsx! { NumInput32 { value: d_temp_hp, min: 0, max: 99999 } } }
                        StatBox { label: "Experiencia", editing: is_ed, display: d_xp.read().to_string(),          accent: "#f59e0b",
                            children: rsx! { NumInput64 { value: d_xp } } }
                    }

                    // Barra HP
                    div { style: "display:flex; flex-direction:column; gap:7px;",
                        div { style: "display:flex; justify-content:space-between; font-size:0.7rem;",
                            span { style: "color:#57534e; letter-spacing:0.08em;", "PUNTOS DE GOLPE" }
                            span { style: "color:{bar_col}; font-weight:600;", "{d_current_hp} / {d_max_hp}" }
                        }
                        div { style: "height:10px; border-radius:99px; background:#111110; overflow:hidden; border:1px solid #1c1917;",
                            div { style: "height:100%; border-radius:99px; width:{hp_pct}%; background:{bar_col};" }
                        }
                    }

                    SectionDivider { label: "ATRIBUTOS" }

                    div { style: "display:grid; grid-template-columns:repeat(6,1fr); gap:8px;",
                        AttrWidget { name: "FUE", value: d_str, editing: is_ed }
                        AttrWidget { name: "DES", value: d_dex, editing: is_ed }
                        AttrWidget { name: "CON", value: d_con, editing: is_ed }
                        AttrWidget { name: "INT", value: d_int, editing: is_ed }
                        AttrWidget { name: "SAB", value: d_wis, editing: is_ed }
                        AttrWidget { name: "CAR", value: d_cha, editing: is_ed }
                    }

                    SectionDivider { label: "HABILIDADES" }

                    {
                        let attr_vals = [
                            *d_str.read(), *d_dex.read(), *d_con.read(),
                            *d_int.read(), *d_wis.read(), *d_cha.read(),
                        ];
                        let prof_bonus = proficiency_bonus(*d_level.read());
                        let prof_ids = character.skill_proficiency_ids.clone();
                        rsx! {
                            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:4px;",
                                for (skill_id, skill_name, attr_idx) in skill_table() {
                                    {
                                        let base_mod = modifier(attr_vals[*attr_idx]);
                                        let is_expert = prof_ids.iter()
                                            .any(|id| id == &format!("{skill_id}:expertise"));
                                        let is_prof = is_expert || prof_ids.iter()
                                            .any(|id| id == skill_id);
                                        let total = base_mod
                                            + if is_expert { prof_bonus * 2 }
                                              else if is_prof { prof_bonus }
                                              else { 0 };
                                        let total_str = fmt_mod(total);
                                        let (icon, icon_col) = if is_expert {
                                            ("★★", "#f59e0b")  // maestría
                                        } else if is_prof {
                                            ("★☆", "#78716c")  // competencia
                                        } else {
                                            ("☆☆", "#292524")  // sin competencia
                                        };
                                        let val_col = if total > 0 { "#10b981" }
                                            else if total < 0 { "#ef4444" }
                                            else { "#78716c" };
                                        rsx! {
                                            div {
                                                style: "display:flex; align-items:center; gap:7px;
                                                        padding:5px 10px; border-radius:8px;
                                                        background:#111110; border:1px solid #1c1917;",
                                                span { style: "font-size:0.7rem; color:{icon_col}; flex-shrink:0;", "{icon}" }
                                                span { style: "font-size:0.72rem; color:#a8a29e; flex:1;", "{skill_name}" }
                                                span { style: "font-size:0.8rem; font-weight:700; color:{val_col}; flex-shrink:0;",
                                                    "{total_str}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
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
                        p { style: "font-size:0.83rem; color:#a8a29e; white-space:pre-wrap; line-height:1.65; margin:0;",
                            "{d_notes}" }
                    }
                }
            }

            // ── Card: Inventario + Hechizos ───────────────────────────────
            InventorySpellsCard {
                character: character.clone(),
            }

            div { style: "height:24px;" }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CARD INVENTARIO + HECHIZOS — tabs
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, PartialEq)]
enum CardTab { Items, Spells }

#[component]
fn InventorySpellsCard(character: SavedCharacter) -> Element {
    let mut active_tab: Signal<CardTab> = use_signal(|| CardTab::Items);
    let tab = active_tab.read().clone();

    rsx! {
        div {
            style: "background:#1c1917; border:1px solid #292524; border-radius:20px; overflow:hidden;",
            div { style: "height:3px; background:linear-gradient(90deg,#1c1917 0%,#44403c 50%,#1c1917 100%);" }
            div { style: "padding:20px; display:flex; flex-direction:column; gap:14px;",

                // ── Tabs ──────────────────────────────────────────────────
                div { style: "display:flex; gap:4px; border-bottom:1px solid #292524; padding-bottom:10px;",
                    for (lbl, tval) in [("Inventario", CardTab::Items), ("Hechizos", CardTab::Spells)] {
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

                // ── Contenido según tab ───────────────────────────────────
                if tab == CardTab::Items {
                    InventoryPanel {
                        character_id: character.id,
                        initial_items: character.inventory.clone(),
                        initial_currency: character.currency.clone(),
                    }
                } else {
                    SpellsPanel {
                        character_id: character.id,
                        initial_slots: character.spell_slots.clone(),
                        initial_known: character.known_spells.clone(),
                        initial_prepared: character.prepared_spells.clone(),
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Widgets de UI reutilizables (locales a esta pantalla)
// ═══════════════════════════════════════════════════════════════════════════

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
                p { style: "font-size:1.7rem; font-weight:800; color:{accent}; margin:0;",
                    "{display}" }
            }
        }
    }
}

#[component]
fn NumInput32(mut value: Signal<u32>, min: u32, max: u32) -> Element {
    rsx! {
        input {
            r#type: "number", min: "{min}", max: "{max}",
            style: "width:100%; background:#1c1917; border:1px solid #b45309; border-radius:8px;
                    padding:6px 4px; text-align:center; font-size:1.1rem; font-weight:700;
                    color:#fef3c7; outline:none; box-sizing:border-box;",
            value: "{value}",
            oninput: move |e| { if let Ok(v) = e.value().parse::<u32>() { value.set(v.clamp(min,max)); } },
        }
    }
}

#[component]
fn NumInput64(mut value: Signal<u64>) -> Element {
    rsx! {
        input {
            r#type: "number", min: "0",
            style: "width:100%; background:#1c1917; border:1px solid #b45309; border-radius:8px;
                    padding:6px 4px; text-align:center; font-size:1.1rem; font-weight:700;
                    color:#fef3c7; outline:none; box-sizing:border-box;",
            value: "{value}",
            oninput: move |e| { if let Ok(v) = e.value().parse::<u64>() { value.set(v); } },
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
                    padding:12px 4px 10px; gap:6px;",
            p { style: "font-size:0.58rem; color:#57534e; text-transform:uppercase;
                         letter-spacing:0.1em; font-weight:600; margin:0;", "{name}" }
            if editing {
                input {
                    r#type: "number", min: "1", max: "30",
                    style: "width:100%; background:#1c1917; border:1px solid #b45309; border-radius:8px;
                            padding:4px 2px; text-align:center; font-size:1.1rem; font-weight:700;
                            color:#fef3c7; outline:none; box-sizing:border-box;",
                    value: "{value}",
                    oninput: move |e| { if let Ok(v) = e.value().parse::<u32>() { value.set(v.clamp(1,30)); } },
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
