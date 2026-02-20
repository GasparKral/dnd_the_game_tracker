use super::dice::DiceRoll;
use super::items::{Item, Rarity};

#[derive(Debug, Clone, Hash)]
pub enum DropType {
    Fixed(u32),
    InRange(DiceRoll),
}

#[derive(Debug, Clone, Hash, Default)]
pub struct Currency {
    pub copper: u32,
    pub silver: u32,
    pub gold: u32,
    pub electrum: u32,
    pub platinum: u32,
}

pub type ItemStack = (u32, Item);
