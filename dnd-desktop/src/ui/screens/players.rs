use crate::states::SharedState;
use chrono;
use dioxus::prelude::*;
use shared::api_types::inventory::{Currency, InventoryItem};
use shared::persistence::SavedCharacter;
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

            // ── CARD 2: Inventario ─────────────────────────────────────────
            div {
                style: "background:#1c1917; border:1px solid #292524; border-radius:20px; overflow:hidden;",
                div { style: "height:3px; background:linear-gradient(90deg,#1c1917 0%,#44403c 50%,#1c1917 100%);" }
                div { style: "padding:20px; display:flex; flex-direction:column; gap:16px;",
                    SectionDivider { label: "INVENTARIO" }
                    CurrencyRow { currency: character.currency.clone() }
                    if character.inventory.is_empty() {
                        div {
                            style: "display:flex; align-items:center; justify-content:center;
                                    height:60px; font-size:0.78rem; color:#44403c;
                                    border:1px dashed #292524; border-radius:12px;",
                            "El inventario está vacío."
                        }
                    } else {
                        div {
                            style: "border:1px solid #292524; border-radius:14px; overflow:hidden;",
                            div {
                                style: "display:grid; grid-template-columns:5fr 3fr 2fr 2fr;
                                        padding:7px 16px; background:#111110; border-bottom:1px solid #1c1917;
                                        font-size:0.62rem; color:#57534e; text-transform:uppercase; letter-spacing:0.09em;",
                                div { "Objeto" }
                                div { "Categoría" }
                                div { style: "text-align:center;", "Cant." }
                                div { style: "text-align:right;", "Peso" }
                            }
                            for item in character.inventory.iter() {
                                ItemRow { item: item.clone() }
                            }
                        }
                        {
                            let total: f32 = character.inventory.iter()
                                .map(|i| i.weight.unwrap_or(0.0) * i.quantity as f32).sum();
                            rsx! {
                                p { style: "font-size:0.68rem; color:#44403c; text-align:right; margin:0;",
                                    "Peso total: {total:.1} lb." }
                            }
                        }
                    }
                }
            }
            div { style: "height:24px;" }
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
