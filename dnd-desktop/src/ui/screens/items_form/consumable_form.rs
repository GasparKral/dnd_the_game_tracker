use super::weapon_form::{FormField, SaveButton, INPUT_STYLE};
use super::NewItemData;
use dioxus::prelude::*;

#[component]
pub fn ConsumableForm(on_save: EventHandler<NewItemData>) -> Element {
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut weight = use_signal(String::new);
    let mut rarity = use_signal(|| "common".to_string());
    let mut source = use_signal(|| "PHB2024".to_string());
    let mut notes = use_signal(String::new);
    let mut effect = use_signal(String::new); // ej. "Cura 2d4+2 PG"
    let mut duration = use_signal(String::new); // ej. "1 hora"
    let mut consumable_type = use_signal(|| "potion".to_string()); // potion|scroll|food|poison|other

    let rarities = ["common", "uncommon", "rare", "very_rare", "legendary"];

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",
            FormField { label: "Nombre *",
                input { style: INPUT_STYLE, placeholder: "Poción de curación…",
                    value: "{name}", oninput: move |e| name.set(e.value()) }
            }
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Subtipo",
                    select { style: INPUT_STYLE, onchange: move |e| consumable_type.set(e.value()),
                        option { value: "potion",  "Poción" }
                        option { value: "scroll",  "Pergamino" }
                        option { value: "food",    "Comida / Bebida" }
                        option { value: "poison",  "Veneno" }
                        option { value: "other",   "Otro" }
                    }
                }
                FormField { label: "Rareza",
                    select { style: INPUT_STYLE, onchange: move |e| rarity.set(e.value()),
                        for r in rarities { option { value: "{r}", "{r}" } }
                    }
                }
            }
            FormField { label: "Efecto",
                input { style: INPUT_STYLE, placeholder: "Cura 2d4+2 PG al beberla…",
                    value: "{effect}", oninput: move |e| effect.set(e.value()) }
            }
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Duración del efecto",
                    input { style: INPUT_STYLE, placeholder: "1 hora",
                        value: "{duration}", oninput: move |e| duration.set(e.value()) }
                }
                FormField { label: "Peso (lb)",
                    input { style: INPUT_STYLE, r#type: "number", placeholder: "0.5",
                        value: "{weight}", oninput: move |e| weight.set(e.value()) }
                }
            }
            FormField { label: "Fuente",
                input { style: INPUT_STYLE, placeholder: "PHB2024",
                    value: "{source}", oninput: move |e| source.set(e.value()) }
            }
            FormField { label: "Descripción",
                textarea { style: "{INPUT_STYLE} resize:vertical;", rows: "3",
                    value: "{description}", oninput: move |e| description.set(e.value()) }
            }
            FormField { label: "Notas",
                textarea { style: "{INPUT_STYLE} resize:vertical;", rows: "2",
                    value: "{notes}", oninput: move |e| notes.set(e.value()) }
            }
            SaveButton {
                onclick: move |_| {
                    let mut extra = std::collections::HashMap::new();
                    extra.insert("consumable_type".into(), consumable_type.read().clone());
                    if !effect.read().is_empty()   { extra.insert("effect".into(), effect.read().clone()); }
                    if !duration.read().is_empty() { extra.insert("duration".into(), duration.read().clone()); }
                    on_save.call(NewItemData {
                        name: name.read().clone(), description: description.read().clone(),
                        weight: weight.read().parse::<f32>().ok(), rarity: rarity.read().clone(),
                        source: source.read().clone(), notes: notes.read().clone(),
                        tags: vec!["consumable".into(), consumable_type.read().clone()],
                        extra,
                    });
                }
            }
        }
    }
}
