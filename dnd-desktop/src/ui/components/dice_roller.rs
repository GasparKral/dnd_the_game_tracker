// ═══════════════════════════════════════════════════════════════════════════
// dice_roller.rs — Componente reutilizable de tirada de dados para el DM
// ═══════════════════════════════════════════════════════════════════════════

use crate::states::SharedState;
use dioxus::prelude::*;
use shared::models::dice::{Dice, DiceRoll, RollMode, RollRequest, RollResult};

// ─── Estado interno del selector de dados ────────────────────────────────────

#[derive(Clone, PartialEq, Default)]
struct DiceSelection {
    d4:   u8,
    d6:   u8,
    d8:   u8,
    d10:  u8,
    d12:  u8,
    d20:  u8,
    d100: u8,
}

impl DiceSelection {
    fn is_empty(&self) -> bool {
        self.d4 == 0
            && self.d6 == 0
            && self.d8 == 0
            && self.d10 == 0
            && self.d12 == 0
            && self.d20 == 0
            && self.d100 == 0
    }

    fn has_d20(&self) -> bool { self.d20 > 0 }

    fn to_rolls(&self) -> Vec<DiceRoll> {
        let mut rolls = Vec::new();
        let pairs = [
            (self.d4,   Dice::D4),
            (self.d6,   Dice::D6),
            (self.d8,   Dice::D8),
            (self.d10,  Dice::D10),
            (self.d12,  Dice::D12),
            (self.d20,  Dice::D20),
            (self.d100, Dice::D100),
        ];
        for (count, dice) in pairs {
            if count > 0 { rolls.push(DiceRoll { count, dice }); }
        }
        rolls
    }
}

// ─── Componente principal ────────────────────────────────────────────────────

#[component]
pub fn DiceRoller() -> Element {
    let state = consume_context::<SharedState>().0;

    let mut selection    = use_signal(DiceSelection::default);
    let mut modifier     = use_signal(|| 0i32);
    let mut mode         = use_signal(|| RollMode::Normal);
    let mut label        = use_signal(String::new);
    let mut roll_history = use_signal(Vec::<RollResult>::new);
    let mut rolling      = use_signal(|| false);
    let mut error_msg    = use_signal(|| Option::<String>::None);

    // Cuando no hay d20, forzar modo Normal
    use_effect(move || {
        if !selection.read().has_d20() {
            mode.set(RollMode::Normal);
        }
    });

    let on_roll = move |_| {
        let sel = selection.read().clone();
        if sel.is_empty() {
            error_msg.set(Some("Selecciona al menos un dado".to_string()));
            return;
        }
        error_msg.set(None);
        rolling.set(true);

        let request = RollRequest {
            rolls:    sel.to_rolls(),
            modifier: *modifier.read(),
            mode:     *mode.read(),
            label: {
                let l = label.read().trim().to_string();
                if l.is_empty() { None } else { Some(l) }
            },
        };

        let st = state.clone();
        spawn(async move {
            let client = reqwest::Client::new();
            match client
                .post("http://localhost:3000/api/roll")
                .json(&request)
                .send()
                .await
            {
                Ok(resp) => match resp.json::<RollResult>().await {
                    Ok(result) => {
                        roll_history.write().insert(0, result);
                        if roll_history.read().len() > 10 {
                            roll_history.write().truncate(10);
                        }
                    }
                    Err(e) => error_msg.set(Some(format!("Error parseando respuesta: {e}"))),
                },
                Err(e) => error_msg.set(Some(format!("Error de red: {e}"))),
            }
            rolling.set(false);
            let _ = st;
        });
    };

    // ── Valores derivados para el botón (evitar if en string) ───────────────
    let btn_bg     = if *rolling.read() { "#292524" } else { "#b45309" };
    let btn_color  = if *rolling.read() { "#78716c" } else { "#fef3c7" };
    let btn_cursor = if *rolling.read() { "not-allowed" } else { "pointer" };

    rsx! {
        div {
            style: "background:#1c1917; border:1px solid #44403c; border-radius:16px;
                    padding:18px; display:flex; flex-direction:column; gap:14px; min-width:320px;",

            p { style: "font-size:0.85rem; font-weight:700; color:#d6d3d1;
                         text-transform:uppercase; letter-spacing:0.08em; margin:0;",
                "🎲 Tirar dados"
            }

            DiceSelector { selection }

            ModifierInput { modifier }

            if selection.read().has_d20() {
                ModeSelector { mode }
            }

            input {
                r#type: "text",
                placeholder: "Etiqueta (ej: Ataque, Percepción…)",
                value: "{label}",
                oninput: move |e| label.set(e.value()),
                style: "background:#111110; border:1px solid #44403c; border-radius:8px;
                         color:#d6d3d1; padding:6px 10px; font-size:0.75rem; outline:none;
                         width:100%; box-sizing:border-box;",
            }

            if let Some(err) = error_msg.read().as_ref() {
                p { style: "color:#ef4444; font-size:0.72rem; margin:0;", "{err}" }
            }

            button {
                onclick: on_roll,
                disabled: *rolling.read(),
                style: "background:{btn_bg}; color:{btn_color};
                         border:none; border-radius:10px; padding:9px 0; font-size:0.85rem;
                         font-weight:700; cursor:{btn_cursor};
                         width:100%; letter-spacing:0.04em;",
                if *rolling.read() { "Tirando…" } else { "⚡ Tirar" }
            }

            if !roll_history.read().is_empty() {
                RollHistory { history: roll_history.read().clone() }
            }
        }
    }
}

// ─── DiceSelector ────────────────────────────────────────────────────────────

#[component]
fn DiceSelector(mut selection: Signal<DiceSelection>) -> Element {
    let dice_list: &[(u8, &str, fn(&DiceSelection) -> u8, fn(&mut DiceSelection, u8))] = &[
        (4,   "d4",  |s| s.d4,   |s, v| s.d4 = v),
        (6,   "d6",  |s| s.d6,   |s, v| s.d6 = v),
        (8,   "d8",  |s| s.d8,   |s, v| s.d8 = v),
        (10,  "d10", |s| s.d10,  |s, v| s.d10 = v),
        (12,  "d12", |s| s.d12,  |s, v| s.d12 = v),
        (20,  "d20", |s| s.d20,  |s, v| s.d20 = v),
        (100, "d%",  |s| s.d100, |s, v| s.d100 = v),
    ];

    rsx! {
        div { style: "display:flex; flex-wrap:wrap; gap:6px;",
            for (_, lbl, getter, setter) in dice_list.iter().copied() {
                {
                    let count = getter(&selection.read());
                    rsx! {
                        DiceChip {
                            label: lbl,
                            count,
                            on_inc: move |_| {
                                let cur = getter(&selection.read());
                                if cur < 10 {
                                    let mut s = selection.write();
                                    setter(&mut s, cur + 1);
                                }
                            },
                            on_dec: move |_| {
                                let cur = getter(&selection.read());
                                if cur > 0 {
                                    let mut s = selection.write();
                                    setter(&mut s, cur - 1);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DiceChip(
    label:  &'static str,
    count:  u8,
    on_inc: EventHandler<MouseEvent>,
    on_dec: EventHandler<MouseEvent>,
) -> Element {
    let active = count > 0;
    let bg  = if active { "#292524" } else { "#111110" };
    let col = if active { "#fbbf24" } else { "#78716c" };
    let brd = if active { "#b45309" } else { "#44403c" };

    rsx! {
        div {
            style: "display:flex; align-items:center; gap:3px; border:1px solid {brd};
                     border-radius:8px; padding:3px 6px; background:{bg};",
            button {
                onclick: move |e| on_dec.call(e),
                style: "background:none; border:none; color:{col}; cursor:pointer;
                         font-size:0.8rem; padding:0 2px; line-height:1;",
                "−"
            }
            span {
                style: "font-size:0.75rem; font-weight:700; color:{col};
                         min-width:28px; text-align:center;",
                if count > 0 { "{count}{label}" } else { "{label}" }
            }
            button {
                onclick: move |e| on_inc.call(e),
                style: "background:none; border:none; color:{col}; cursor:pointer;
                         font-size:0.8rem; padding:0 2px; line-height:1;",
                "+"
            }
        }
    }
}

// ─── ModifierInput ───────────────────────────────────────────────────────────

#[component]
fn ModifierInput(mut modifier: Signal<i32>) -> Element {
    let mod_val = *modifier.read();
    let mod_col = if mod_val >= 0 { "#34d399" } else { "#f87171" };
    let mod_str = if mod_val >= 0 {
        format!("+{mod_val}")
    } else {
        format!("{mod_val}")
    };

    rsx! {
        div { style: "display:flex; align-items:center; gap:8px;",
            span { style: "font-size:0.72rem; color:#78716c; white-space:nowrap;",
                "Modificador"
            }
            button {
                onclick: move |_| { *modifier.write() -= 1; },
                style: "background:#111110; border:1px solid #44403c; border-radius:6px;
                         color:#d6d3d1; width:26px; height:26px; cursor:pointer; font-size:0.9rem;",
                "−"
            }
            span {
                style: "font-size:1rem; font-weight:700; color:{mod_col};
                         min-width:32px; text-align:center;",
                "{mod_str}"
            }
            button {
                onclick: move |_| { *modifier.write() += 1; },
                style: "background:#111110; border:1px solid #44403c; border-radius:6px;
                         color:#d6d3d1; width:26px; height:26px; cursor:pointer; font-size:0.9rem;",
                "+"
            }
            button {
                onclick: move |_| modifier.set(0),
                style: "background:none; border:none; color:#57534e; cursor:pointer;
                         font-size:0.65rem; padding:0;",
                "reset"
            }
        }
    }
}

// ─── ModeSelector ────────────────────────────────────────────────────────────

#[component]
fn ModeSelector(mut mode: Signal<RollMode>) -> Element {
    let modes = [
        (RollMode::Disadvantage, "Desventaja", "#ef4444"),
        (RollMode::Normal,       "Normal",     "#a8a29e"),
        (RollMode::Advantage,    "Ventaja",    "#34d399"),
    ];

    rsx! {
        div { style: "display:flex; gap:6px; align-items:center;",
            span { style: "font-size:0.72rem; color:#78716c;", "Modo" }
            for (m, lbl, col) in modes {
                {
                    let is_active = *mode.read() == m;
                    let bg       = if is_active { "#292524" } else { "#111110" };
                    let brd      = if is_active { col }       else { "#44403c" };
                    let weight   = if is_active { "700" }     else { "400" };
                    rsx! {
                        button {
                            onclick: move |_| mode.set(m),
                            style: "background:{bg}; border:1px solid {brd}; border-radius:8px;
                                     color:{col}; font-size:0.72rem; padding:4px 10px; cursor:pointer;
                                     font-weight:{weight};",
                            "{lbl}"
                        }
                    }
                }
            }
        }
    }
}

// ─── RollHistory ─────────────────────────────────────────────────────────────

#[component]
fn RollHistory(history: Vec<RollResult>) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:5px;",
            p { style: "font-size:0.65rem; color:#57534e; text-transform:uppercase;
                         letter-spacing:0.07em; margin:0;",
                "Historial reciente"
            }
            // Fix: RollHistoryEntry es un componente con prop `result`,
            // se llama con la sintaxis de componente normal de Dioxus.
            for result in history.into_iter() {
                RollHistoryEntry { result }
            }
        }
    }
}

#[component]
fn RollHistoryEntry(result: RollResult) -> Element {
    let dice_desc: String = result
        .request
        .rolls
        .iter()
        .map(|dr| dr.to_string())
        .collect::<Vec<_>>()
        .join(" + ");

    let entry_label = result
        .request
        .label
        .clone()
        .unwrap_or_else(|| dice_desc.clone());

    let modifier = result.request.modifier;
    let modifier_str = if modifier == 0 {
        String::new()
    } else if modifier > 0 {
        format!(" +{modifier}")
    } else {
        format!(" {modifier}")
    };

    let mode_badge: Option<(&str, &str)> = match result.request.mode {
        RollMode::Advantage    => Some(("V", "#34d399")),
        RollMode::Disadvantage => Some(("D", "#ef4444")),
        RollMode::Normal       => None,
    };

    let total_col = if result.total >= 18 {
        "#fbbf24"
    } else if result.total <= 2 {
        "#ef4444"
    } else {
        "#d6d3d1"
    };

    let vals_str: String = result
        .individual_rolls
        .iter()
        .flat_map(|v| v.iter().map(|x| x.to_string()))
        .collect::<Vec<_>>()
        .join(", ");

    let total = result.total;

    rsx! {
        div {
            style: "display:flex; justify-content:space-between; align-items:center;
                     background:#111110; border:1px solid #292524; border-radius:8px;
                     padding:5px 10px;",

            div { style: "display:flex; flex-direction:column; gap:1px;",
                div { style: "display:flex; align-items:center; gap:5px;",
                    span { style: "font-size:0.72rem; color:#a8a29e;", "{entry_label}" }
                    if let Some((badge, col)) = mode_badge {
                        span {
                            style: "font-size:0.6rem; color:{col}; border:1px solid {col};
                                     border-radius:4px; padding:0 3px;",
                            "{badge}"
                        }
                    }
                }
                span { style: "font-size:0.6rem; color:#57534e;",
                    "{dice_desc}{modifier_str}"
                }
                span { style: "font-size:0.6rem; color:#44403c;",
                    "[{vals_str}]"
                }
                if let Some(discarded) = result.discarded_d20 {
                    span { style: "font-size:0.6rem; color:#57534e;",
                        "descartado: {discarded}"
                    }
                }
            }

            span {
                style: "font-size:1.4rem; font-weight:800; color:{total_col};",
                "{total}"
            }
        }
    }
}
