use crate::traits::race::Race;

#[derive(Debug)]
pub struct HalfOrc;

impl Race for HalfOrc {
    fn id(&self) -> &'static str {
        "half_orc"
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 9m, Visión en la Oscuridad 18m, Resistencia (sobrevivir con 1 PG una vez por descanso),
        // Feroz (bonus prof en Intimidación), Ataques Implacables (daño adicional con armas cuerpo a cuerpo)
    }
}
