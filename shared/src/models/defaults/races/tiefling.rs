use crate::traits::race::Race;

#[derive(Debug)]
pub struct Tiefling;

impl Race for Tiefling {
    fn id(&self) -> &'static str {
        "tiefling"
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 9m, Visión en la Oscuridad 18m, Resistencia a Fuego,
        // Legado Infernal (conjuros innatos según linaje: Asmodeus, Baalzebul, Dispater, Fierna,
        // Glasya, Levistus, Mammon, Mephistopheles, Zariel)
    }
}
