use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Sistema de bonificadores de estadística
// ---------------------------------------------------------------------------

/// Estadística que puede ser modificada por un objeto equipado.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BonusStat {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    ArmorClass,
    MaxHp,
    Speed,
    AttackBonus,
    DamageBonus,
    SavingThrowStr,
    SavingThrowDex,
    SavingThrowCon,
    SavingThrowInt,
    SavingThrowWis,
    SavingThrowCha,
}

impl BonusStat {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Strength         => "FUE",
            Self::Dexterity        => "DES",
            Self::Constitution     => "CON",
            Self::Intelligence     => "INT",
            Self::Wisdom           => "SAB",
            Self::Charisma         => "CAR",
            Self::ArmorClass       => "CA",
            Self::MaxHp            => "PG máx",
            Self::Speed            => "Velocidad",
            Self::AttackBonus      => "Ataque",
            Self::DamageBonus      => "Daño",
            Self::SavingThrowStr   => "Sal. FUE",
            Self::SavingThrowDex   => "Sal. DES",
            Self::SavingThrowCon   => "Sal. CON",
            Self::SavingThrowInt   => "Sal. INT",
            Self::SavingThrowWis   => "Sal. SAB",
            Self::SavingThrowCha   => "Sal. CAR",
        }
    }
}

/// Tipo de bonificador — determina las reglas de apilamiento (D&D 5.5e).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BonusType {
    /// Bonus de objeto equipado — se apilan entre sí
    Item,
    /// Bonus circunstancial — se apilan
    Circumstance,
    /// Bonus de estado — NO se apilan, solo el mayor cuenta
    Status,
    /// Sin tipo — siempre se apila
    Untyped,
}

impl BonusType {
    fn default_untyped() -> BonusType {
        BonusType::Untyped
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatBonus {
    pub stat: BonusStat,
    pub value: i16,
    #[serde(default = "BonusType::default_untyped")]
    pub bonus_type: BonusType,
    #[serde(default)]
    pub source: String,
}

// ---------------------------------------------------------------------------
// Categorías de objeto
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    Weapon,
    Armour,
    Consumable,
    Tool,
    Treasure,
    Accessory,
    Misc,
}

impl ItemCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Weapon     => "Arma",
            Self::Armour     => "Armadura",
            Self::Consumable => "Consumible",
            Self::Tool       => "Herramienta",
            Self::Treasure   => "Tesoro",
            Self::Accessory  => "Accesorio",
            Self::Misc       => "Misc",
        }
    }
}

// ---------------------------------------------------------------------------
// Objeto del inventario
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InventoryItem {
    pub id: Uuid,
    pub name: String,
    pub category: ItemCategory,
    #[serde(default)]
    pub description: String,
    pub quantity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
    #[serde(default)]
    pub equipped: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stat_bonuses: Vec<StatBonus>,
    #[serde(default)]
    pub notes: String,
}

impl InventoryItem {
    pub fn new(
        name: impl Into<String>,
        category: ItemCategory,
        description: impl Into<String>,
        quantity: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            category,
            description: description.into(),
            quantity,
            weight: None,
            equipped: false,
            accessory_type: None,
            stat_bonuses: Vec::new(),
            notes: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Monedas
// ---------------------------------------------------------------------------

// Bug 1 corregido: orden de campos consistente con models::inventory::Currency
// (copper → silver → electrum → gold → platinum en ambos structs).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Currency {
    #[serde(default)]
    pub copper: u32,
    #[serde(default)]
    pub silver: u32,
    #[serde(default)]
    pub electrum: u32,
    #[serde(default)]
    pub gold: u32,
    #[serde(default)]
    pub platinum: u32,
}

// ---------------------------------------------------------------------------
// Requests / Responses
// ---------------------------------------------------------------------------

// Bug 5 corregido en todos los request types que solo derivaban Deserialize:
// eliminados los `skip_serializing_if` que eran ruido inoperante en structs
// sin Serialize. Se añade Serialize + Clone a todos para uniformidad y para
// que puedan viajar por WebSocket / logs sin tocar shared en el futuro.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddItemRequest {
    pub name: String,
    pub category: ItemCategory,
    #[serde(default)]
    pub description: String,
    pub quantity: u32,
    #[serde(default)]
    pub weight: Option<f32>,
    #[serde(default)]
    pub accessory_type: Option<String>,
    #[serde(default)]
    pub stat_bonuses: Vec<StatBonus>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    #[serde(default)]
    pub quantity: Option<u32>,
    #[serde(default)]
    pub equipped: Option<bool>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCurrencyRequest {
    #[serde(default)]
    pub copper: Option<u32>,
    #[serde(default)]
    pub silver: Option<u32>,
    #[serde(default)]
    pub electrum: Option<u32>,
    #[serde(default)]
    pub gold: Option<u32>,
    #[serde(default)]
    pub platinum: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct InventoryResponse {
    pub items: Vec<InventoryItem>,
    pub currency: Currency,
    /// Peso total de todos los objetos (suma de weight * quantity)
    pub total_weight: f32,
}

impl InventoryResponse {
    pub fn from_parts(items: Vec<InventoryItem>, currency: Currency) -> Self {
        let total_weight = items
            .iter()
            .map(|i| i.weight.unwrap_or(0.0) * i.quantity as f32)
            .sum();
        Self { items, currency, total_weight }
    }
}
