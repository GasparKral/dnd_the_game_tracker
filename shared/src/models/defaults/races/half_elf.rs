use crate::traits::race::Race;

#[derive(Debug)]
pub struct HalfElf;

impl Race for HalfElf {
    fn id(&self) -> &'static str {
        "half_elf"
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 9m, Visión en la Oscuridad 18m, Sentidos Feéricos,
        // Herencia Humana (Versátil: competencia en habilidad o herramienta),
        // Herencia Élfica (elegir rasgo de elfo: Trance, Linaje Élfico o Máscara de lo Salvaje)
    }
}
