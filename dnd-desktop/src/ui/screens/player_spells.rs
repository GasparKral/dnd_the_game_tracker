// ═══════════════════════════════════════════════════════════════════════════
// player_spells.rs — Panel de hechizos de un personaje concreto
//
// Bugs corregidos:
//  • Los hechizos known/prepared son POR PERSONAJE (se reciben como props y
//    se sincronizan via polling usando el character_id correcto).
//  • El dropdown de escuela tiene estilo oscuro aplicado.
//  • Lectura de hechizos desde el vault (dnd_type: spell).
// ═══════════════════════════════════════════════════════════════════════════

use crate::states::SharedState;
use crate::vault::frontmatter::DndEntryType;
use dioxus::prelude::*;
use shared::api_types::spells::{
    AddSpellRequest, Spell, SpellComponents, SpellSchool, SpellSlotLevel,
    UpdateSpellSlotsRequest,
};
use std::time::Duration;
use uuid::Uuid;

// ─── Estilos compartidos ────────────────────────────────────────────────────

pub const SELECT_STYLE: &str =
    "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
     background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none;
     box-sizing:border-box; appearance:none; cursor:pointer;";

pub const INPUT_STYLE: &str =
    "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
     background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none;
     box-sizing:border-box;";

// ─── Panel principal de hechizos ────────────────────────────────────────────

#[component]
pub fn SpellsPanel(
    character_id: Uuid,
    initial_slots: Vec<SpellSlotLevel>,
    initial_known: Vec<Spell>,
    initial_prepared: Vec<Spell>,
) -> Element {
    let state = consume_context::<SharedState>().0;

    // Estado vivo — se sincroniza por polling cada 4 s
    let mut live_slots: Signal<Vec<SpellSlotLevel>> = use_signal(|| initial_slots.clone());
    let mut live_known: Signal<Vec<Spell>> = use_signal(|| initial_known.clone());
    let mut live_prepared: Signal<Vec<Spell>> = use_signal(|| initial_prepared.clone());

    // Reset al cambiar de personaje
    {
        let rs = initial_slots.clone();
        let rk = initial_known.clone();
        let rp = initial_prepared.clone();
        use_effect(move || {
            live_slots.set(rs.clone());
            live_known.set(rk.clone());
            live_prepared.set(rp.clone());
        });
    }

    // Polling 4s — sólo para ESTE character_id
    {
        let sp = state.clone();
        use_future(move || {
            let s = sp.clone();
            async move {
                loop {
                    tokio::time::sleep(Duration::from_secs(4)).await;
                    if let Ok(sr) = s.persistence.get_spells(character_id).await {
                        live_slots.set(sr.spell_slots);
                        live_known.set(sr.known_spells);
                        live_prepared.set(sr.prepared_spells);
                    }
                }
            }
        });
    }

    // UI state
    let mut show_add_spell: Signal<bool> = use_signal(|| false);
    let mut show_vault_spells: Signal<bool> = use_signal(|| false);
    let mut vault_spells: Signal<Vec<serde_json::Value>> = use_signal(Vec::new);

    // Snapshots para render
    let slots = live_slots.read().clone();
    let known = live_known.read().clone();
    let prepared = live_prepared.read().clone();

    // Closures de toggle/remove
    let sc_toggle = state.clone();
    let sc_remove = state.clone();

    // Abrir vault de hechizos
    let sv_btn = state.clone();

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",

            // ── Espacios de hechizo ─────────────────────────────────────
            SpellSlotsPanel {
                character_id,
                slots: slots.clone(),
                on_update: move |ns| live_slots.set(ns),
            }

            // ── Hechizos preparados ─────────────────────────────────────
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
                            character_id,
                            on_toggle: move |_| {},
                            on_remove: move |_| {},
                        }
                    }
                }
            }

            // ── Hechizos conocidos ──────────────────────────────────────
            div { style: "display:flex; align-items:center; justify-content:space-between; gap:8px;",
                SectionDivider { label: "CONOCIDOS" }
                div { style: "display:flex; gap:6px; flex-shrink:0;",
                    // Botón añadir desde vault
                    button {
                        style: "padding:4px 10px; font-size:0.62rem; border-radius:8px; cursor:pointer;
                                background:#071a25; color:#67e8f9; border:1px solid #0e7490;",
                        onclick: move |_| {
                            let sv = sv_btn.clone();
                            show_vault_spells.set(true);
                            spawn(async move {
                                if let Ok(entries) = sv.vault.entries_by_kind(DndEntryType::Spell).await {
                                    let list = entries.iter().map(|e| {
                                        serde_json::json!({
                                            "name": e.display_name(),
                                            "level": e.frontmatter.extra.get("level")
                                                .and_then(|v| v.as_u64()).unwrap_or(0),
                                            "school": e.frontmatter.extra.get("school")
                                                .and_then(|v| v.as_str()).unwrap_or("unknown"),
                                            "casting_time": e.frontmatter.extra.get("casting_time")
                                                .and_then(|v| v.as_str()).unwrap_or(""),
                                            "range": e.frontmatter.extra.get("range")
                                                .and_then(|v| v.as_str()).unwrap_or(""),
                                            "duration": e.frontmatter.extra.get("duration")
                                                .and_then(|v| v.as_str()).unwrap_or(""),
                                            "description": e.frontmatter.extra.get("description")
                                                .and_then(|v| v.as_str()).unwrap_or(""),
                                            "damage": e.frontmatter.extra.get("damage")
                                                .and_then(|v| v.as_str()),
                                            "concentration": e.frontmatter.extra.get("concentration")
                                                .and_then(|v| v.as_bool()).unwrap_or(false),
                                            "ritual": e.frontmatter.extra.get("ritual")
                                                .and_then(|v| v.as_bool()).unwrap_or(false),
                                        })
                                    }).collect();
                                    vault_spells.set(list);
                                }
                            });
                        },
                        "📖 Vault"
                    }
                    // Botón añadir manualmente
                    button {
                        style: "padding:4px 10px; font-size:0.62rem; border-radius:8px; cursor:pointer;
                                background:#071a0e; color:#34d399; border:1px solid #065f46;",
                        onclick: move |_| show_add_spell.set(true),
                        "+ Manual"
                    }
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
                            let sc1 = sc_toggle.clone();
                            let sc2 = sc_remove.clone();
                            rsx! {
                                SpellRowDm {
                                    spell: spell.clone(),
                                    is_prepared: is_prep,
                                    character_id,
                                    on_toggle: move |_| {
                                        let s = sc1.clone();
                                        spawn(async move {
                                            if s.persistence.toggle_prepared_spell(character_id, sid).await.is_ok() {
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

        // ── Modal: Añadir hechizo manual ─────────────────────────────────────
        if *show_add_spell.read() {
            AddSpellModal {
                character_id,
                on_close: move || show_add_spell.set(false),
                on_added: move |spell: Spell| {
                    live_known.write().push(spell.clone());
                    if spell.level == 0 { /* trucos no se preparan */ }
                    show_add_spell.set(false);
                },
            }
        }

        // ── Modal: Hechizos del vault ─────────────────────────────────────────
        if *show_vault_spells.read() {
            VaultSpellsModal {
                character_id,
                spells: vault_spells.read().clone(),
                on_close: move || {
                    show_vault_spells.set(false);
                    vault_spells.set(vec![]);
                },
                on_added: move |spell: Spell| {
                    let already = live_known.read().iter().any(|s| s.name == spell.name);
                    if !already {
                        live_known.write().push(spell);
                    }
                },
            }
        }
    }
}

// ─── Modal: añadir hechizo manual ───────────────────────────────────────────

#[component]
fn AddSpellModal(
    character_id: Uuid,
    on_close: EventHandler<()>,
    on_added: EventHandler<Spell>,
) -> Element {
    let state = consume_context::<SharedState>().0;

    let mut spell_name = use_signal(String::new);
    let mut spell_level: Signal<u8> = use_signal(|| 0);
    let mut spell_school = use_signal(|| "unknown".to_string());
    let mut spell_cast = use_signal(String::new);
    let mut spell_range = use_signal(String::new);
    let mut spell_dur = use_signal(String::new);
    let mut spell_desc = use_signal(String::new);
    let mut spell_dmg = use_signal(String::new);
    let mut spell_conc: Signal<bool> = use_signal(|| false);
    let mut spell_ritual: Signal<bool> = use_signal(|| false);
    let mut spell_prepared: Signal<bool> = use_signal(|| false);

    let save = move |_| {
        let school = school_from_str(&spell_school.read());
        let req = AddSpellRequest {
            name: spell_name.read().clone(),
            level: *spell_level.read(),
            school,
            casting_time: spell_cast.read().clone(),
            range: spell_range.read().clone(),
            duration: spell_dur.read().clone(),
            components: SpellComponents::default(),
            description: spell_desc.read().clone(),
            damage: opt_str(&spell_dmg.read()),
            saving_throw: None,
            notes: String::new(),
            concentration: *spell_conc.read(),
            ritual: *spell_ritual.read(),
            prepared: *spell_prepared.read(),
        };
        let s = state.clone();
        spawn(async move {
            if let Ok(spell) = s.persistence.add_known_spell(character_id, req).await {
                on_added.call(spell);
            }
        });
    };

    rsx! {
        div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.78);
                    display:flex; align-items:center; justify-content:center; z-index:60;",
            div {
                style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                        padding:24px; width:480px; max-width:95vw; max-height:88vh;
                        overflow-y:auto; display:flex; flex-direction:column; gap:14px;",

                h3 { style: "font-size:1rem; font-weight:700; color:#fef3c7; margin:0;",
                    "✨ Nuevo Hechizo" }

                // Nombre + nivel
                div { style: "display:grid; grid-template-columns:3fr 1fr; gap:8px;",
                    SpellField { label: "Nombre",
                        input { style: INPUT_STYLE, value: "{spell_name}",
                            oninput: move |e| spell_name.set(e.value()) }
                    }
                    SpellField { label: "Nivel (0=truco)",
                        input {
                            r#type: "number", min: "0", max: "9",
                            style: INPUT_STYLE,
                            value: "{spell_level}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<u8>() { spell_level.set(v.min(9)); }
                            }
                        }
                    }
                }

                // Escuela — con estilo oscuro aplicado
                SpellField { label: "Escuela",
                    select {
                        style: SELECT_STYLE,
                        oninput: move |e| spell_school.set(e.value()),
                        option { value: "abjuration",    "Abjuración" }
                        option { value: "conjuration",   "Conjuración" }
                        option { value: "divination",    "Adivinación" }
                        option { value: "enchantment",   "Encantamiento" }
                        option { value: "evocation",     "Evocación" }
                        option { value: "illusion",      "Ilusión" }
                        option { value: "necromancy",    "Nigromancia" }
                        option { value: "transmutation", "Transmutación" }
                        option { value: "unknown", selected: true, "Desconocida" }
                    }
                }

                // Tiempo / Alcance / Duración
                div { style: "display:grid; grid-template-columns:repeat(3,1fr); gap:8px;",
                    SpellField { label: "Tiempo lanzamiento",
                        input { style: INPUT_STYLE, value: "{spell_cast}",
                            oninput: move |e| spell_cast.set(e.value()) }
                    }
                    SpellField { label: "Alcance",
                        input { style: INPUT_STYLE, value: "{spell_range}",
                            oninput: move |e| spell_range.set(e.value()) }
                    }
                    SpellField { label: "Duración",
                        input { style: INPUT_STYLE, value: "{spell_dur}",
                            oninput: move |e| spell_dur.set(e.value()) }
                    }
                }

                // Daño
                SpellField { label: "Daño (ej: 8d6 fuego)",
                    input { style: INPUT_STYLE, value: "{spell_dmg}",
                        oninput: move |e| spell_dmg.set(e.value()) }
                }

                // Descripción
                SpellField { label: "Descripción",
                    textarea {
                        style: "width:100%; padding:8px 10px; font-size:0.82rem; border-radius:8px;
                                background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                                outline:none; resize:none; min-height:72px; font-family:inherit;
                                box-sizing:border-box;",
                        value: "{spell_desc}",
                        oninput: move |e| spell_desc.set(e.value())
                    }
                }

                // Flags
                div { style: "display:flex; gap:16px; flex-wrap:wrap;",
                    CheckFlag { label: "Concentración", value: spell_conc }
                    CheckFlag { label: "Ritual",        value: spell_ritual }
                    CheckFlag { label: "Preparado",     value: spell_prepared }
                }

                // Acciones
                div { style: "display:flex; justify-content:flex-end; gap:8px;",
                    button {
                        style: "padding:7px 16px; font-size:0.72rem; border-radius:8px; cursor:pointer;
                                background:#1c1917; color:#78716c; border:1px solid #292524;",
                        onclick: move |_| on_close.call(()), "Cancelar"
                    }
                    button {
                        style: "padding:7px 16px; font-size:0.72rem; font-weight:600; border-radius:8px;
                                cursor:pointer; background:#1e3a5f; color:#93c5fd; border:1px solid #1d4ed8;",
                        onclick: save, "✨ Añadir hechizo"
                    }
                }
            }
        }
    }
}

// ─── Modal: hechizos del vault ───────────────────────────────────────────────

#[component]
fn VaultSpellsModal(
    character_id: Uuid,
    spells: Vec<serde_json::Value>,
    on_close: EventHandler<()>,
    on_added: EventHandler<Spell>,
) -> Element {
    let state = consume_context::<SharedState>().0;
    let mut search = use_signal(String::new);
    let mut school_filter = use_signal(|| "all".to_string());

    let search_val = search.read().to_lowercase();
    let school_val = school_filter.read().clone();

    let filtered: Vec<_> = spells.iter().filter(|s| {
        let name = s.get("name").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
        let school = s.get("school").and_then(|v| v.as_str()).unwrap_or("unknown");
        let name_ok = search_val.is_empty() || name.contains(&search_val);
        let school_ok = school_val == "all" || school == school_val;
        name_ok && school_ok
    }).collect();

    rsx! {
        div {
            style: "position:fixed; inset:0; background:rgba(0,0,0,0.82);
                    display:flex; align-items:center; justify-content:center; z-index:60;",
            div {
                style: "background:#1c1917; border:1px solid #44403c; border-radius:18px;
                        padding:24px; width:600px; max-width:95vw; max-height:85vh;
                        display:flex; flex-direction:column; gap:12px;",

                // Header
                div { style: "display:flex; justify-content:space-between; align-items:center;",
                    h3 { style: "font-size:1rem; font-weight:700; color:#fef3c7; margin:0;",
                        "📖 Hechizos del Vault" }
                    button {
                        style: "padding:4px 12px; font-size:0.7rem; border-radius:8px; cursor:pointer;
                                background:#1c1917; color:#78716c; border:1px solid #292524;",
                        onclick: move |_| on_close.call(()), "✕ Cerrar"
                    }
                }

                // Búsqueda
                input {
                    style: "padding:7px 12px; font-size:0.78rem; border-radius:9px;
                            background:#0c0a09; border:1px solid #292524; color:#e7e5e4;
                            outline:none; width:100%; box-sizing:border-box;",
                    placeholder: "🔍 Buscar hechizo…",
                    value: "{search}",
                    oninput: move |e| search.set(e.value()),
                }

                // Filtros de escuela
                div { style: "display:flex; gap:5px; flex-wrap:wrap;",
                    for (val, lbl) in [
                        ("all","Todos"),("abjuration","Abjuración"),("conjuration","Conjuración"),
                        ("divination","Adivinación"),("enchantment","Encantamiento"),
                        ("evocation","Evocación"),("illusion","Ilusión"),
                        ("necromancy","Nigromancia"),("transmutation","Transmutación"),
                    ] {
                        {
                            let v2 = val.to_string();
                            let active = *school_filter.read() == val;
                            let (bg, col, brd) = if active {
                                ("#1e1030","#a78bfa","#4c1d95")
                            } else {
                                ("#0c0a09","#78716c","#1c1917")
                            };
                            rsx! {
                                button {
                                    style: "padding:3px 10px; font-size:0.62rem; border-radius:16px;
                                            background:{bg}; color:{col}; border:1px solid {brd}; cursor:pointer;",
                                    onclick: move |_| school_filter.set(v2.clone()),
                                    "{lbl}"
                                }
                            }
                        }
                    }
                }

                p { style: "font-size:0.66rem; color:#57534e; margin:0;",
                    "{filtered.len()} resultado(s)" }

                // Lista
                if spells.is_empty() {
                    p { style: "color:#44403c; font-size:0.8rem; text-align:center; padding:24px 0;",
                        "No hay hechizos con dnd_type:spell en el vault, o el vault no está configurado." }
                } else if filtered.is_empty() {
                    p { style: "color:#44403c; font-size:0.8rem; text-align:center; padding:24px 0;",
                        "Ningún hechizo coincide con los filtros." }
                } else {
                    div { style: "overflow-y:auto; max-height:360px; display:flex; flex-direction:column; gap:6px; padding-right:4px;",
                        for vs in filtered.iter() {
                            {
                                let name     = vs.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let level    = vs.get("level").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                                let school_s = vs.get("school").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                                let cast     = vs.get("casting_time").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let range    = vs.get("range").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let dur      = vs.get("duration").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let desc     = vs.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                let dmg      = vs.get("damage").and_then(|v| v.as_str()).map(str::to_string);
                                let conc     = vs.get("concentration").and_then(|v| v.as_bool()).unwrap_or(false);
                                let ritual   = vs.get("ritual").and_then(|v| v.as_bool()).unwrap_or(false);
                                let level_lbl = if level == 0 { "Truco".to_string() } else { format!("Nv{level}") };
                                let school_e = school_from_str(&school_s);

                                let sa = state.clone();
                                let name2 = name.clone();
                                let school_s2 = school_s.clone();

                                rsx! {
                                    div {
                                        style: "display:flex; justify-content:space-between; align-items:center;
                                                padding:9px 14px; background:#111110; border-radius:10px;
                                                border:1px solid #1c1917; gap:10px;",
                                        div { style: "flex:1; min-width:0; display:flex; flex-direction:column; gap:2px;",
                                            div { style: "display:flex; align-items:center; gap:8px;",
                                                span { style: "font-size:0.6rem; font-weight:700; color:#a78bfa;
                                                               background:#1e1030; border:1px solid #4c1d95;
                                                               border-radius:6px; padding:2px 6px;",
                                                    "{level_lbl}" }
                                                span { style: "font-size:0.82rem; font-weight:600; color:#fef3c7;",
                                                    "{name}" }
                                                span { style: "font-size:0.62rem; color:#78716c;", "{school_s}" }
                                            }
                                            if !desc.is_empty() {
                                                p { style: "font-size:0.68rem; color:#78716c; margin:0;
                                                        white-space:nowrap; overflow:hidden; text-overflow:ellipsis;",
                                                    "{desc}" }
                                            }
                                        }
                                        button {
                                            style: "padding:5px 12px; font-size:0.65rem; border-radius:8px;
                                                    cursor:pointer; border:1px solid #166534;
                                                    background:#052e16; color:#34d399; white-space:nowrap; flex-shrink:0;",
                                            onclick: move |_| {
                                                use shared::api_types::spells::AddSpellRequest;
                                                let req = AddSpellRequest {
                                                    name: name2.clone(),
                                                    level,
                                                    school: school_from_str(&school_s2),
                                                    casting_time: cast.clone(),
                                                    range: range.clone(),
                                                    duration: dur.clone(),
                                                    components: SpellComponents::default(),
                                                    description: desc.clone(),
                                                    damage: dmg.clone(),
                                                    saving_throw: None,
                                                    notes: String::new(),
                                                    concentration: conc,
                                                    ritual,
                                                    prepared: false,
                                                };
                                                let s = sa.clone();
                                                spawn(async move {
                                                    if let Ok(spell) = s.persistence.add_known_spell(character_id, req).await {
                                                        on_added.call(spell);
                                                    }
                                                });
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

// ─── SpellSlotsPanel ─────────────────────────────────────────────────────────

#[component]
pub fn SpellSlotsPanel(
    character_id: Uuid,
    slots: Vec<SpellSlotLevel>,
    on_update: EventHandler<Vec<SpellSlotLevel>>,
) -> Element {
    let state = consume_context::<SharedState>().0;
    let mut local: Signal<Vec<SpellSlotLevel>> = use_signal(|| slots.clone());
    use_effect(move || { local.set(slots.clone()); });

    let save_slots = {
        let s = state.clone();
        move |_| {
            let req = UpdateSpellSlotsRequest { slots: local.read().clone() };
            let s2 = s.clone();
            let snap = local.read().clone();
            spawn(async move { let _ = s2.persistence.update_spell_slots(character_id, req).await; });
            on_update.call(snap);
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
            div { style: "display:grid; grid-template-columns:repeat(9,1fr); gap:6px;",
                for lvl_idx in 0..9u8 {
                    {
                        let lvl_num = lvl_idx + 1;
                        let slot = local.read().iter()
                            .find(|s| s.level == lvl_num)
                            .cloned()
                            .unwrap_or(SpellSlotLevel { level: lvl_num, total: 0, remaining: 0 });
                        rsx! {
                            div { style: "display:flex; flex-direction:column; align-items:center; gap:4px;",
                                span { style: "font-size:0.58rem; color:#57534e;", "Nv{lvl_num}" }
                                input {
                                    r#type: "number", min: "0", max: "9",
                                    style: "width:100%; background:#1c1917; border:1px solid #44403c;
                                            border-radius:6px; padding:4px 2px; text-align:center;
                                            font-size:0.85rem; color:#e7e5e4; outline:none; box-sizing:border-box;",
                                    title: "Total",
                                    value: "{slot.total}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u8>() {
                                            let mut s = local.write();
                                            if let Some(ex) = s.iter_mut().find(|x| x.level == lvl_num) {
                                                ex.total = v;
                                                if ex.remaining > v { ex.remaining = v; }
                                            } else {
                                                s.push(SpellSlotLevel { level: lvl_num, total: v, remaining: v });
                                            }
                                        }
                                    }
                                }
                                input {
                                    r#type: "number", min: "0", max: "{slot.total}",
                                    style: "width:100%; background:#1c1917; border:1px solid #065f46;
                                            border-radius:6px; padding:4px 2px; text-align:center;
                                            font-size:0.85rem; color:#34d399; outline:none; box-sizing:border-box;",
                                    title: "Restantes",
                                    value: "{slot.remaining}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<u8>() {
                                            let mut s = local.write();
                                            if let Some(ex) = s.iter_mut().find(|x| x.level == lvl_num) {
                                                ex.remaining = v.min(ex.total);
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

// ─── SpellRowDm ──────────────────────────────────────────────────────────────

#[component]
pub fn SpellRowDm(
    spell: Spell,
    is_prepared: bool,
    character_id: Uuid,
    on_toggle: EventHandler<Uuid>,
    on_remove: EventHandler<Uuid>,
) -> Element {
    let mut expanded = use_signal(|| false);
    let level_str = if spell.level == 0 { "Truco".to_string() } else { format!("Nv{}", spell.level) };
    let school_str = spell.school.label();
    let prep_bg  = if is_prepared { "#1a2e0a" } else { "#111110" };
    let prep_col = if is_prepared { "#86efac" } else { "#57534e" };
    let prep_brd = if is_prepared { "#166534" } else { "#292524" };
    let conc_badge = if spell.concentration { " ◇C" } else { "" };
    let rit_badge  = if spell.ritual { " ◆R" } else { "" };
    let sid = spell.id;

    rsx! {
        div { style: "background:#111110; border:1px solid #292524; border-radius:10px; overflow:hidden;",
            div {
                style: "display:flex; align-items:center; gap:8px; padding:9px 12px; cursor:pointer;",
                onclick: move |_| { let v = *expanded.read(); expanded.set(!v); },
                span {
                    style: "flex-shrink:0; font-size:0.6rem; font-weight:700; color:#a78bfa;
                            background:#1e1030; border:1px solid #4c1d95; border-radius:6px; padding:2px 6px;",
                    "{level_str}"
                }
                span {
                    style: "flex:1; font-size:0.83rem; font-weight:500; color:#d6d3d1;
                            overflow:hidden; text-overflow:ellipsis; white-space:nowrap;",
                    "{spell.name}{conc_badge}{rit_badge}"
                }
                span { style: "font-size:0.62rem; color:#78716c; flex-shrink:0;", "{school_str}" }
                button {
                    style: "flex-shrink:0; padding:2px 8px; font-size:0.6rem; border-radius:6px;
                            background:{prep_bg}; color:{prep_col}; border:1px solid {prep_brd}; cursor:pointer;",
                    onclick: move |e| { e.stop_propagation(); on_toggle.call(sid); },
                    if is_prepared { "✔ Prep" } else { "Preparar" }
                }
                button {
                    style: "flex-shrink:0; padding:2px 7px; font-size:0.6rem; border-radius:6px;
                            background:#200a0a; color:#f87171; border:1px solid #7f1d1d; cursor:pointer;",
                    onclick: move |e| { e.stop_propagation(); on_remove.call(sid); },
                    "×"
                }
                span { style: "font-size:0.65rem; color:#57534e;",
                    if *expanded.read() { "▴" } else { "▾" }
                }
            }
            if *expanded.read() {
                div {
                    style: "padding:10px 14px; border-top:1px solid #1c1917;
                            display:flex; flex-direction:column; gap:7px;",
                    div { style: "display:flex; flex-wrap:wrap; gap:10px;",
                        if !spell.casting_time.is_empty() { SpellMeta { label: "⏱ Tiempo", value: spell.casting_time.clone() } }
                        if !spell.range.is_empty()        { SpellMeta { label: "🎯 Alcance", value: spell.range.clone() } }
                        if !spell.duration.is_empty()     { SpellMeta { label: "⌛ Duración", value: spell.duration.clone() } }
                        if let Some(dmg) = &spell.damage  { SpellMeta { label: "⚔️ Daño", value: dmg.clone() } }
                        if let Some(sv)  = &spell.saving_throw { SpellMeta { label: "🛡 Salvación", value: sv.clone() } }
                    }
                    if !spell.description.is_empty() {
                        p { style: "font-size:0.78rem; color:#a8a29e; line-height:1.55;
                                    white-space:pre-wrap; margin:0;", "{spell.description}" }
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

// ─── Widgets auxiliares ──────────────────────────────────────────────────────

#[component]
pub fn SectionDivider(label: &'static str) -> Element {
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
fn SpellField(label: &'static str, children: Element) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:4px;",
            label { style: "font-size:0.62rem; color:#78716c;", "{label}" }
            { children }
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
fn CheckFlag(label: &'static str, mut value: Signal<bool>) -> Element {
    let checked = *value.read();
    let col = if checked { "#fbbf24" } else { "#57534e" };
    let check_bg = if checked { "#92400e" } else { "transparent" };
    rsx! {
        button {
            style: "display:flex; align-items:center; gap:6px; background:none; border:none;
                    cursor:pointer; color:{col}; font-size:0.75rem; padding:0;",
            onclick: move |_| { let v = *value.read(); value.set(!v); },
            span { style: "width:14px; height:14px; border-radius:4px; border:1px solid {col};
                           background:{check_bg}; display:flex; align-items:center; justify-content:center;",
                if checked { span { style: "font-size:0.6rem; color:#fef3c7;", "✓" } }
            }
            "{label}"
        }
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

pub fn school_from_str(s: &str) -> SpellSchool {
    match s {
        "abjuration"    => SpellSchool::Abjuration,
        "conjuration"   => SpellSchool::Conjuration,
        "divination"    => SpellSchool::Divination,
        "enchantment"   => SpellSchool::Enchantment,
        "evocation"     => SpellSchool::Evocation,
        "illusion"      => SpellSchool::Illusion,
        "necromancy"    => SpellSchool::Necromancy,
        "transmutation" => SpellSchool::Transmutation,
        _               => SpellSchool::Unknown,
    }
}

fn opt_str(s: &str) -> Option<String> {
    if s.is_empty() { None } else { Some(s.to_string()) }
}
