// src/messages.rs

use serde::{Deserialize, Serialize};
use shared::models::dice::RollResult;
use uuid::Uuid;

/// Mensajes que llegan DESDE el móvil al servidor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Join {
        player_name: String,
        character_name: String,
    },
    RollDice {
        /// Resultado completo: incluye dados, modo, modificador, valores individuales y total
        roll_result: RollResult,
    },
    RequestSync,
    /// El jugador modificó su inventario — avisa al DM para refrescar
    InventoryUpdated {
        character_id: Uuid,
    },
}

/// Mensajes que van DESDE el servidor al móvil
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    // Conexión
    Welcome {
        player_id: Uuid,
        player_name: String,
    },
    // Combate
    CombatStarted,
    CombatEnded,
    InitiativeUpdate {
        order: Vec<InitiativeEntry>,
    },
    HpUpdate {
        character_id: Uuid,
        current: i32,
        max: i32,
    },
    ConditionUpdate {
        character_id: Uuid,
        conditions: Vec<String>,
    },
    // Tiradas
    DiceRoll {
        player_id: Uuid,
        roll_result: RollResult,
    },
    /// Tirada realizada por el propio DM (broadcast al feed de todos)
    DmDiceRoll {
        roll_result: RollResult,
    },
    // DM → jugador específico
    PrivateMessage {
        target_id: Uuid,
        text: String,
    },
    // DM → todos
    Announcement {
        text: String,
    },
    /// El inventario o monedas de un personaje cambiaron — el jugador debe recargar
    InventoryChanged {
        character_id: Uuid,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiativeEntry {
    pub character_id: Uuid,
    pub name: String,
    pub initiative: i32,
    pub is_player: bool,
}
