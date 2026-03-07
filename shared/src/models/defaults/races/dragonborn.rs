use crate::traits::race::Race;

#[derive(Debug)]
pub struct Dragonborn;

impl Race for Dragonborn {
    fn id(&self) -> &'static str {
        "dragonborn"
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 9m, Arma de Aliento (daño por tipo de dragón),
        // Resistencia según tipo dracónico, Instinto Dracónico (ventaja en Intimidación o Persuasión),
        // Linaje Dracónico (Ácido/Relámpago/Frío/Fuego/Veneno + subtipos Abismal y Gema)
    }
}
