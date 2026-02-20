use crate::models::character::Player;
use std::fmt::Debug;

pub trait Background: Debug {
    fn id(&self) -> &'static str;
    fn apply(&self, character: &mut Player);
}
