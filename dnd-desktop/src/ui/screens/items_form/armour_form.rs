// Formulario de creación de ARMADURAS.
// Campos DnD 5.5e: CA base, tipo de armadura, fuerza mínima, penalización sigilo.

use super::weapon_form::{FormField, SaveButton, INPUT_STYLE};
use super::NewItemData;
use dioxus::prelude::*;

#[component]
pub fn ArmourForm(on_save: EventHandler<NewItemData>) -> Element {
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut weight = use_signal(String::new);
    let mut rarity = use_signal(|| "common".to_string());
    let mut source = use_signal(|| "PHB2024".to_string());
    let mut notes = use_signal(String::new);

    // Campos específicos de armadura
    let mut armour_category = use_signal(|| "light".to_string()); // light|medium|heavy|shield
    let mut base_ac = use_signal(String::new); // ej. "14"
    let mut dex_cap = use_signal(String::new); // ej. "2" (medium), "" (light=full), "0" (heavy)
    let mut str_req = use_signal(String::new); // Fuerza mínima
    let mut stealth_disadv = use_signal(|| false); // Desventaja sigilo
    let mut ac_description = use_signal(String::new); // ej. "14 + DES (max 2)"

    let rarities = ["common", "uncommon", "rare", "very_rare", "legendary"];

    // Auto-fill del campo ac_description según tipo
    let mut update_ac_desc = move || {
        let base = base_ac.read().clone();
        let dex = dex_cap.read().clone();
        let desc = match armour_category.read().as_str() {
            "light" => format!("{base} + DES"),
            "medium" => {
                let cap = if dex.is_empty() {
                    "2".to_string()
                } else {
                    dex.clone()
                };
                format!("{base} + DES (máx {cap})")
            }
            "heavy" => base.clone(),
            "shield" => format!("+{base}"),
            _ => base.clone(),
        };
        ac_description.set(desc);
    };

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",

            // Nombre
            FormField { label: "Nombre *",
                input {
                    style: INPUT_STYLE,
                    placeholder: "Cota de malla…",
                    value: "{name}",
                    oninput: move |e| name.set(e.value()),
                }
            }

            // Categoría de armadura + Rareza
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Categoría",
                    select {
                        style: INPUT_STYLE,
                        onchange: move |e| { armour_category.set(e.value()); update_ac_desc(); },
                        option { value: "light",  "Ligera" }
                        option { value: "medium", "Media" }
                        option { value: "heavy",  "Pesada" }
                        option { value: "shield", "Escudo" }
                    }
                }
                FormField { label: "Rareza",
                    select {
                        style: INPUT_STYLE,
                        onchange: move |e| rarity.set(e.value()),
                        for r in rarities {
                            option { value: "{r}", "{r}" }
                        }
                    }
                }
            }

            // CA base + CAP de DES
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "CA base",
                    input {
                        style: INPUT_STYLE,
                        r#type: "number",
                        placeholder: "14",
                        value: "{base_ac}",
                        oninput: move |e| { base_ac.set(e.value()); update_ac_desc(); },
                    }
                }
                FormField { label: "Límite DES (vacío = sin límite)",
                    input {
                        style: INPUT_STYLE,
                        r#type: "number",
                        placeholder: "2  (media) / 0 (pesada)",
                        value: "{dex_cap}",
                        oninput: move |e| { dex_cap.set(e.value()); update_ac_desc(); },
                    }
                }
            }

            // Descripción CA generada
            if !ac_description.read().is_empty() {
                div { style: "font-size:0.7rem; color:#34d399; padding:4px 0;",
                    "CA calculada: {ac_description}"
                }
            }

            // Fuerza mínima + Penalización sigilo
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Fuerza mínima",
                    input {
                        style: INPUT_STYLE,
                        r#type: "number",
                        placeholder: "15",
                        value: "{str_req}",
                        oninput: move |e| str_req.set(e.value()),
                    }
                }
                FormField { label: "Penalización sigilo",
                    div { style: "display:flex; align-items:center; gap:8px; padding:9px 0;",
                        input {
                            r#type: "checkbox",
                            checked: "{stealth_disadv}",
                            onchange: move |e| stealth_disadv.set(e.checked()),
                        }
                        span { style: "font-size:0.78rem; color:#a8a29e;", "Desventaja en Sigilo" }
                    }
                }
            }

            // Peso + Fuente
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Peso (lb)",
                    input {
                        style: INPUT_STYLE,
                        r#type: "number",
                        placeholder: "20",
                        value: "{weight}",
                        oninput: move |e| weight.set(e.value()),
                    }
                }
                FormField { label: "Fuente",
                    input {
                        style: INPUT_STYLE,
                        placeholder: "PHB2024",
                        value: "{source}",
                        oninput: move |e| source.set(e.value()),
                    }
                }
            }

            // Descripción
            FormField { label: "Descripción",
                textarea {
                    style: "{INPUT_STYLE} resize:vertical;",
                    rows: "3",
                    placeholder: "Descripción y lore…",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
            }

            // Notas
            FormField { label: "Notas",
                textarea {
                    style: "{INPUT_STYLE} resize:vertical;",
                    rows: "2",
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            SaveButton {
                onclick: move |_| {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("armour_category".into(), armour_category.read().clone());
                    if !base_ac.read().is_empty() {
                        extra.insert("base_ac".into(), base_ac.read().clone());
                    }
                    if !ac_description.read().is_empty() {
                        extra.insert("ac_formula".into(), ac_description.read().clone());
                    }
                    if !str_req.read().is_empty() {
                        extra.insert("str_requirement".into(), str_req.read().clone());
                    }
                    if *stealth_disadv.read() {
                        extra.insert("stealth_disadvantage".into(), "true".into());
                    }
                    on_save.call(NewItemData {
                        name:        name.read().clone(),
                        description: description.read().clone(),
                        weight:      weight.read().parse::<f32>().ok(),
                        rarity:      rarity.read().clone(),
                        source:      source.read().clone(),
                        notes:       notes.read().clone(),
                        tags:        vec!["armour".into(), armour_category.read().clone()],
                        extra,
                    });
                }
            }
        }
    }
}
