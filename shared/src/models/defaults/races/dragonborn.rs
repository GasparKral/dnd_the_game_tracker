use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Dragonborn;

impl Race for Dragonborn {
    fn id(&self) -> &'static str { "dragonborn" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "dragonborn".into(),
            name: "Dracónido".into(),
            source: "PHB2024".into(),
            description: Some("Orgullosos guerreros con sangre dracónica y un arma de aliento devastadora.".into()),
            lore: Some("Nacidos de la magia de los dragones, los dracónidos \
                       honran a sus antepasados con una vida de honor y poder.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "dragonborn.lineage".into(),
                    label: "Linaje dracónico".into(),
                    options: vec![
                        SelectOption::new("black",  "Dragón Negro",    "Ácido. Aliento de ácido (C.Def. 3 m × 9 m)."),
                        SelectOption::new("blue",   "Dragón Azul",     "Relámpago. Aliento de rayo (C.Def. 1.5 m × 9 m)."),
                        SelectOption::new("red",    "Dragón Rojo",     "Fuego. Aliento de fuego (C.Def. 3 m × 9 m)."),
                        SelectOption::new("white",  "Dragón Blanco",   "Frío. Aliento de frío (C.Def. 3 m × 9 m)."),
                        SelectOption::new("green",  "Dragón Verde",    "Veneno. Aliento de veneno (C.Def. 3 m × 9 m)."),
                        SelectOption::new("gold",   "Dragón Dorado",   "Fuego. Aliento de fuego o gas debilitante."),
                        SelectOption::new("silver", "Dragón Plateado", "Frío. Aliento de frío o gas paralizante."),
                        SelectOption::new("bronze", "Dragón Bronce",   "Relámpago. Aliento de rayo o gas repelente."),
                        SelectOption::new("copper", "Dragón Cobre",    "Ácido. Aliento de ácido o gas lenificante."),
                        SelectOption::new("brass",  "Dragón Latón",    "Fuego. Aliento de fuego o gas soporifíco."),
                    ],
                },
            ],
            required_choices: vec!["dragonborn.lineage".into()],
            traits_preview: vec![
                "Arma de Aliento".into(),
                "Resistencia Dracónica".into(),
                "Instinto Dracónico".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Arma de Aliento",
                    "Exhaó un cónero o línea de energía según tu linaje. Cada criatura en el área realiza una ST (CD = 8 + bono prof + mod Con)."),
                TraitDetail::new("Resistencia Dracónica",
                    "Tienes resistencia al tipo de daño asociado a tu linaje dracónico."),
                TraitDetail::new("Instinto Dracónico",
                    "Tienes ventaja en las pruebas de Intimidación o Persuasión (elige al obtener este rasgo)."),
            ],
            speed_m: Some(9),
            size: Some("Medium".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
