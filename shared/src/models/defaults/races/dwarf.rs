use crate::traits::race::Race;

#[derive(Debug)]
pub struct Dwarf;

impl Race for Dwarf {
    fn id(&self) -> &'static str {
        return "Dwarf";
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, character: &mut crate::models::character::Player) {}
}
