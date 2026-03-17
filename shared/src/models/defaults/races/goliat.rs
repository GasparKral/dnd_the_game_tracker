use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::traits::race::Race;

#[derive(Debug)]
pub struct Goliat;

impl Race for Goliat {
    fn id(&self) -> &'static str {
        "goliat"
    }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "goliat".into(),
            name: "Goliat".into(),
            source: "PHB2024".into(),
            description: Some("".into()),
            lore: Some("".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "gigiant.linage".into(),
                label: "Linage de los Gigantes".into(),
                options: vec![
                    SelectOption::new("fire","Abrasión del Fuego"," Cuando aciertes a un objetivo con una tirada de ataque y le causes daño, 
también puedes causarle 1d10 de daño de fuego."),
                    SelectOption::new("slopes","Caída de las Colinas"," Cuando aciertes a una criatura Grande o más pequeña con 
una tirada de ataque y le causes daño, también puedes infligirle el estado de derribada."),
                    SelectOption::new("clouths","Escursión de las Nubes","Como acción adicional, te teletransportas mágicamente hasta 9 m a un espacio sin ocupar que puedas ver."),
                    SelectOption::new("frost","Frío de la Escarcha","Cuando aciertes a un objetivo con una tirada de ataque y le causes daño, 
también puedes causarle 1d6 de daño de frío y reducir su velocidad en 3 m hasta el principio de tu siguiente turno."),
                    SelectOption::new("stone","Resistencia de la Piedra","Cuando recibas daño, puedes usar una reacción para tirar 1d12. Suma tu modificador por Constitución al resultado y reduce el daño en ese total."),
                    SelectOption::new("rain","Trueno de la Tormenta","Cuando una criatura que esté a 18 m o menos de ti te cause daño, puedes usar una reacción para infligirle 1d8 de daño de trueno."),
                ],
            }],
            required_choices: vec!["gigiant.linage".into()],
            traits_preview: vec![
                "Constitución Poderosa".into(),
                "Forma Grande".into(),
            ],
            traits_detail: vec![
                TraitDetail::new("Constitución Poderosa","Tienes ventaja en cualquier prueba de característica que hagas para poner fin al estado de agarrado. Además, al determinar tu capacidad de carga,cuentas como si tuvieras un tamaño una categoría superior."
                    ),
                    TraitDetail::new("Forma Grande",". A partir del nivel 5 de personaje, puedes cambiar de tamaño a Grande como acción adicional si estás en un lugar lo bastante espacioso. Esta transformación dura 10 minutos o hasta que le pongas fin (no requiere acción). Durante ese tiempo, tendrás ventaja en las pruebas de Fuerza y tu velocidad aumentará en 3 m. Cuando uses este atributo, no podrás volver a hacerlo hasta que finalices un descanso largo.")
            ],
            speed_m: Some(11),
            size: Some("Medium".into()),
        }
    }

    fn apply(&self, _character: &mut crate::models::character::Player) {}
}
