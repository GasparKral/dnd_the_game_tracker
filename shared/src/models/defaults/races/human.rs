use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Human;

impl Race for Human {
    fn id(&self) -> &'static str { "human" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "human".into(),
            name: "Humano".into(),
            source: "PHB2024".into(),
            description: Some("Versátiles y ambiciosos, los humanos son la raza más extendida.".into()),
            lore: Some("Los humanos son la especie más adaptable y ambiciosa. \
                       Su corta vida les impulsa a alcanzar la gloria rápidamente.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "human.extra_skill".into(),
                    label: "Habilidad adicional (Versátil)".into(),
                    options: vec![
                        SelectOption::new("athletics",       "Atletismo",       "Fuerza"),
                        SelectOption::new("acrobatics",      "Acrobacias",      "Destreza"),
                        SelectOption::new("sleight_of_hand", "Juego de Manos",  "Destreza"),
                        SelectOption::new("stealth",         "Sigilo",          "Destreza"),
                        SelectOption::new("arcana",          "Arcanos",         "Inteligencia"),
                        SelectOption::new("history",         "Historia",        "Inteligencia"),
                        SelectOption::new("investigation",   "Investigación",   "Inteligencia"),
                        SelectOption::new("nature",          "Naturaleza",      "Inteligencia"),
                        SelectOption::new("religion",        "Religión",        "Inteligencia"),
                        SelectOption::new("animal_handling", "Trato Animales",  "Sabiduría"),
                        SelectOption::new("insight",         "Perspicacia",     "Sabiduría"),
                        SelectOption::new("medicine",        "Medicina",        "Sabiduría"),
                        SelectOption::new("perception",      "Percepción",      "Sabiduría"),
                        SelectOption::new("survival",        "Supervivencia",   "Sabiduría"),
                        SelectOption::new("deception",       "Engaño",          "Carisma"),
                        SelectOption::new("intimidation",    "Intimidación",    "Carisma"),
                        SelectOption::new("performance",     "Interpretación",  "Carisma"),
                        SelectOption::new("persuasion",      "Persuasión",      "Carisma"),
                    ],
                },
            ],
            required_choices: vec!["human.extra_skill".into()],
            traits_preview: vec!["Versátil".into(), "Talento Heroíco".into()],
            traits_detail: vec![
                TraitDetail::new("Versátil",
                    "Ganas competencia en una habilidad adicional a tu elección."),
                TraitDetail::new("Talento Heroíco",
                    "Obtienes un don de origen al nivel 1 (cualquier don de la lista de origen)."),
            ],
            speed_m: Some(9),
            size: Some("Medium".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
