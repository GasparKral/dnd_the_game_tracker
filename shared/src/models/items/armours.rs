use super::ItemBase;
use crate::models::damage::DamageType;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ArmourClass {
    // CA fija, no añade DES — Plate, Chain Mail
    Fixed(u32),
    // CA base + DEX hasta un máximo — Scale Mail, Chain Shirt
    WithDexCap { base: u32, max_dex: u32 },
    // CA base + DEX completo — Leather, Studded Leather
    WithFullDex(u32),
    // Escudo, se suma a la CA existente
    Shield(u32),
}

impl ArmourClass {
    pub fn calculate(&self, dex_modifier: i32) -> u32 {
        match self {
            ArmourClass::Fixed(base) => *base,
            ArmourClass::WithDexCap { base, max_dex } => {
                let dex_bonus = dex_modifier.max(0) as u32;
                base + dex_bonus.min(*max_dex)
            }
            ArmourClass::WithFullDex(base) => {
                let dex_bonus = dex_modifier.max(0) as u32;
                base + dex_bonus
            }
            ArmourClass::Shield(bonus) => *bonus, // se aplica aparte
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MagicArmourProperty {
    PlusAC(u32),                               // +1, +2, +3
    Resistance(DamageType),                    // resistencia a un tipo de daño
    Immunity(DamageType),                      // inmunidad a un tipo de daño
    SpellSlotBonus { level: u32, slots: u32 }, // Armadura del archimago, etc
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ArmourProperty {
    // Requiere fuerza mínima para no tener penalización de movimiento
    StrengthRequirement(u32),
    // Desventaja en Stealth
    StealthDisadvantage,
    // 5.5e: algunas armaduras tienen estas
    Noisy,       // igual que StealthDisadvantage pero narrativamente distinto
    Bulky,       // desventaja en ciertos checks atléticos
    Comfortable, // ventaja en tiradas de resistencia por clima frío
}

#[derive(Debug, Clone, Copy, Hash)]
pub enum ArmourPiece {
    Helmet,
    ChestPlate,
    Leggins,
    Boots,
    Rings,
    Charms,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ArmourCategory {
    Light,
    Medium,
    Heavy,
    Shield,
}

#[derive(Debug, Clone, Hash)]
pub struct Armour {
    pub base: ItemBase,
    pub category: ArmourCategory,
    pub piece_type: ArmourPiece,
    pub armour_class: ArmourClass,
    pub properties: Vec<ArmourProperty>,
    pub magic_properties: Vec<MagicArmourProperty>,
}

impl Armour {
    pub fn equip(&mut self) {
        self.base.equiped = true;
    }
    pub fn unequip(&mut self) {
        self.base.equiped = false;
    }
    pub fn is_magical(&self) -> bool {
        !self.magic_properties.is_empty()
    }

    pub fn effective_ac(&self, dex_modifier: i32) -> u32 {
        let base = self.armour_class.calculate(dex_modifier);
        // sumamos el bonus mágico si lo hay
        let magic_bonus: u32 = self
            .magic_properties
            .iter()
            .filter_map(|p| match p {
                MagicArmourProperty::PlusAC(b) => Some(*b),
                _ => None,
            })
            .sum();
        base + magic_bonus
    }

    pub fn has_stealth_disadvantage(&self) -> bool {
        self.properties
            .contains(&ArmourProperty::StealthDisadvantage)
            || self.properties.contains(&ArmourProperty::Noisy)
    }

    pub fn strength_requirement(&self) -> Option<u32> {
        self.properties.iter().find_map(|p| match p {
            ArmourProperty::StrengthRequirement(s) => Some(*s),
            _ => None,
        })
    }

    pub fn resistances(&self) -> Vec<&DamageType> {
        self.magic_properties
            .iter()
            .filter_map(|p| match p {
                MagicArmourProperty::Resistance(dt) => Some(dt),
                _ => None,
            })
            .collect()
    }
}
