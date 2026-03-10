// Formulario de creación de ARMAS.
// Campos específicos DnD 5.5e: daño, tipo de daño, propiedades, alcance.

use super::NewItemData;
use dioxus::prelude::*;

#[component]
pub fn WeaponForm(on_save: EventHandler<NewItemData>) -> Element {
    // Campos base
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut weight = use_signal(String::new);
    let mut rarity = use_signal(|| "common".to_string());
    let mut source = use_signal(|| "Homebrew".to_string());
    let mut notes = use_signal(String::new);

    // Campos específicos de arma
    let mut damage_dice = use_signal(String::new); // ej. "1d8"
    let mut damage_type = use_signal(|| "slashing".to_string());
    let mut weapon_type = use_signal(|| "simple".to_string()); // simple | martial
    let mut range_normal = use_signal(String::new); // ej. "20"
    let mut range_long = use_signal(String::new); // ej. "60"
    let mut properties = use_signal(String::new); // finesse, thrown, versatile…
    let mut two_handed_dmg = use_signal(String::new); // versatile damage

    let rarities = [
        "common",
        "uncommon",
        "rare",
        "very_rare",
        "legendary",
        "artifact",
    ];
    let damage_types = [
        "slashing",
        "piercing",
        "bludgeoning",
        "fire",
        "cold",
        "lightning",
        "thunder",
        "poison",
        "acid",
        "psychic",
        "radiant",
        "necrotic",
        "force",
    ];

    rsx! {
        div { style: "display:flex; flex-direction:column; gap:14px;",

            // Nombre
            FormField { label: "Nombre *",
                input {
                    style: INPUT_STYLE,
                    placeholder: "Espada larga +1…",
                    value: "{name}",
                    oninput: move |e| name.set(e.value()),
                }
            }

            // Tipo de arma + Rareza en fila
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Tipo",
                    select {
                        style: INPUT_STYLE,
                        onchange: move |e| weapon_type.set(e.value()),
                        option { value: "simple",  "Simple" }
                        option { value: "martial", "Marcial" }
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

            // Daño + Tipo de daño en fila
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Dado de daño",
                    input {
                        style: INPUT_STYLE,
                        placeholder: "1d8",
                        value: "{damage_dice}",
                        oninput: move |e| damage_dice.set(e.value()),
                    }
                }
                FormField { label: "Tipo de daño",
                    select {
                        style: INPUT_STYLE,
                        onchange: move |e| damage_type.set(e.value()),
                        for dt in damage_types {
                            option { value: "{dt}", "{dt}" }
                        }
                    }
                }
            }

            // Daño versátil (opcional)
            FormField { label: "Daño versátil (dos manos)",
                input {
                    style: INPUT_STYLE,
                    placeholder: "1d10 (si tiene versatile)",
                    value: "{two_handed_dmg}",
                    oninput: move |e| two_handed_dmg.set(e.value()),
                }
            }

            // Alcance
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Alcance normal (ft)",
                    input {
                        style: INPUT_STYLE,
                        placeholder: "5",
                        value: "{range_normal}",
                        oninput: move |e| range_normal.set(e.value()),
                    }
                }
                FormField { label: "Alcance largo (ft)",
                    input {
                        style: INPUT_STYLE,
                        placeholder: "—",
                        value: "{range_long}",
                        oninput: move |e| range_long.set(e.value()),
                    }
                }
            }

            // Propiedades
            FormField { label: "Propiedades (separadas por coma)",
                input {
                    style: INPUT_STYLE,
                    placeholder: "finesse, light, thrown",
                    value: "{properties}",
                    oninput: move |e| properties.set(e.value()),
                }
            }

            // Peso + Fuente
            div { style: "display:grid; grid-template-columns:1fr 1fr; gap:12px;",
                FormField { label: "Peso (lb)",
                    input {
                        style: INPUT_STYLE,
                        r#type: "number",
                        placeholder: "3.0",
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
                    placeholder: "Descripción y lore del arma…",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
            }

            // Notas
            FormField { label: "Notas (propiedades mágicas, condiciones…)",
                textarea {
                    style: "{INPUT_STYLE} resize:vertical;",
                    rows: "2",
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            // Botón guardar
            SaveButton {
                onclick: move |_| {
                    let mut extra = std::collections::HashMap::new();
                    let dmg = format!("{} {}", damage_dice.read(), damage_type.read());
                    extra.insert("damage".into(), dmg.trim().to_string());
                    extra.insert("weapon_type".into(), weapon_type.read().clone());
                    if !range_normal.read().is_empty() || !range_long.read().is_empty() {
                        extra.insert("range".into(),
                            format!("{}/{}", range_normal.read(), range_long.read()));
                    }
                    if !properties.read().is_empty() {
                        extra.insert("properties".into(), properties.read().clone());
                    }
                    if !two_handed_dmg.read().is_empty() {
                        extra.insert("versatile_damage".into(), two_handed_dmg.read().clone());
                    }
                    on_save.call(NewItemData {
                        name:        name.read().clone(),
                        description: description.read().clone(),
                        weight:      weight.read().parse::<f32>().ok(),
                        rarity:      rarity.read().clone(),
                        source:      source.read().clone(),
                        notes:       notes.read().clone(),
                        tags:        vec!["weapon".into(), weapon_type.read().clone()],
                        extra,
                    });
                }
            }
        }
    }
}

// ── Helpers de UI compartidos (visibles solo en este módulo vía re-export) ────

pub const INPUT_STYLE: &str =
    "width:100%; padding:7px 11px; font-size:0.78rem; border-radius:8px;
     background:#0c0a09; border:1px solid #292524; color:#e7e5e4; outline:none; box-sizing:border-box;";

#[component]
pub fn FormField(label: &'static str, children: Element) -> Element {
    rsx! {
        div { style: "display:flex; flex-direction:column; gap:4px;",
            label { style: "font-size:0.65rem; color:#78716c; text-transform:uppercase; letter-spacing:0.07em;",
                "{label}"
            }
            { children }
        }
    }
}

#[component]
pub fn SaveButton(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { style: "display:flex; justify-content:flex-end; padding-top:4px;",
            button {
                style: "padding:8px 22px; font-size:0.78rem; font-weight:600;
                        border-radius:10px; cursor:pointer; border:1px solid #065f46;
                        background:#071a0e; color:#34d399;",
                onclick: move |e| onclick.call(e),
                "💾 Guardar en vault"
            }
        }
    }
}
