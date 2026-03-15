// =============================================================================
// ui/screens/combat.rs — Pantalla de gestión de combate para el DM
// Dioxus 0.7
// =============================================================================

#![allow(non_snake_case)]

use dioxus::prelude::*;
use shared::api_types::combat::{
    AddCombatantRequest, AddFromTemplateRequest, CombatState, Combatant, CombatantKind, Condition,
    EnemyTemplate, RollInitiativeResponse, UpdateConditionsRequest, UpdateHpRequest,
};
use shared::persistence::SavedCharacter;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Helpers HTTP — funciones async puras sin signals
// ---------------------------------------------------------------------------

fn api(path: &str) -> String {
    format!("http://localhost:3000/api{}", path)
}

async fn fetch_combat() -> Option<CombatState> {
    reqwest::get(api("/combat")).await.ok()?.json().await.ok()
}

async fn fetch_characters() -> Vec<SavedCharacter> {
    #[derive(serde::Deserialize)]
    struct Resp { characters: Vec<SavedCharacter> }
    let Ok(resp) = reqwest::get(api("/characters")).await else { return vec![]; };
    resp.json::<Resp>().await.map(|r| r.characters).unwrap_or_default()
}

async fn post_empty(path: &str) -> Option<CombatState> {
    reqwest::Client::new()
        .post(api(path))
        .header("content-type", "application/json")
        .body("{}")
        .send().await.ok()?
        .json().await.ok()
}

async fn post_json<T: serde::Serialize, R: serde::de::DeserializeOwned>(
    path: &str, body: T,
) -> Option<R> {
    reqwest::Client::new()
        .post(api(path)).json(&body)
        .send().await.ok()?
        .json().await.ok()
}

async fn patch_json<T: serde::Serialize, R: serde::de::DeserializeOwned>(
    path: &str, body: T,
) -> Option<R> {
    reqwest::Client::new()
        .patch(api(path)).json(&body)
        .send().await.ok()?
        .json().await.ok()
}

async fn http_delete(path: &str) -> bool {
    reqwest::Client::new()
        .delete(api(path))
        .send().await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Panel activo
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
enum Panel { None, Players, Manual, Template }

// ---------------------------------------------------------------------------
// CombatScreen — pantalla principal
// ---------------------------------------------------------------------------

#[component]
pub fn CombatScreen() -> Element {
    let mut combat: Signal<Option<CombatState>> = use_signal(|| None);
    let mut panel = use_signal(|| Panel::None);
    let mut detail_id: Signal<Option<Uuid>> = use_signal(|| None);
    let mut roll_log: Signal<Vec<String>>   = use_signal(Vec::new);

    // Carga inicial del estado de combate
    use_effect(move || {
        spawn(async move { *combat.write() = fetch_combat().await; });
    });

    // ── Acciones de ciclo ─────────────────────────────────────────────────

    let on_start = move |_: Event<MouseData>| {
        spawn(async move {
            if let Some(s) = post_empty("/combat/start").await { *combat.write() = Some(s); }
        });
    };
    let on_end = move |_: Event<MouseData>| {
        spawn(async move {
            if let Some(s) = post_empty("/combat/end").await { *combat.write() = Some(s); }
        });
    };
    let on_reset = move |_: Event<MouseData>| {
        spawn(async move {
            if let Some(s) = post_empty("/combat/reset").await {
                *combat.write()    = Some(s);
                *roll_log.write()  = Vec::new();
                *detail_id.write() = None;
            }
        });
    };
    let on_next = move |_: Event<MouseData>| {
        spawn(async move {
            if let Some(s) = post_empty("/combat/next-turn").await { *combat.write() = Some(s); }
        });
    };
    let on_roll = move |_: Event<MouseData>| {
        #[derive(serde::Serialize)]
        struct Req { reroll_all: bool }
        spawn(async move {
            let resp: Option<RollInitiativeResponse> =
                post_json("/combat/roll-initiative", Req { reroll_all: true }).await;
            if let Some(r) = resp {
                *roll_log.write() = r.rolls.iter().map(|x|
                    format!("{}: d20({}) + {} = {}", x.name, x.d20_roll, x.bonus, x.total)
                ).collect();
                *combat.write() = Some(r.state);
            }
        });
    };

    // ── Acciones sobre combatientes ───────────────────────────────────────

    let on_remove = move |id: Uuid| {
        spawn(async move {
            if http_delete(&format!("/combat/combatant/{}", id)).await {
                *combat.write() = fetch_combat().await;
                if detail_id() == Some(id) { *detail_id.write() = None; }
            }
        });
    };

    let on_hp = move |id: Uuid, delta: i32| {
        spawn(async move {
            let _: Option<Combatant> = patch_json(
                &format!("/combat/combatant/{}/hp", id),
                UpdateHpRequest { delta, temporary: false, set_temp: false },
            ).await;
            *combat.write() = fetch_combat().await;
        });
    };

    let on_goto_turn = move |id: Uuid| {
        spawn(async move {
            if let Some(s) = post_empty(&format!("/combat/turn/{}", id)).await {
                *combat.write() = Some(s);
            }
        });
    };

    let on_condition = move |id: Uuid, cond: Condition| {
        spawn(async move {
            if let Some(s) = fetch_combat().await {
                if let Some(c) = s.combatant_by_id(id) {
                    let mut conds = c.conditions.clone();
                    if conds.contains(&cond) { conds.retain(|x| x != &cond); }
                    else { conds.push(cond); }
                    let _: Option<Combatant> = patch_json(
                        &format!("/combat/combatant/{}/conditions", id),
                        UpdateConditionsRequest { conditions: conds, concentrating_on: c.concentrating_on.clone() },
                    ).await;
                    *combat.write() = fetch_combat().await;
                }
            }
        });
    };

    // Callback que los paneles invocan cuando han terminado de guardar.
    // Cierra el panel y recarga el estado de combate.
    // IMPORTANTE: no puede pasarse dentro de un spawn (EventHandler no es Send).
    // Lo usamos directamente desde el contexto reactivo del componente hijo.
    let mut reload_combat = move || {
        *panel.write() = Panel::None;
        spawn(async move { *combat.write() = fetch_combat().await; });
    };

    // ── Datos para el render ──────────────────────────────────────────────

    let is_active  = combat().as_ref().map(|s| s.active).unwrap_or(false);
    let round      = combat().as_ref().map(|s| s.round).unwrap_or(1);
    let cur_idx    = combat().as_ref().map(|s| s.current_turn_index).unwrap_or(0);
    let combatants = combat().map(|s| s.combatants).unwrap_or_default();
    let detail_snap = detail_id().and_then(|id| combatants.iter().find(|c| c.id == id).cloned());

    rsx! {
        div { class: "flex flex-col h-full bg-stone-950 text-stone-100 overflow-hidden",

            // ── Cabecera ──────────────────────────────────────────────────
            div { class: "flex items-center justify-between px-6 py-3 bg-stone-900 border-b border-stone-800 shrink-0",
                div { class: "flex items-center gap-3",
                    h1 { class: "text-xl font-bold text-amber-400 font-serif tracking-wide", "⚔ Combate" }
                    if is_active {
                        span { class: "text-xs px-2 py-0.5 rounded-full bg-red-800 text-red-100 font-semibold",
                            "RONDA {round}"
                        }
                    }
                }
                div { class: "flex gap-2 flex-wrap",
                    if !is_active {
                        button { class: BTN_GREEN, onclick: on_start, "▶ Iniciar" }
                    } else {
                        button { class: BTN_BLUE,  onclick: on_next,  "→ Sig. turno" }
                        button { class: BTN_STONE, onclick: on_end,   "■ Terminar" }
                    }
                    button { class: BTN_AMBER, onclick: on_roll,  "🎲 Tirar iniciativa" }
                    button { class: BTN_RED,   onclick: on_reset, "⟳ Resetear" }
                }
            }

            // ── Log de iniciativa ─────────────────────────────────────────
            if !roll_log().is_empty() {
                div { class: "mx-4 mt-2 p-3 bg-stone-900 border border-stone-700 rounded-lg font-mono shrink-0",
                    div { class: "flex justify-between items-center mb-1",
                        span { class: "text-amber-400 text-xs font-semibold uppercase tracking-wider",
                            "🎲 Iniciativa"
                        }
                        button { class: "text-stone-500 hover:text-stone-300 text-xs",
                            onclick: move |_| roll_log.write().clear(), "✕"
                        }
                    }
                    for line in roll_log() {
                        p { class: "text-stone-300 text-xs leading-5", "{line}" }
                    }
                }
            }

            // ── Cuerpo ────────────────────────────────────────────────────
            div { class: "flex flex-1 overflow-hidden",

                div { class: "flex flex-col flex-1 overflow-hidden",

                    // Barra de botones de panel
                    div { class: "flex gap-2 px-4 py-2 shrink-0 border-b border-stone-800 flex-wrap",
                        PanelBtn {
                            active: panel() == Panel::Players,
                            label_off: "👤 Añadir jugadores",
                            label_on:  "✕ Jugadores",
                            onclick: move |_| {
                                *panel.write() = if panel() == Panel::Players { Panel::None } else { Panel::Players };
                            }
                        }
                        PanelBtn {
                            active: panel() == Panel::Manual,
                            label_off: "+ Combatiente manual",
                            label_on:  "✕ Cancelar",
                            onclick: move |_| {
                                *panel.write() = if panel() == Panel::Manual { Panel::None } else { Panel::Manual };
                            }
                        }
                        PanelBtn {
                            active: panel() == Panel::Template,
                            label_off: "🐉 Plantilla monstruo",
                            label_on:  "✕ Cancelar",
                            onclick: move |_| {
                                *panel.write() = if panel() == Panel::Template { Panel::None } else { Panel::Template };
                            }
                        }
                    }

                    // Paneles de formulario
                    // on_done usa un signal de "petición de recarga" para cruzar el boundary
                    // sin mover EventHandler a un spawn
                    if panel() == Panel::Players {
                        PlayersPanel {
                            on_done: move |_| reload_combat()
                        }
                    }
                    if panel() == Panel::Manual {
                        ManualForm {
                            on_done: move |_| reload_combat()
                        }
                    }
                    if panel() == Panel::Template {
                        TemplateForm {
                            on_done: move |_| reload_combat()
                        }
                    }

                    // Lista de combatientes
                    div { class: "flex-1 overflow-y-auto px-4 py-4", style: "display: flex; flex-direction: column; gap: 10px;",
                        if combatants.is_empty() {
                            div { class: "text-center py-16 text-sm select-none",
                                p { class: "text-4xl mb-3", "⚔️" }
                                p { class: "font-medium text-stone-500", "Sin combatientes." }
                                p { class: "text-stone-700 text-xs mt-1",
                                    "Usa los botones de arriba para añadir jugadores o enemigos."
                                }
                            }
                        }
                        for (idx, c) in combatants.iter().enumerate() {
                            {
                                let id      = c.id;
                                let is_turn = idx == cur_idx && is_active;
                                let is_sel  = detail_id() == Some(id);
                                let snap    = c.clone();
                                rsx! {
                                    CombatCard {
                                        key: "{id}",
                                        combatant: snap,
                                        is_active_turn: is_turn,
                                        is_selected: is_sel,
                                        on_select: move |_| {
                                            *detail_id.write() = if detail_id() == Some(id) { None } else { Some(id) };
                                        },
                                        on_remove:    move |_|            { on_remove(id); },
                                        on_damage:    move |d: i32|       { on_hp(id, -d); },
                                        on_heal:      move |d: i32|       { on_hp(id,  d); },
                                        on_goto_turn: move |_|            { on_goto_turn(id); },
                                        on_condition: move |c: Condition| { on_condition(id, c); },
                                    }
                                }
                            }
                        }
                    }
                }

                // Panel lateral de detalle
                if let Some(c) = detail_snap {
                    DetailPanel {
                        combatant: c,
                        on_condition: move |cond: Condition| {
                            if let Some(id) = detail_id() { on_condition(id, cond); }
                        },
                        on_close: move |_| *detail_id.write() = None,
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PlayersPanel
// ---------------------------------------------------------------------------

/// El truco para cruzar el boundary async-EventHandler en Dioxus 0.7:
/// - Los datos HTTP se preparan síncronamente (leyendo signals antes del spawn)
/// - El spawn hace el HTTP
/// - Al terminar el spawn escribe en un signal local `done`
/// - Un use_effect reactivo a `done` llama a on_done() síncronamente
#[component]
fn PlayersPanel(on_done: EventHandler<()>) -> Element {
    let mut chars: Signal<Vec<SavedCharacter>> = use_signal(Vec::new);
    let mut loading  = use_signal(|| true);
    let mut selected: Signal<Vec<Uuid>> = use_signal(Vec::new);
    let mut error    = use_signal(|| Option::<String>::None);
    // done: se pone a true cuando el spawn HTTP termina → dispara el effect
    let mut done     = use_signal(|| false);

    // Cargar personajes al montar
    use_effect(move || {
        spawn(async move {
            *loading.write() = true;
            let result = fetch_characters().await;
            if result.is_empty() {
                *error.write() = Some("No hay personajes en la campaña activa.".into());
            }
            *chars.write() = result;
            *loading.write() = false;
        });
    });

    // Reacciona a done: llama on_done síncronamente (fuera de spawn)
    use_effect(move || {
        if done() {
            *done.write() = false;
            on_done.call(());
        }
    });

    let mut toggle = move |id: Uuid| {
        let mut sel = selected.write();
        if sel.contains(&id) { sel.retain(|x| *x != id); }
        else { sel.push(id); }
    };

    let submit = move |_: Event<MouseData>| {
        if selected().is_empty() { return; }

        // Construir requests ANTES del spawn (aquí tenemos acceso síncrono a signals)
        let requests: Vec<AddCombatantRequest> = chars()
            .into_iter()
            .filter(|c| selected().contains(&c.id))
            .map(|ch| {
                let dex_mod = ((ch.attributes.dexterity as i32) - 10) / 2;
                let base_ac = (10 + dex_mod).max(10) as u32;
                AddCombatantRequest {
                    name:             ch.name.clone(),
                    kind:             CombatantKind::Player,
                    hp_max:           ch.max_hp as i32,
                    armor_class:      base_ac,
                    initiative_bonus: dex_mod,
                    strength:     Some(ch.attributes.strength as i32),
                    dexterity:    Some(ch.attributes.dexterity as i32),
                    constitution: Some(ch.attributes.constitution as i32),
                    intelligence: Some(ch.attributes.intelligence as i32),
                    wisdom:       Some(ch.attributes.wisdom as i32),
                    charisma:     Some(ch.attributes.charisma as i32),
                    abilities:    vec![],
                    character_id: Some(ch.id),
                    legendary_action_count: None,
                }
            })
            .collect();

        if requests.is_empty() { return; }

        // El spawn solo recibe datos primitivos (no signals, no handlers)
        spawn(async move {
            let client = reqwest::Client::new();
            for req in requests {
                let _ = client.post(api("/combat/combatant")).json(&req).send().await;
            }
            // Señal al effect para que llame on_done fuera del spawn
            *done.write() = true;
        });
    };

    let all_sel = !chars().is_empty() && chars().iter().all(|c| selected().contains(&c.id));

    rsx! {
        div { class: "shrink-0 border-b border-stone-700 bg-stone-950 px-4 py-3",
            div { class: "flex items-center justify-between mb-3",
                p { class: "text-xs font-semibold uppercase tracking-wider text-sky-400",
                    "👤 Seleccionar jugadores"
                }
                if !chars().is_empty() {
                    button {
                        class: "text-xs text-stone-400 hover:text-stone-200 underline",
                        onclick: move |_| {
                            if all_sel { *selected.write() = vec![]; }
                            else { *selected.write() = chars().iter().map(|c| c.id).collect(); }
                        },
                        if all_sel { "Deseleccionar todos" } else { "Seleccionar todos" }
                    }
                }
            }

            if loading() {
                p { class: "text-stone-500 text-sm text-center py-4 italic", "Cargando personajes…" }
            } else if let Some(msg) = error() {
                p { class: "text-red-400 text-sm text-center py-3 italic", "{msg}" }
            } else {
                div { class: "space-y-1 max-h-52 overflow-y-auto mb-3",
                    for ch in chars() {
                        {
                            let id      = ch.id;
                            let is_sel  = selected().contains(&id);
                            let dex_mod = ((ch.attributes.dexterity as i32) - 10) / 2;
                            let dex_str = if dex_mod >= 0 { format!("+{}", dex_mod) } else { dex_mod.to_string() };
                            let row_cls = if is_sel {
                                "flex items-center gap-3 px-3 py-2 rounded border cursor-pointer transition bg-sky-900/50 border-sky-600"
                            } else {
                                "flex items-center gap-3 px-3 py-2 rounded border cursor-pointer transition bg-stone-900 border-stone-700 hover:border-stone-500"
                            };
                            let cb_cls = if is_sel {
                                "w-4 h-4 rounded border-2 flex items-center justify-center shrink-0 bg-sky-500 border-sky-500"
                            } else {
                                "w-4 h-4 rounded border-2 flex items-center justify-center shrink-0 border-stone-500"
                            };
                            rsx! {
                                div {
                                    key: "{id}",
                                    class: "{row_cls}",
                                    onclick: move |_| toggle(id),
                                    div { class: "{cb_cls}",
                                        if is_sel {
                                            span { class: "text-white text-xs font-bold leading-none", "✓" }
                                        }
                                    }
                                    div { class: "flex-1 min-w-0",
                                        div { class: "flex items-baseline gap-1.5",
                                            span { class: "font-semibold text-sm text-stone-100 truncate", "{ch.name}" }
                                            span { class: "text-xs text-stone-500 shrink-0", "· {ch.player_name}" }
                                        }
                                        span { class: "text-xs text-stone-500", "Nv {ch.level} {ch.class_id}" }
                                    }
                                    div { class: "flex gap-1.5 text-xs font-mono shrink-0",
                                        span { class: "bg-emerald-900/50 text-emerald-300 px-1.5 py-0.5 rounded",
                                            "❤ {ch.current_hp}/{ch.max_hp}"
                                        }
                                        span { class: "bg-amber-900/50 text-amber-300 px-1.5 py-0.5 rounded",
                                            "DEX {dex_str}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    let lbl = if selected().is_empty() {
                        "Selecciona jugadores".to_string()
                    } else {
                        format!("Añadir {} al combate", selected().len())
                    };
                    let cls = if selected().is_empty() { BTN_DISABLED } else { BTN_BLUE };
                    rsx! {
                        button {
                            class: "{cls}",
                            disabled: selected().is_empty(),
                            onclick: submit,
                            "{lbl}"
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ManualForm
// ---------------------------------------------------------------------------

#[component]
fn ManualForm(on_done: EventHandler<()>) -> Element {
    let mut name   = use_signal(String::new);
    let mut kind   = use_signal(|| "enemy".to_string());
    let mut hp     = use_signal(|| "20".to_string());
    let mut ca     = use_signal(|| "12".to_string());
    let mut init_b = use_signal(|| "0".to_string());
    let mut count  = use_signal(|| "1".to_string());
    let mut err    = use_signal(|| false);
    let mut done   = use_signal(|| false);

    // Llama on_done síncronamente cuando el spawn termina
    use_effect(move || {
        if done() {
            *done.write() = false;
            on_done.call(());
        }
    });

    let submit = move |_: Event<MouseData>| {
        let n = name().trim().to_string();
        if n.is_empty() { *err.write() = true; return; }

        let hp_v: i32 = hp().parse().unwrap_or(10);
        let ca_v: u32 = ca().parse().unwrap_or(10);
        let ib: i32   = init_b().parse().unwrap_or(0);
        let cnt: u32  = count().parse().unwrap_or(1).max(1).min(20);

        let kind_val = match kind().as_str() {
            "player"    => CombatantKind::Player,
            "companion" => CombatantKind::Companion,
            "npc"       => CombatantKind::Npc,
            _           => CombatantKind::Enemy,
        };

        let requests: Vec<AddCombatantRequest> = (1..=cnt).map(|i| {
            let suffix = if cnt > 1 { format!(" {}", i) } else { String::new() };
            AddCombatantRequest {
                name: format!("{}{}", n, suffix),
                kind: kind_val.clone(),
                hp_max: hp_v, armor_class: ca_v, initiative_bonus: ib,
                strength: None, dexterity: None, constitution: None,
                intelligence: None, wisdom: None, charisma: None,
                abilities: vec![], character_id: None,
                legendary_action_count: None,
            }
        }).collect();

        spawn(async move {
            let client = reqwest::Client::new();
            for req in requests {
                let _ = client.post(api("/combat/combatant")).json(&req).send().await;
            }
            *done.write() = true;
        });
    };

    rsx! {
        div { class: "shrink-0 border-b border-stone-700 bg-stone-950 px-4 py-3",
            p { class: "text-xs font-semibold uppercase tracking-wider text-amber-400 mb-3",
                "+ Nuevo combatiente manual"
            }
            div { class: "grid grid-cols-2 md:grid-cols-3 gap-2 mb-3",
                div { class: "col-span-2 md:col-span-3",
                    label { class: LBL, "Nombre *" }
                    {
                        let cls = if err() { INP_ERR } else { INP };
                        rsx! {
                            input {
                                class: "{cls}",
                                placeholder: "Goblin, Araña Gigante…",
                                value: "{name}",
                                oninput: move |e| { *name.write() = e.value(); *err.write() = false; },
                            }
                        }
                    }
                    if err() { p { class: "text-red-400 text-xs mt-0.5", "El nombre es obligatorio." } }
                }
                div {
                    label { class: LBL, "Tipo" }
                    select { class: INP, onchange: move |e| *kind.write() = e.value(),
                        option { value: "enemy",     "Enemigo" }
                        option { value: "player",    "PJ" }
                        option { value: "companion", "Compañero" }
                        option { value: "npc",       "PNJ" }
                    }
                }
                div {
                    label { class: LBL, "Cantidad" }
                    input { class: INP, r#type: "number", min: "1", max: "20",
                        value: "{count}", oninput: move |e| *count.write() = e.value() }
                }
                div {
                    label { class: LBL, "HP máx" }
                    input { class: INP, r#type: "number", min: "1",
                        value: "{hp}", oninput: move |e| *hp.write() = e.value() }
                }
                div {
                    label { class: LBL, "CA" }
                    input { class: INP, r#type: "number", min: "1",
                        value: "{ca}", oninput: move |e| *ca.write() = e.value() }
                }
                div {
                    label { class: LBL, "Bono Iniciativa" }
                    input { class: INP, r#type: "number",
                        value: "{init_b}", oninput: move |e| *init_b.write() = e.value() }
                }
            }
            button { class: BTN_AMBER, onclick: submit, "Añadir" }
        }
    }
}

// ---------------------------------------------------------------------------
// TemplateForm
// ---------------------------------------------------------------------------

#[component]
fn TemplateForm(on_done: EventHandler<()>) -> Element {
    let mut name  = use_signal(String::new);
    let mut cr    = use_signal(String::new);
    let mut ctype = use_signal(|| "humanoid".to_string());
    let mut hp    = use_signal(|| "30".to_string());
    let mut ca    = use_signal(|| "13".to_string());
    let mut str_v = use_signal(|| "10".to_string());
    let mut dex_v = use_signal(|| "10".to_string());
    let mut con_v = use_signal(|| "10".to_string());
    let mut int_v = use_signal(|| "10".to_string());
    let mut wis_v = use_signal(|| "10".to_string());
    let mut cha_v = use_signal(|| "10".to_string());
    let mut count = use_signal(|| "1".to_string());
    let mut err   = use_signal(|| false);
    let mut done  = use_signal(|| false);

    use_effect(move || {
        if done() {
            *done.write() = false;
            on_done.call(());
        }
    });

    let submit = move |_: Event<MouseData>| {
        let n = name().trim().to_string();
        if n.is_empty() { *err.write() = true; return; }

        let cr_s = cr().trim().to_string();
        let cnt: u32 = count().parse().unwrap_or(1).max(1).min(20);

        let template = EnemyTemplate {
            name: n, creature_type: ctype(),
            cr: if cr_s.is_empty() { None } else { Some(cr_s) },
            hp_max:       hp().parse().unwrap_or(30),
            armor_class:  ca().parse().unwrap_or(13),
            speed: 30,
            strength:     str_v().parse().unwrap_or(10),
            dexterity:    dex_v().parse().unwrap_or(10),
            constitution: con_v().parse().unwrap_or(10),
            intelligence: int_v().parse().unwrap_or(10),
            wisdom:       wis_v().parse().unwrap_or(10),
            charisma:     cha_v().parse().unwrap_or(10),
            description: String::new(), abilities: vec![],
            legendary_action_count: None, vault_path: None, tags: vec![],
        };

        spawn(async move {
            let _: Option<serde_json::Value> = post_json(
                "/combat/combatant/from-template",
                AddFromTemplateRequest { template, count: cnt },
            ).await;
            *done.write() = true;
        });
    };

    rsx! {
        div { class: "shrink-0 border-b border-stone-700 bg-stone-950 px-4 py-3",
            p { class: "text-xs font-semibold uppercase tracking-wider text-red-400 mb-3",
                "🐉 Plantilla de monstruo"
            }
            div { class: "grid grid-cols-2 md:grid-cols-4 gap-2 mb-2",
                div { class: "col-span-2",
                    label { class: LBL, "Nombre *" }
                    {
                        let cls = if err() { INP_ERR } else { INP };
                        rsx! {
                            input {
                                class: "{cls}",
                                placeholder: "Dragón Rojo Joven…",
                                value: "{name}",
                                oninput: move |e| { *name.write() = e.value(); *err.write() = false; }
                            }
                        }
                    }
                    if err() { p { class: "text-red-400 text-xs mt-0.5", "El nombre es obligatorio." } }
                }
                div {
                    label { class: LBL, "Tipo criatura" }
                    input { class: INP, placeholder: "dragon, undead…",
                        value: "{ctype}", oninput: move |e| *ctype.write() = e.value() }
                }
                div {
                    label { class: LBL, "CR" }
                    input { class: INP, placeholder: "1/2, 5…",
                        value: "{cr}", oninput: move |e| *cr.write() = e.value() }
                }
                div {
                    label { class: LBL, "HP máx" }
                    input { class: INP, r#type: "number",
                        value: "{hp}", oninput: move |e| *hp.write() = e.value() }
                }
                div {
                    label { class: LBL, "CA" }
                    input { class: INP, r#type: "number",
                        value: "{ca}", oninput: move |e| *ca.write() = e.value() }
                }
                div {
                    label { class: LBL, "Cantidad" }
                    input { class: INP, r#type: "number", min: "1", max: "20",
                        value: "{count}", oninput: move |e| *count.write() = e.value() }
                }
            }
            p { class: "text-xs text-stone-500 uppercase tracking-wider mb-1", "Atributos" }
            div { class: "grid grid-cols-6 gap-1 mb-3",
                AttrInput { label: "FUE", val: str_v }
                AttrInput { label: "DES", val: dex_v }
                AttrInput { label: "CON", val: con_v }
                AttrInput { label: "INT", val: int_v }
                AttrInput { label: "SAB", val: wis_v }
                AttrInput { label: "CAR", val: cha_v }
            }
            button { class: BTN_RED, onclick: submit, "Añadir monstruo" }
        }
    }
}

// ---------------------------------------------------------------------------
// CombatCard  — layout grid 3 columnas
// ---------------------------------------------------------------------------

#[component]
fn CombatCard(
    combatant: Combatant,
    is_active_turn: bool,
    is_selected: bool,
    on_select:    EventHandler<()>,
    on_remove:    EventHandler<()>,
    on_damage:    EventHandler<i32>,
    on_heal:      EventHandler<i32>,
    on_goto_turn: EventHandler<()>,
    on_condition: EventHandler<Condition>,
) -> Element {
    let mut dmg_val  = use_signal(String::new);
    let mut heal_val = use_signal(String::new);

    // ── estilos condicionales ─────────────────────────────────────────────
    let card_cls = if is_active_turn {
        "rounded-xl border-2 border-amber-400 bg-stone-800 shadow-lg shadow-amber-900/30 transition-all"
    } else if is_selected {
        "rounded-xl border-2 border-sky-500 bg-stone-850 shadow transition-all"
    } else {
        "rounded-xl border border-stone-700 bg-stone-900 hover:border-stone-500 transition-all"
    };

    let (kind_bg, kind_label) = match combatant.kind {
        CombatantKind::Player    => ("bg-sky-700 text-sky-100",    "PJ"),
        CombatantKind::Enemy     => ("bg-red-800 text-red-100",    "ENE"),
        CombatantKind::Companion => ("bg-purple-800 text-purple-100", "COMP"),
        CombatantKind::Npc       => ("bg-stone-600 text-stone-200",  "PNJ"),
    };

    let name_cls = if combatant.is_down() {
        "font-bold text-base line-through text-stone-500 truncate"
    } else if is_active_turn {
        "font-bold text-base text-amber-200 truncate"
    } else {
        "font-bold text-base text-stone-100 truncate"
    };

    let init_str = combatant.initiative
        .map(|i| format!("{:+}", i))
        .unwrap_or_else(|| "—".into());
    let hp_temp_str = if combatant.hp_temp > 0 {
        format!(" +{}tmp", combatant.hp_temp)
    } else {
        String::new()
    };
    let hp_pct = combatant.hp_percentage();
    let bar_col = combatant.hp_bar_color();
    let hp_text_cls = if combatant.is_down() {
        "text-stone-500 line-through"
    } else {
        "text-emerald-300"
    };

    // Borde izquierdo de color según tipo para identificación rápida
    let left_accent = match combatant.kind {
        CombatantKind::Player    => "#0ea5e9", // sky-500
        CombatantKind::Enemy     => "#ef4444", // red-500
        CombatantKind::Companion => "#a855f7", // purple-500
        CombatantKind::Npc       => "#78716c", // stone-500
    };
    let card_border = if is_active_turn {
        format!("border: 2px solid #fbbf24; border-left: 6px solid #fbbf24; background: #292524; border-radius: 12px; box-shadow: 0 0 20px rgba(251,191,36,0.15);")
    } else if is_selected {
        format!("border: 2px solid #38bdf8; border-left: 6px solid {}; background: #1c1917; border-radius: 12px;", left_accent)
    } else {
        format!("border: 1px solid #44403c; border-left: 6px solid {}; background: #1c1917; border-radius: 12px;", left_accent)
    };

    rsx! {
        div {
            style: "{card_border} cursor: pointer; user-select: none; overflow: hidden;",
            onclick: move |_| on_select.call(()),

            // ══ GRID 3 ZONAS con style inline ═════════════════════════════
            div { style: "display: grid; grid-template-columns: 1fr 2fr 1fr; min-height: 90px;",

                // ── Zona 1: Tipo + Nombre + Iniciativa ────────────────────
                div {
                    style: "padding: 12px 16px; border-right: 1px solid #44403c; display: flex; flex-direction: column; justify-content: center; gap: 6px;",
                    // Tipo badge
                    div { style: "display: flex; align-items: center; gap: 8px;",
                        if is_active_turn {
                            span { style: "color: #fbbf24; font-size: 14px; font-weight: bold;", "▶" }
                        }
                        span {
                            class: "{kind_bg}",
                            style: "font-size: 11px; font-weight: 700; padding: 2px 8px; border-radius: 4px; letter-spacing: 0.05em;",
                            "{kind_label}"
                        }
                    }
                    // Nombre
                    span {
                        class: "{name_cls}",
                        style: "font-size: 15px; font-weight: 700; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        "{combatant.name}"
                    }
                    // Iniciativa
                    div { style: "display: flex; align-items: center; gap: 6px; margin-top: 2px;",
                        span { style: "font-size: 11px; color: #78716c;", "Init" }
                        span { style: "font-size: 16px; font-weight: 900; font-family: monospace; color: #fbbf24;", "{init_str}" }
                    }
                }

                // ── Zona 2: HP grande + barra + condiciones ───────────────
                div {
                    style: "padding: 12px 16px; border-right: 1px solid #44403c; display: flex; flex-direction: column; justify-content: center; gap: 8px;",

                    // HP
                    div { style: "display: flex; align-items: baseline; gap: 4px;",
                        span {
                            class: "{hp_text_cls}",
                            style: "font-size: 28px; font-weight: 900; font-family: monospace; line-height: 1;",
                            "{combatant.hp_current}"
                        }
                        span { style: "color: #57534e; font-size: 16px;", "/" }
                        span { style: "color: #a8a29e; font-size: 16px; font-family: monospace;", "{combatant.hp_max}" }
                        if combatant.hp_temp > 0 {
                            span { style: "color: #38bdf8; font-size: 12px; font-family: monospace; margin-left: 4px;", "{hp_temp_str}" }
                        }
                        span { style: "color: #44403c; font-size: 11px; margin-left: auto;", "PV" }
                    }
                    // Barra HP
                    div { style: "height: 8px; background: #44403c; border-radius: 9999px; overflow: hidden;",
                        div {
                            class: "{bar_col}",
                            style: "height: 100%; border-radius: 9999px; width: {hp_pct:.1}%; transition: width 0.4s ease;"
                        }
                    }
                    // Condiciones
                    div { style: "display: flex; flex-wrap: wrap; gap: 4px; min-height: 20px;",
                        if combatant.conditions.is_empty() {
                            span { style: "font-size: 11px; color: #44403c; font-style: italic;", "Sin condiciones" }
                        }
                        for cond in combatant.conditions.clone() {
                            {
                                let label = cond.label();
                                let color = cond.color_class();
                                let c2 = cond.clone();
                                rsx! {
                                    span {
                                        class: "{color}",
                                        style: "font-size: 11px; padding: 2px 6px; border-radius: 9999px; color: white; cursor: pointer;",
                                        title: "Click para quitar",
                                        onclick: move |e: Event<MouseData>| {
                                            e.stop_propagation();
                                            on_condition.call(c2.clone());
                                        },
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Zona 3: CA + Controles ────────────────────────────────
                div {
                    style: "padding: 12px 12px; display: flex; flex-direction: column; justify-content: space-between; gap: 8px;",
                    onclick: move |e| e.stop_propagation(),

                    // CA + botones turno/eliminar
                    div { style: "display: flex; align-items: center; justify-content: space-between;",
                        div { style: "text-align: center;",
                            div { style: "font-size: 11px; color: #78716c;", "CA" }
                            div { style: "font-size: 24px; font-weight: 900; font-family: monospace; color: #7dd3fc;",
                                "{combatant.armor_class}"
                            }
                        }
                        div { style: "display: flex; flex-direction: column; gap: 4px;",
                            button {
                                style: "padding: 4px 8px; border-radius: 6px; background: rgba(120,53,15,0.7); color: #fde68a; font-size: 11px; border: none; cursor: pointer;",
                                onclick: move |_| on_goto_turn.call(()),
                                "▶ Turno"
                            }
                            button {
                                style: "padding: 4px 8px; border-radius: 6px; background: rgba(68,64,60,0.7); color: #a8a29e; font-size: 11px; border: none; cursor: pointer;",
                                onclick: move |_| on_remove.call(()),
                                "✕ Quitar"
                            }
                        }
                    }

                    // Inputs daño + curación
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        // Daño
                        div { style: "display: flex; align-items: center; gap: 4px;",
                            input {
                                style: "width: 50px; text-align: center; font-size: 13px; background: #292524; border: 1px solid #44403c; border-radius: 6px; padding: 4px; font-family: monospace; color: #f5f5f4; outline: none;",
                                r#type: "number", min: "1", placeholder: "dmg",
                                value: "{dmg_val}",
                                oninput: move |e| *dmg_val.write() = e.value(),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        let v: i32 = dmg_val().parse().unwrap_or(0);
                                        if v > 0 { on_damage.call(v); *dmg_val.write() = String::new(); }
                                    }
                                }
                            }
                            button {
                                style: "flex: 1; padding: 5px 6px; border-radius: 6px; background: #7f1d1d; color: #fecaca; font-size: 11px; font-weight: 700; border: none; cursor: pointer;",
                                onclick: move |_| {
                                    let v: i32 = dmg_val().parse().unwrap_or(0);
                                    if v > 0 { on_damage.call(v); *dmg_val.write() = String::new(); }
                                },
                                "💢 Daño"
                            }
                        }
                        // Curación
                        div { style: "display: flex; align-items: center; gap: 4px;",
                            input {
                                style: "width: 50px; text-align: center; font-size: 13px; background: #292524; border: 1px solid #44403c; border-radius: 6px; padding: 4px; font-family: monospace; color: #f5f5f4; outline: none;",
                                r#type: "number", min: "1", placeholder: "cur",
                                value: "{heal_val}",
                                oninput: move |e| *heal_val.write() = e.value(),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        let v: i32 = heal_val().parse().unwrap_or(0);
                                        if v > 0 { on_heal.call(v); *heal_val.write() = String::new(); }
                                    }
                                }
                            }
                            button {
                                style: "flex: 1; padding: 5px 6px; border-radius: 6px; background: #14532d; color: #bbf7d0; font-size: 11px; font-weight: 700; border: none; cursor: pointer;",
                                onclick: move |_| {
                                    let v: i32 = heal_val().parse().unwrap_or(0);
                                    if v > 0 { on_heal.call(v); *heal_val.write() = String::new(); }
                                },
                                "💚 Curar"
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// DetailPanel
// ---------------------------------------------------------------------------

#[component]
fn DetailPanel(
    combatant: Combatant,
    on_condition: EventHandler<Condition>,
    on_close:     EventHandler<()>,
) -> Element {
    let all_conds = Condition::all_standard();
    rsx! {
        div { class: "w-72 bg-stone-900 border-l border-stone-800 flex flex-col shrink-0",
            div { class: "flex items-center justify-between px-4 py-3 border-b border-stone-800 bg-stone-950 shrink-0",
                span { class: "font-semibold text-amber-300 truncate text-sm", "{combatant.name}" }
                button {
                    class: "text-stone-500 hover:text-stone-200 text-lg leading-none ml-2 shrink-0",
                    onclick: move |_| on_close.call(()), "✕"
                }
            }
            div { class: "flex-1 overflow-y-auto p-4 space-y-4",
                div { class: "grid grid-cols-3 gap-2",
                    StatBox { label: "HP",   value: format!("{}/{}", combatant.hp_current, combatant.hp_max) }
                    StatBox { label: "CA",   value: combatant.armor_class.to_string() }
                    StatBox { label: "Init", value: combatant.initiative.map(|i| format!("{:+}", i)).unwrap_or_else(|| "—".into()) }
                }
                if combatant.hp_temp > 0 {
                    div { class: "text-center text-xs text-sky-300 bg-sky-900/30 rounded p-1.5 border border-sky-800",
                        "⚡ PV temporales: +{combatant.hp_temp}"
                    }
                }
                if combatant.strength.is_some() {
                    div {
                        p { class: "text-xs text-stone-500 uppercase tracking-wider font-semibold mb-2", "Atributos" }
                        div { class: "grid grid-cols-3 gap-1",
                            AttrBox { abbr: "FUE", value: combatant.strength.unwrap_or(10) }
                            AttrBox { abbr: "DES", value: combatant.dexterity.unwrap_or(10) }
                            AttrBox { abbr: "CON", value: combatant.constitution.unwrap_or(10) }
                            AttrBox { abbr: "INT", value: combatant.intelligence.unwrap_or(10) }
                            AttrBox { abbr: "SAB", value: combatant.wisdom.unwrap_or(10) }
                            AttrBox { abbr: "CAR", value: combatant.charisma.unwrap_or(10) }
                        }
                    }
                }
                if !combatant.abilities.is_empty() {
                    div {
                        p { class: "text-xs text-stone-500 uppercase tracking-wider font-semibold mb-2", "Habilidades" }
                        div { class: "space-y-2",
                            for ability in combatant.abilities.clone() {
                                div { class: "text-xs bg-stone-800 rounded p-2 border border-stone-700",
                                    div { class: "flex items-center gap-2 mb-1",
                                        span { class: "font-semibold text-amber-300", "{ability.name}" }
                                        span { class: "text-stone-500 capitalize text-xs", "({ability.action_cost})" }
                                        if let Some(dmg) = ability.damage_roll {
                                            span { class: "ml-auto font-mono text-red-300", "🗡 {dmg}" }
                                        }
                                    }
                                    p { class: "text-stone-400 leading-tight", "{ability.description}" }
                                    if let Some(dc) = ability.save_dc {
                                        p { class: "text-orange-400 mt-0.5", "CD: {dc}" }
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(spell) = combatant.concentrating_on.clone() {
                    div { class: "bg-purple-900/40 border border-purple-700 rounded px-3 py-2 text-xs",
                        span { class: "text-purple-300 font-semibold", "✨ Concentración: " }
                        span { class: "text-purple-200", "{spell}" }
                    }
                }
                if !combatant.notes.is_empty() {
                    div { class: "text-xs text-stone-400 bg-stone-800 rounded p-2 italic border border-stone-700",
                        "{combatant.notes}"
                    }
                }
                div {
                    p { class: "text-xs text-stone-500 uppercase tracking-wider font-semibold mb-2", "Condiciones" }
                    div { class: "flex flex-wrap gap-1",
                        for cond in all_conds {
                            {
                                let active = combatant.conditions.contains(&cond);
                                let bg = if active { cond.color_class() } else { "bg-stone-800 border border-stone-700" };
                                let label = cond.label();
                                let c2 = cond.clone();
                                rsx! {
                                    button {
                                        class: "text-xs px-2 py-0.5 rounded-full {bg} text-white transition hover:opacity-80",
                                        onclick: move |_| on_condition.call(c2.clone()),
                                        "{label}"
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

// ---------------------------------------------------------------------------
// Micro-componentes
// ---------------------------------------------------------------------------

#[component]
fn PanelBtn(
    active: bool,
    label_off: &'static str,
    label_on:  &'static str,
    onclick: EventHandler<()>,
) -> Element {
    let cls = if active { BTN_ACTIVE } else { BTN_STONE };
    rsx! {
        button { class: "{cls}", onclick: move |_| onclick.call(()),
            if active { "{label_on}" } else { "{label_off}" }
        }
    }
}

#[component]
fn StatBox(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "bg-stone-800 rounded p-2 text-center",
            p { class: "text-xs text-stone-500 mb-0.5", "{label}" }
            p { class: "font-mono font-bold text-stone-100 text-sm", "{value}" }
        }
    }
}

#[component]
fn AttrBox(abbr: &'static str, value: i32) -> Element {
    let m  = Combatant::modifier(value);
    let ms = if m >= 0 { format!("+{}", m) } else { m.to_string() };
    rsx! {
        div { class: "bg-stone-800 rounded p-1.5 text-center",
            p { class: "text-stone-500 text-xs", "{abbr}" }
            p { class: "font-bold text-stone-100 text-xs", "{value}" }
            p { class: "text-stone-400 text-xs", "{ms}" }
        }
    }
}

#[component]
fn AttrInput(label: &'static str, val: Signal<String>) -> Element {
    rsx! {
        div {
            label { class: "block text-xs text-stone-500 mb-0.5 text-center", "{label}" }
            input {
                class: "w-full text-center text-xs bg-stone-800 border border-stone-700 rounded px-1 py-1 font-mono text-stone-100 outline-none focus:border-amber-500",
                r#type: "number", value: "{val}",
                oninput: move |e| *val.write() = e.value(),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Estilos
// ---------------------------------------------------------------------------

const BTN_GREEN:    &str = "px-3 py-1.5 rounded bg-emerald-700 hover:bg-emerald-600 text-sm font-semibold transition text-white";
const BTN_BLUE:     &str = "px-4 py-1.5 rounded bg-sky-700 hover:bg-sky-600 text-sm font-semibold transition text-white";
const BTN_AMBER:    &str = "px-4 py-1.5 rounded bg-amber-700 hover:bg-amber-600 text-sm font-semibold transition text-white";
const BTN_RED:      &str = "px-4 py-1.5 rounded bg-red-900 hover:bg-red-800 text-sm font-semibold transition text-white";
const BTN_STONE:    &str = "px-3 py-1.5 rounded bg-stone-700 hover:bg-stone-600 text-sm transition text-stone-200";
const BTN_ACTIVE:   &str = "px-3 py-1.5 rounded bg-stone-600 border border-stone-400 text-sm font-semibold transition text-white";
const BTN_DISABLED: &str = "px-4 py-1.5 rounded bg-stone-700 text-stone-500 text-sm font-semibold cursor-not-allowed";

const LBL:     &str = "block text-xs text-stone-400 mb-1";
const INP:     &str = "w-full bg-stone-800 border border-stone-700 rounded px-2 py-1.5 text-sm text-stone-100 outline-none focus:border-amber-500 transition";
const INP_ERR: &str = "w-full bg-stone-800 border border-red-500 rounded px-2 py-1.5 text-sm text-stone-100 outline-none focus:border-red-400 transition";
