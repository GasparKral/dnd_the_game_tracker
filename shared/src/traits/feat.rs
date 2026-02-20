use crate::models::character::Player;
use std::fmt::Debug;

pub trait Feat: Debug {
    fn id(&self) -> &'static str;
    fn apply(&self, character: &mut Player);
}
