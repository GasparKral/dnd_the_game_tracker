use crate::api_types::catalog::{CatalogEntry, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct HalfOrc;

impl Race for HalfOrc {
    fn id(&self) -> &'static str { "half_orc" }

    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "half_orc".into(),
            name: "Semiorco".into(),
            source: "PHB2024".into(),
            description: Some("Feroces y resistentes, los semiorcos poseen una tenacidad sobrenatural.".into()),
            lore: Some("Portadores de la ferocidad orca y la adaptabilidad humana, los semiorcos \
                       sobreviven allí donde otros caen.".into()),
            image_url: None,
            choices: vec![],
            required_choices: vec![],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Resistencia".into(),
                "Feroz".into(),
                "Ataques Implacables".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Visión en la Oscuridad",
                    "Ves en penumbra hasta 18 m como luz brillante y en oscuridad como penumbra."),
                TraitDetail::new("Resistencia",
                    "Cuando los PG caen a 0 sin morir, quedan en 1 PG en cambio. Usable 1 vez por descanso largo."),
                TraitDetail::new("Feroz",
                    "Obtienes competencia en Intimidación."),
                TraitDetail::new("Ataques Implacables",
                    "Cuando realizas el ataque que derriba a un enemigo, puedes hacer un ataque adicional como acción adicional."),
            ],
            speed_m: Some(9),
            size: Some("Medium".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
