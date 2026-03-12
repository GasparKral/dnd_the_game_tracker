use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Elf;

impl Race for Elf {
    fn id(&self) -> &'static str { "elf" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "elf".into(),
            name: "Elfo".into(),
            source: "PHB2024".into(),
            description: Some("Seres mágicos de gran belleza que viven miles de años.".into()),
            lore: Some("Los elfos son una raza mágica de larga vida, vinculados a los bosques \
                       y a los reinos feéricos. Cada linaje porta una herencia arcana distinta.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "elf.lineage".into(),
                    label: "Linaje élfico".into(),
                    options: vec![
                        SelectOption::new("high_elf", "Alto Elfo",
                            "Herencia arcana innata: aprendes un truco de Mago y hablas un idioma extra."),
                        SelectOption::new("wood_elf", "Elfo del Bosque",
                            "Velocidad 10.5 m, Máscara de lo Salvaje: puedes ocultarte en terreno natural."),
                        SelectOption::new("drow", "Drow",
                            "Visión en la Oscuridad superior 36 m, magia de Drow innáta (Luces Danzantes, Oscuridad, Levitar)."),
                    ],
                },
            ],
            required_choices: vec!["elf.lineage".into()],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Sentidos Féericos".into(),
                "Trance".into(),
                "Linaje Élfico".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Visión en la Oscuridad",
                    "Ves en penumbra hasta 18 m como si fuera luz brillante, y en oscuridad como penumbra."),
                TraitDetail::new("Sentidos Féericos",
                    "Tienes ventaja en las tiradas de salvación para evitar el estado Hechizado."),
                TraitDetail::new("Trance",
                    "No necesitas dormir. Meditas profundamente durante 4 horas (equivalente a un descanso largo de 8 h)."),
                TraitDetail::new("Linaje Élfico",
                    "Elige Alto Elfo, Elfo del Bosque o Drow. Cada linaje otorga rasgos adicionales únicos."),
            ],
            speed_m: Some(9),
            size: Some("Medium".into()),
        }
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
