use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Dwarf;

impl Race for Dwarf {
    fn id(&self) -> &'static str { "dwarf" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "dwarf".into(),
            name: "Enano".into(),
            source: "PHB2024".into(),
            description: Some("Resistentes y tenaces, los enanos son maestros artesanos de las montañas.".into()),
            lore: Some("Los enanos forjan su legado en piedra y acero. Cada clan guarda \
                       tradiciones milenarias de combate y artesanado incomparables.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "dwarf.lineage".into(),
                    label: "Linaje enano".into(),
                    options: vec![
                        SelectOption::new("hill_dwarf", "Enano de las Colinas",
                            "Sabiduría +1, PG máximos +1 por nivel (Robustez Enana)."),
                        SelectOption::new("mountain_dwarf", "Enano de la Montaña",
                            "Proficiencia con armadura ligera y media, Fuerza +2."),
                        SelectOption::new("duergar", "Duergar",
                            "Resistencia a veneno ampliada, Agrandarse/Reducirse 1/desc. corto, Invisibilidad 1/desc. corto."),
                    ],
                },
            ],
            required_choices: vec!["dwarf.lineage".into()],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Resistencia Enana".into(),
                "Fortaleza Pétrea".into(),
                "Velocidad de Herramienta".into(),
                "Entrenamiento de Combate Enano".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Visión en la Oscuridad",
                    "Ves en penumbra hasta 18 m como luz brillante y en oscuridad como penumbra."),
                TraitDetail::new("Resistencia Enana",
                    "Ventaja en salvaciones contra veneno y resistencia al daño por veneno."),
                TraitDetail::new("Fortaleza Pétrea",
                    "Cuando te mueves, puedes hacerlo en terreno difícil subterráneo sin coste extra de movimiento."),
                TraitDetail::new("Velocidad de Herramienta",
                    "Proficiencia con herramientas de artesano: cantero, herrero o cervecero."),
                TraitDetail::new("Entrenamiento de Combate Enano",
                    "Proficiencia con hacha de batalla, hacha de mano, martillo ligero y martillo de guerra."),
            ],
            speed_m: Some(8),   // 25 ft = ~7.5 m, redondeado a 8 m en la UI
            size: Some("Medium".into()),
        }
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
