use crate::traits::race::Race;

#[derive(Debug)]
pub struct Dwarf;

impl Race for Dwarf {
    fn id(&self) -> &'static str { "dwarf" }
    fn sub_race(&self) -> Option<&'static str> { None }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 7.5m (sin penalización por armadura pesada),
        // Visión en la Oscuridad 18m, Resistencia Enana (ventaja en salvaciones
        // vs veneno, resistencia a daño por veneno), Robustez Enana (+1 PG/nivel),
        // Linaje (Enano de las Colinas / Enano de las Montañas / Duergar)
    }
}
