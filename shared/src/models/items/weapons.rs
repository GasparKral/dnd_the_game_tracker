use crate::models::{
    damage::DamageType,
    dice::*,
    items::{ItemBase, Rarity},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeaponProperty {
    Finesse,
    Light,
    Heavy,
    Reach,
    Thrown { normal_range: u32, long_range: u32 },
    Versatile(Vec<(DiceRoll, DamageType)>), // el daño al usarla a dos manos
    Loading,
    Ammunition { normal_range: u32, long_range: u32 },
    TwoHanded,
    Silvered,
    Magical,
    // 5.5e añade estas:
    Nick,   // permite el ataque extra del dual wielding sin bonus action
    Graze,  // en fallo, haces daño de modificador igual
    Push,   // puedes empujar al objetivo
    Slow,   // reduces la velocidad del objetivo
    Topple, // puedes derribar al objetivo
    Vex,    // ventaja en el siguiente ataque contra ese objetivo
    Cleave, // puedes atacar a otro objetivo cercano
    Sap,    // el objetivo tiene desventaja en su siguiente tirada
}

// Efectos que muchas armas mágicas comparten
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MagicProperty {
    PlusDamage(u32),                       // +1, +2, +3
    PlusAttack(u32),                       // bonus al hit
    ElementalDamage(DiceRoll, DamageType), // 1d6 fuego extra, etc
    Returning,                             // vuelve a la mano al lanzarla
    Sentient { alignment: String },        // arma sintiente
    Vorpal,                                // crítico en 19-20 y decapita
    Dancing,                               // puede atacar sola
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeaponCategory {
    Simple,
    Martial,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum WeaponType {
    Melee,
    Ranged,
}

#[derive(Debug, Clone, Hash)]
pub struct Weapon {
    pub base: ItemBase,
    pub category: WeaponCategory,
    pub weapon_type: WeaponType,
    pub damage: Vec<(DiceRoll, DamageType)>,
    pub properties: Vec<WeaponProperty>,
    pub notes: Vec<String>,
}

impl Weapon {
    pub fn equip(&mut self) {
        self.base.equiped = true;
    }
    pub fn unequip(&mut self) {
        self.base.equiped = false;
    }
    pub fn is_two_handed(&self) -> bool {
        self.properties.contains(&WeaponProperty::TwoHanded) || !self.versatile_damage().is_none()
    }

    pub fn versatile_damage(&self) -> Option<&Vec<(DiceRoll, DamageType)>> {
        self.properties.iter().find_map(|p| match p {
            WeaponProperty::Versatile(damage) => Some(damage),
            _ => None,
        })
    }

    pub fn range(&self) -> Option<(u32, u32)> {
        self.properties.iter().find_map(|p| match p {
            WeaponProperty::Thrown {
                normal_range,
                long_range,
            }
            | WeaponProperty::Ammunition {
                normal_range,
                long_range,
            } => Some((*normal_range, *long_range)),
            _ => None,
        })
    }

    pub fn has_property(&self, property: &WeaponProperty) -> bool {
        self.properties.contains(property)
    }
}
