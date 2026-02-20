use crate::models::character::Player;
use std::fmt::Debug;

pub trait Class: Debug {
    fn id(&self) -> &'static str;
    fn sub_class(&self) -> &'static str;
    fn apply(&self, character: &mut Player);
}
