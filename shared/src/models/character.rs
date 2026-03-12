use crate::models::{
    attributes::{Attribute, HitPoints, MagicStats},
    dice::*,
    effect::Effect,
    inventory::{Currency, DropType, ItemStack},
    items::Item,
};

use crate::traits::{background::Background, class::Class, feat::Feat, race::Race};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
    pub speed: i32,
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

// ---------------------------------------------------------------------------
// Proficiencias — escritas por race/class/background en apply()
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SkillProf {
    Athletics,
    Acrobatics,
    SleightOfHand,
    Stealth,
    Arcana,
    History,
    Investigation,
    Nature,
    Religion,
    AnimalHandling,
    Insight,
    Medicine,
    Perception,
    Survival,
    Deception,
    Intimidation,
    Performance,
    Persuasion,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SavingThrowProf {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ArmorProf {
    Light,
    Medium,
    Heavy,
    Shield,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeaponProf {
    Simple,
    Martial,
    Specific(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DamageKind {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

/// Rasgo especial narrativo/mecánico sin campo numérico directo.
/// Lo usan race/class/background/feats para registrar rasgos que
/// el cliente puede mostrar en la ficha.
#[derive(Debug, Clone, Hash)]
pub struct SpecialTrait {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Player {
    pub entity: Entity,
    pub level: u32,
    pub exp: u64,
    pub bonus: u32,

    // ── Rasgos mecánicos concedidos por raza/clase/trasfondo/dotes ──────────
    /// Visión en la oscuridad en metros (0 = sin visión especial)
    pub darkvision: u32,
    /// Resistencias a tipos de daño
    pub damage_resistances: Vec<DamageKind>,
    /// Inmunidades a tipos de daño
    pub damage_immunities: Vec<DamageKind>,
    /// Proficiencias en habilidades
    pub skill_proficiencies: Vec<SkillProf>,
    /// Experticia (doble bono de proficiencia)
    pub skill_expertises: Vec<SkillProf>,
    /// Proficiencias en salvaciones
    pub saving_throw_proficiencies: Vec<SavingThrowProf>,
    /// Proficiencias en armaduras
    pub armor_proficiencies: Vec<ArmorProf>,
    /// Proficiencias en armas
    pub weapon_proficiencies: Vec<WeaponProf>,
    /// Idiomas adicionales concedidos por raza o trasfondo
    pub extra_languages: Vec<String>,
    /// Rasgos especiales que no caben en un campo numérico
    pub special_traits: Vec<SpecialTrait>,

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

    // ── Helpers para apply() ─────────────────────────────────────────────

    pub fn add_skill(&mut self, s: SkillProf) {
        if !self.skill_proficiencies.contains(&s) {
            self.skill_proficiencies.push(s);
        }
    }

    pub fn add_expertise(&mut self, s: SkillProf) {
        if !self.skill_expertises.contains(&s) {
            self.skill_expertises.push(s);
        }
    }

    pub fn add_save(&mut self, s: SavingThrowProf) {
        if !self.saving_throw_proficiencies.contains(&s) {
            self.saving_throw_proficiencies.push(s);
        }
    }

    pub fn add_armor_prof(&mut self, a: ArmorProf) {
        if !self.armor_proficiencies.contains(&a) {
            self.armor_proficiencies.push(a);
        }
    }

    pub fn add_weapon_prof(&mut self, w: WeaponProf) {
        if !self.weapon_proficiencies.contains(&w) {
            self.weapon_proficiencies.push(w);
        }
    }

    pub fn add_resistance(&mut self, d: DamageKind) {
        if !self.damage_resistances.contains(&d) {
            self.damage_resistances.push(d);
        }
    }

    pub fn add_immunity(&mut self, d: DamageKind) {
        if !self.damage_immunities.contains(&d) {
            self.damage_immunities.push(d);
        }
    }

    pub fn add_language(&mut self, lang: &str) {
        let s = lang.to_string();
        if !self.extra_languages.contains(&s) {
            self.extra_languages.push(s);
        }
    }

    pub fn add_trait(&mut self, t: SpecialTrait) {
        if !self.special_traits.iter().any(|x| x.id == t.id) {
            self.special_traits.push(t);
        }
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
