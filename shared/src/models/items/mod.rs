pub mod armours;
pub mod generics;
pub mod weapons;

#[derive(Debug, Clone, Copy, Hash)]
pub enum Rarity {
    Common,
    UnCommon,
    Rare,
    ExtremilyRare,
    Legendary,
}

#[derive(Debug, Clone, Hash)]
pub struct ItemBase {
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    equiped: bool,
}

#[derive(Debug, Clone, Hash)]
pub enum Item {
    Generic(ItemBase),
    Weapon(weapons::Weapon),
    Armour(armours::Armour),
}

impl Item {
    fn base(&self) -> &ItemBase {
        match self {
            Self::Generic(i) => &i,
            Self::Weapon(w) => &w.base,
            Self::Armour(a) => &a.base,
        }
    }

    pub fn name(&self) -> &str {
        &self.base().name
    }
    pub fn description(&self) -> &str {
        &self.base().description
    }
    pub fn rarity(&self) -> Rarity {
        self.base().rarity
    }
}
