use crate::traits::race::Race;

#[derive(Debug)]
pub struct Human;

impl Race for Human {
    fn id(&self) -> &'static str {
        return "Human";
    }
    fn sub_race(&self) -> Option<&'static str> {
        None
    }
    fn apply(&self, character: &mut crate::models::character::Player) {}
}
