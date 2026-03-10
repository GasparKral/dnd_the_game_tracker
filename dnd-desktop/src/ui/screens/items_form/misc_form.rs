// Formulario genérico para objetos que no encajan en otro tipo.

use super::weapon_form::{FormField, SaveButton, INPUT_STYLE};
use super::NewItemData;
use dioxus::prelude::*;

#[component]
pub fn MiscForm(on_save: EventHandler<NewItemData>) -> Element {
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut weight = use_signal(String::new);
    let mut rarity = use_signal(|| "common".to_string());
    let mut source = use_signal(|| "Homebrew".to_string());
    let mut notes = use_signal(String::new);
    let mut tags_raw = use_signal(String::new); // tags libres separadas por coma

    let rarities = [
        "common",
        "uncommon",
        "rare",
        "very_rare",
        "legendary",
        "artifact",
    ];

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",
            FormField { label: "Nombre *",
                input { style: INPUT_STYLE, placeholder: "Nombre del objeto…",
                    value: "{name}", oninput: move |e| name.set(e.value()) }
            }
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Rareza",
                    select { style: INPUT_STYLE, onchange: move |e| rarity.set(e.value()),
                        for r in rarities { option { value: "{r}", "{r}" } }
                    }
                }
                FormField { label: "Peso (lb)",
                    input { style: INPUT_STYLE, r#type: "number", placeholder: "1.0",
                        value: "{weight}", oninput: move |e| weight.set(e.value()) }
                }
            }
            FormField { label: "Fuente",
                input { style: INPUT_STYLE, placeholder: "Homebrew",
                    value: "{source}", oninput: move |e| source.set(e.value()) }
            }
            FormField { label: "Descripción",
                textarea { style: "{INPUT_STYLE} resize:vertical;", rows: "4",
                    placeholder: "Descripción del objeto y sus propiedades…",
                    value: "{description}", oninput: move |e| description.set(e.value()) }
            }
            FormField { label: "Tags (separados por coma)",
                input { style: INPUT_STYLE, placeholder: "mágico, quest, frágil",
                    value: "{tags_raw}", oninput: move |e| tags_raw.set(e.value()) }
            }
            FormField { label: "Notas",
                textarea { style: "{INPUT_STYLE} resize:vertical;", rows: "2",
                    value: "{notes}", oninput: move |e| notes.set(e.value()) }
            }
            SaveButton {
                onclick: move |_| {
                    let tags: Vec<String> = tags_raw
                        .read()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    on_save.call(NewItemData {
                        name: name.read().clone(), description: description.read().clone(),
                        weight: weight.read().parse::<f32>().ok(), rarity: rarity.read().clone(),
                        source: source.read().clone(), notes: notes.read().clone(),
                        tags,
                        extra: Default::default(),
                    });
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// dnd-desktop/src/ui/screens/item_forms/treasure_form.rs
// ─────────────────────────────────────────────────────────────────────────────

// NOTE: Este archivo es treasure_form.rs — se separa en su propio fichero en el proyecto.
// Aquí lo incluimos para la entrega junto con misc_form.rs.

use dioxus::prelude::*;

#[component]
pub fn TreasureForm(on_save: EventHandler<super::NewItemData>) -> Element {
    use super::weapon_form::{FormField, SaveButton, INPUT_STYLE};

    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut weight = use_signal(String::new);
    let mut rarity = use_signal(|| "uncommon".to_string());
    let mut source = use_signal(|| "Homebrew".to_string());
    let mut notes = use_signal(String::new);
    let mut gp_value = use_signal(String::new);
    let mut treasure_type = use_signal(|| "gem".to_string());

    let rarities = [
        "common",
        "uncommon",
        "rare",
        "very_rare",
        "legendary",
        "artifact",
    ];

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",
            FormField { label: "Nombre *",
                input { style: INPUT_STYLE, placeholder: "Rubí de sangre…",
                    value: "{name}", oninput: move |e| name.set(e.value()) }
            }
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Tipo de tesoro",
                    select { style: INPUT_STYLE, onchange: move |e| treasure_type.set(e.value()),
                        option { value: "gem",       "Gema" }
                        option { value: "art",       "Obra de arte" }
                        option { value: "jewellery", "Joyería" }
                        option { value: "coin",      "Moneda especial" }
                        option { value: "other",     "Otro" }
                    }
                }
                FormField { label: "Rareza",
                    select { style: INPUT_STYLE, onchange: move |e| rarity.set(e.value()),
                        for r in rarities { option { value: "{r}", "{r}" } }
                    }
                }
            }
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Valor (PO)",
                    input { style: INPUT_STYLE, r#type: "number", placeholder: "50",
                        value: "{gp_value}", oninput: move |e| gp_value.set(e.value()) }
                }
                FormField { label: "Peso (lb)",
                    input { style: INPUT_STYLE, r#type: "number", placeholder: "0.1",
                        value: "{weight}", oninput: move |e| weight.set(e.value()) }
                }
            }
            FormField { label: "Fuente",
                input { style: INPUT_STYLE, placeholder: "Homebrew",
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
                    extra.insert("treasure_type".into(), treasure_type.read().clone());
                    if !gp_value.read().is_empty() {
                        extra.insert("gp_value".into(), gp_value.read().clone());
                    }
                    on_save.call(super::NewItemData {
                        name: name.read().clone(), description: description.read().clone(),
                        weight: weight.read().parse::<f32>().ok(), rarity: rarity.read().clone(),
                        source: source.read().clone(), notes: notes.read().clone(),
                        tags: vec!["treasure".into(), treasure_type.read().clone()],
                        extra,
                    });
                }
            }
        }
    }
}
