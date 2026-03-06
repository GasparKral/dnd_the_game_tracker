use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    Misc,
}

impl ItemCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Weapon => "Arma",
            Self::Armour => "Armadura",
            Self::Consumable => "Consumible",
            Self::Tool => "Herramienta",
            Self::Treasure => "Tesoro",
            Self::Misc => "Misc",
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
    /// Cantidad del stack
    pub quantity: u32,
    /// Peso unitario en libras (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
    /// true si el objeto está equipado actualmente
    #[serde(default)]
    pub equipped: bool,
    /// Notas libres (ej. propiedades mágicas, condiciones)
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
            notes: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Monedas
// ---------------------------------------------------------------------------

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

#[derive(Debug, Deserialize)]
pub struct AddItemRequest {
    pub name: String,
    pub category: ItemCategory,
    #[serde(default)]
    pub description: String,
    pub quantity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCurrencyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copper: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silver: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electrum: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gold: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        Self {
            items,
            currency,
            total_weight,
        }
    }
}
