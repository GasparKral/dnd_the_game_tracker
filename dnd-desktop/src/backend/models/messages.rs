// src/messages.rs

use serde::{Deserialize, Serialize};
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
        dice: String, // "1d20", "2d6+3"...
        result: u32,
    },
    RequestSync,
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
        dice: String,
        result: u32,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiativeEntry {
    pub character_id: Uuid,
    pub name: String,
    pub initiative: i32,
    pub is_player: bool,
}
