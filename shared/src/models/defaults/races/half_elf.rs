use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct HalfElf;

impl Race for HalfElf {
    fn id(&self) -> &'static str { "half_elf" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "half_elf".into(),
            name: "Semielfo".into(),
            source: "PHB2024".into(),
            description: Some("Con lo mejor de dos mundos, los semielfos combinan adaptabilidad humana con gracia féerica.".into()),
            lore: Some("Ni completamente humanos ni completamente elfos, los semielfos encuentran \
                       su lugar entre dos mundos, siendo mediadores natos.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "half_elf.elven_heritage".into(),
                    label: "Herencia élfica".into(),
                    options: vec![
                        SelectOption::new("trance", "Trance",
                            "Solo necesitas 4 horas de meditación para obtener los beneficios de un descanso largo."),
                        SelectOption::new("elven_lineage", "Linaje Élfico",
                            "Obtienes conjuros innatos de elfo (según el linaje: Alto, Bosque o Drow)."),
                        SelectOption::new("mask_of_the_wild", "Máscara de lo Salvaje",
                            "Puedes intentar ocultarte cuando solo estás ligeramente oscurecido por follaje, lluvia, nieve u otros fenómenos naturales."),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "half_elf.extra_skill".into(),
                    label: "Habilidad adicional (Herencia Humana)".into(),
                    options: vec![
                        SelectOption::bare("athletics",       "Atletismo"),
                        SelectOption::bare("acrobatics",      "Acrobacias"),
                        SelectOption::bare("stealth",         "Sigilo"),
                        SelectOption::bare("arcana",          "Arcanos"),
                        SelectOption::bare("history",         "Historia"),
                        SelectOption::bare("investigation",   "Investigación"),
                        SelectOption::bare("perception",      "Percepción"),
                        SelectOption::bare("persuasion",      "Persuasión"),
                        SelectOption::bare("deception",       "Engaño"),
                        SelectOption::bare("insight",         "Perspicacia"),
                        SelectOption::bare("survival",        "Supervivencia"),
                        SelectOption::bare("intimidation",    "Intimidación"),
                    ],
                },
            ],
            required_choices: vec!["half_elf.elven_heritage".into(), "half_elf.extra_skill".into()],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Sentidos Féericos".into(),
                "Herencia Humana".into(),
                "Herencia Élfica".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Visión en la Oscuridad",
                    "Ves en penumbra hasta 18 m como luz brillante y en oscuridad como penumbra."),
                TraitDetail::new("Sentidos Féericos",
                    "Ventaja en salvaciones para evitar el estado Hechizado."),
                TraitDetail::new("Herencia Humana",
                    "Ganas competencia en una habilidad adicional a tu elección."),
                TraitDetail::new("Herencia Élfica",
                    "Elige un rasgo élfico: Trance, Linaje Élfico o Máscara de lo Salvaje."),
            ],
            speed_m: Some(9),
            size: Some("Medium".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
