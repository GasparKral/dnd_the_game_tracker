use crate::traits::race::Race;

#[derive(Debug)]
pub struct Elf;

impl Race for Elf {
    fn id(&self) -> &'static str { "elf" }
    fn sub_race(&self) -> Option<&'static str> { None }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 9m, Visión en la Oscuridad 18m, Sentidos Feéricos
        // (ventaja en salvaciones vs Hechizado), Trance (descanso largo en 4h),
        // Linaje Élfico (Alto Elfo / Elfo del Bosque / Elfo Oscuro)
    }
}
