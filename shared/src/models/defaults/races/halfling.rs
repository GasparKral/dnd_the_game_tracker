use crate::traits::race::Race;

#[derive(Debug)]
pub struct Halfling;

impl Race for Halfling {
    fn id(&self) -> &'static str {
        "halfling"
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 7.5m, Suerte (relanzar 1s), Valentía (ventaja vs miedo),
        // Agilidad Halfling (moverse por espacio de criatura mayor), Linaje (Pies Ligeros o Robusto)
    }
}
