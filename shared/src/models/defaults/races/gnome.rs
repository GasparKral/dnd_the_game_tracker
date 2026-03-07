use crate::traits::race::Race;

#[derive(Debug)]
pub struct Gnome;

impl Race for Gnome {
    fn id(&self) -> &'static str {
        "gnome"
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 7.5m, Astucia Gnómica (ventaja en salvaciones Int/Sab/Car vs magia),
        // Visión en la Oscuridad 18m, Linaje (Gnomo de las Rocas o Gnomo Silvático o Gnomo de las Profundidades)
    }
}
