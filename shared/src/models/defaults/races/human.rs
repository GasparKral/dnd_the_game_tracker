use crate::traits::race::Race;

#[derive(Debug)]
pub struct Human;

impl Race for Human {
    fn id(&self) -> &'static str { "human" }
    fn sub_race(&self) -> Option<&'static str> { None }
    fn apply(&self, _character: &mut crate::models::character::Player) {
        // PHB 2024: Velocidad 9m, Versátil (competencia en habilidad adicional),
        // Talento Heroico (don extra al nivel 1 de la lista de dotes de nivel 1)
    }
}
