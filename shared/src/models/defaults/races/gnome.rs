use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Gnome;

impl Race for Gnome {
    fn id(&self) -> &'static str { "gnome" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "gnome".into(),
            name: "Gnomo".into(),
            source: "PHB2024".into(),
            description: Some("Inventivos e inteligentes, los gnomos poseen una curiosidad insaciable.".into()),
            lore: Some("Los gnomos viven en exuberante celebración de la vida. \
                       Artilugios, magia y risa son sus compañeros de viaje más preciados.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "gnome.lineage".into(),
                    label: "Linaje gnómico".into(),
                    options: vec![
                        SelectOption::new("rock", "Gnomo de las Rocas",
                            "Conocimiento de Artificiero: truco Prestidigitación, proficiencia con herramientas de artesano."),
                        SelectOption::new("forest", "Gnomo Silvático",
                            "Magia Natural: trucos Ilusiones Menores, hablar con animales pequeños a voluntad."),
                        SelectOption::new("deep", "Gnomo de las Profundidades",
                            "Camuflaje Superior: ventaja en pruebas de Sigilo en terreno rocoso o subterráneo."),
                    ],
                },
            ],
            required_choices: vec!["gnome.lineage".into()],
            traits_preview: vec![
                "Astucia Gnómica".into(),
                "Visión en la Oscuridad".into(),
                "Linaje Gnómico".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Astucia Gnómica",
                    "Ventaja en las salvaciones de Inteligencia, Sabiduría y Carisma contra magia."),
                TraitDetail::new("Visión en la Oscuridad",
                    "Ves en penumbra hasta 18 m como luz brillante y en oscuridad como penumbra."),
            ],
            speed_m: Some(8),
            size: Some("Small".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
