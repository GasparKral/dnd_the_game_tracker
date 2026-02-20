use crate::models::character::Player;
use std::fmt::Debug;

pub trait Race: Debug {
    fn id(&self) -> &'static str;
    fn sub_race(&self) -> Option<&'static str>;
    fn apply(&self, character: &mut Player);
}
