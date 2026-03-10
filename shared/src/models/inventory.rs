use super::dice::DiceRoll;
use super::items::{Item, Rarity};

#[derive(Debug, Clone, Hash)]
pub enum DropType {
    Fixed(u32),
    InRange(DiceRoll),
}

// Bug 1 corregido: orden de campos alineado con api_types::inventory::Currency
// (copper → silver → electrum → gold → platinum).
// Antes electrum y gold estaban intercambiados respecto al DTO de red.
#[derive(Debug, Clone, Hash, Default, PartialEq, Eq)]
pub struct Currency {
    pub copper:   u32,
    pub silver:   u32,
    pub electrum: u32,
    pub gold:     u32,
    pub platinum: u32,
}

pub type ItemStack = (u32, Item);
