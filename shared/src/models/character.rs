use crate::models::{
    attributes::{Attribute, HitPoints, MagicStats},
    dice::*,
    effect::Effect,
    inventory::{Currency, DropType, ItemStack},
    items::Item,
};

use crate::traits::{background::Background, class::Class, feat::Feat, race::Race};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Hash)]
pub enum Alignment {
    LawfullGood,
    NeutralGood,
    ChaoticGood,
    LawfullNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfullEvil,
    NeutralEvil,
    ChaoticEvil,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

#[derive(Debug, Hash)]
pub struct Entity {
    id: u64,
    pub hp: HitPoints,
    pub alignment: Alignment,
    pub name: String,
    pub class_armour: u32,
    pub speed: u32,
    pub size: Size,
    pub inventory: Vec<ItemStack>,
    pub iniciative: u8,
    pub perceptions: u32,
    pub languages: Vec<String>,
    pub currency: Currency,
    pub notes: Vec<String>,
}

impl Entity {
    pub fn id(&self) -> u64 {
        self.id
    }
}

#[derive(Debug)]
pub struct Player {
    pub entity: Entity,
    level: u32,
    exp: u64,
    pub bonus: u32,
    pub race: Box<dyn Race>,
    pub class: Box<dyn Class>,
    pub background: Box<dyn Background>,
    pub feats: Vec<Box<dyn Feat>>,
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity.hash(state);
        self.level.hash(state);
        self.exp.hash(state);
        self.bonus.hash(state);

        self.race.id().hash(state);
        self.class.id().hash(state);
        self.background.id().hash(state);
    }
}

impl Player {
    pub fn level(&self) -> u32 {
        self.level
    }
    pub fn exp(&self) -> u64 {
        self.exp
    }
    pub fn add_exp(&mut self, value: u64) {
        self.exp += value;
        if self.check_level_up() {
            self.level_up();
        }
    }

    fn check_level_up(&self) -> bool {
        match self.level {
            1 => self.exp >= 0,
            2 => self.exp >= 300,
            3 => self.exp >= 900,
            4 => self.exp >= 2700,
            5 => self.exp >= 6500,
            6 => self.exp >= 14000,
            7 => self.exp >= 23000,
            8 => self.exp >= 34000,
            9 => self.exp >= 48000,
            10 => self.exp >= 64000,
            11 => self.exp >= 85000,
            12 => self.exp >= 100000,
            13 => self.exp >= 120000,
            14 => self.exp >= 140000,
            15 => self.exp >= 165000,
            16 => self.exp >= 195000,
            17 => self.exp >= 225000,
            18 => self.exp >= 265000,
            19 => self.exp >= 305000,
            20 => self.exp >= 355000,
            _ => false,
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
    }
}

pub type CRRequierement = u8;

#[derive(Debug, Hash)]
pub enum Archetype {
    Aberration,
    Humanoid,
    Beast,
    Undead,
    CelestialFiend,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Giant,
    Ooze,
    Plant,
}

#[derive(Debug, Hash)]
pub struct Enemy {
    pub entity: Entity,
    pub drop_table: Vec<(CRRequierement, DropType, Item)>,
    pub exp_drop: u64,
    pub cr: u32,
    pub archetype: Archetype,
}
