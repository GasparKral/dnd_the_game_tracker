use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Tiefling;

impl Race for Tiefling {
    fn id(&self) -> &'static str { "tiefling" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "tiefling".into(),
            name: "Tiefling".into(),
            source: "PHB2024".into(),
            description: Some("Marcados por herencia infernal, los tieflings poseen poderes oscuros innatos.".into()),
            lore: Some("No eligieron su herencia, pero aprenden a vivir con ella.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "tiefling.lineage".into(),
                    label: "Linaje infernal".into(),
                    options: vec![
                        SelectOption::new("asmodeus", "Asmodeo",
                            "Llama Infernal + Oscuridad. Resistencia al fuego."),
                        SelectOption::new("zariel", "Zariel",
                            "Llama Infernal + Marca Abrasadora. Resistencia al fuego."),
                        SelectOption::new("levistus", "Levistus",
                            "Armadura de Agíaz (frío) + Escudo de Escarcha. Resistencia al frío."),
                        SelectOption::new("glasya", "Glasya",
                            "Imagen Menor + Misty Step. Resistencia al fuego."),
                        SelectOption::new("mammon", "Mammón",
                            "Floating Disk + Arcane Lock. Resistencia al fuego."),
                        SelectOption::new("mephistopheles", "Mefistófeles",
                            "Llama Infernal + Geas. Resistencia al fuego."),
                        SelectOption::new("baalzebul", "Baalzebul",
                            "Llama Infernal + Crown of Madness. Resistencia al fuego."),
                        SelectOption::new("dispater", "Dispater",
                            "Disfrazarse + Invisibilidad. Resistencia al fuego."),
                        SelectOption::new("fierna", "Fierna",
                            "Amigos + Charm Person + Dominate Person. Resistencia al fuego."),
                    ],
                },
            ],
            required_choices: vec!["tiefling.lineage".into()],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Resistencia al Fuego".into(),
                "Legado Infernal".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Visión en la Oscuridad",
                    "Ves en penumbra hasta 18 m como luz brillante y en oscuridad como penumbra."),
                TraitDetail::new("Resistencia al Fuego",
                    "Tienes resistencia al daño de fuego."),
                TraitDetail::new("Legado Infernal",
                    "Según tu linaje aprendes conjuros innátos que lanzas sin componentes."),
            ],
            speed_m: Some(9),
            size: Some("Medium".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
