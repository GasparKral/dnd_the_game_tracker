use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Halfling;

impl Race for Halfling {
    fn id(&self) -> &'static str { "halfling" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "halfling".into(),
            name: "Mediano".into(),
            source: "PHB2024".into(),
            description: Some("Pequeños y afortunados, los medianos destacan por su suerte innata y valentía.".into()),
            lore: Some("Desconfiados de las aventuras pero llenos de recursos, los medianos \
                       encuentran el hogar dondequiera que van.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "halfling.lineage".into(),
                    label: "Linaje de mediano".into(),
                    options: vec![
                        SelectOption::new("lightfoot", "Pies Ligeros",
                            "Puedes ocultarte detrás de criaturas de tamaño Medium o mayor."),
                        SelectOption::new("stout", "Robusto",
                            "Resistencia al daño por veneno y ventaja en salvaciones contra veneno."),
                        SelectOption::new("ghostwise", "Alma de Espectro",
                            "Telepatía limitada: puedes comunicarte telepáticamente 9 m."),
                    ],
                },
            ],
            required_choices: vec!["halfling.lineage".into()],
            traits_preview: vec![
                "Suerte".into(),
                "Valentía".into(),
                "Agilidad Halfling".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Suerte",
                    "Cuando sacas un 1 en un d20 para un ataque, prueba de habilidad o salvación, puedes relanzar el dado."),
                TraitDetail::new("Valentía",
                    "Tienes ventaja en las tiradas de salvación contra el estado Asustado."),
                TraitDetail::new("Agilidad Halfling",
                    "Puedes moverte a través del espacio de cualquier criatura de un tamaño mayor que el tuyo."),
            ],
            speed_m: Some(8),
            size: Some("Small".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
