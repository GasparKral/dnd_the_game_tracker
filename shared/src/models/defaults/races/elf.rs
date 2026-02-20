use crate::traits::race::Race;

#[derive(Debug)]
pub struct Elf;

impl Race for Elf {
    fn id(&self) -> &'static str {
        return "Elf";
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, character: &mut crate::models::character::Player) {}
}
