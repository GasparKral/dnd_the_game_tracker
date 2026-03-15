// =============================================================================
// api_types/combat.rs — DTOs de combate (DM ↔ servidor ↔ jugadores)
// DnD 5.5e 2024
// =============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Condiciones DnD 2024
// ---------------------------------------------------------------------------

/// Condiciones del SRD 5.1 / 2024.
/// Se serializa como snake_case para que el JSON sea idiomático.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    // ── Condiciones estándar ──────────────────────────────────────────────
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,   // nivel 1-6 → ver `ExhaustionLevel`
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
    // ── Condiciones extendidas/homebrew comunes ───────────────────────────
    Burning,      // En llamas
    Frozen,       // Congelado / Paralized por frío
    Slowed,       // Ralentizado (hechizo Slow)
    Concentrating, // Marcador de concentración
    Blessed,      // Bendecido (hechizo Bless)
    Cursed,       // Maldito
    Hexed,        // Hex del brujo
    HuntersMark,  // Hunter's Mark del explorador
    // ── Custom (el DM puede añadir etiquetas libres) ──────────────────────
    Custom(String),
}

impl Condition {
    pub fn label(&self) -> String {
        match self {
            Self::Blinded       => "Cegado".into(),
            Self::Charmed       => "Hechizado".into(),
            Self::Deafened      => "Ensordecido".into(),
            Self::Exhaustion    => "Agotamiento".into(),
            Self::Frightened    => "Asustado".into(),
            Self::Grappled      => "Agarrado".into(),
            Self::Incapacitated => "Incapacitado".into(),
            Self::Invisible     => "Invisible".into(),
            Self::Paralyzed     => "Paralizado".into(),
            Self::Petrified     => "Petrificado".into(),
            Self::Poisoned      => "Envenenado".into(),
            Self::Prone         => "Derribado".into(),
            Self::Restrained    => "Retenido".into(),
            Self::Stunned       => "Aturdido".into(),
            Self::Unconscious   => "Inconsciente".into(),
            Self::Burning       => "En llamas 🔥".into(),
            Self::Frozen        => "Congelado ❄️".into(),
            Self::Slowed        => "Ralentizado".into(),
            Self::Concentrating => "Concentrado ✨".into(),
            Self::Blessed       => "Bendecido ⬆️".into(),
            Self::Cursed        => "Maldito ☠️".into(),
            Self::Hexed         => "Hex".into(),
            Self::HuntersMark   => "Marca del Cazador".into(),
            Self::Custom(s)     => s.clone(),
        }
    }

    /// Color de badge para la UI.
    pub fn color_class(&self) -> &'static str {
        match self {
            Self::Burning    => "bg-orange-600",
            Self::Frozen     => "bg-blue-400",
            Self::Poisoned   => "bg-green-600",
            Self::Paralyzed | Self::Stunned | Self::Unconscious => "bg-yellow-600",
            Self::Blessed | Self::Concentrating | Self::HuntersMark => "bg-purple-600",
            Self::Cursed | Self::Hexed => "bg-red-800",
            Self::Invisible  => "bg-slate-500",
            Self::Slowed     => "bg-cyan-700",
            _ => "bg-stone-600",
        }
    }

    /// Lista de todas las condiciones predefinidas (sin Custom) para mostrar en selector.
    pub fn all_standard() -> Vec<Condition> {
        vec![
            Self::Blinded, Self::Charmed, Self::Deafened, Self::Exhaustion,
            Self::Frightened, Self::Grappled, Self::Incapacitated, Self::Invisible,
            Self::Paralyzed, Self::Petrified, Self::Poisoned, Self::Prone,
            Self::Restrained, Self::Stunned, Self::Unconscious,
            Self::Burning, Self::Frozen, Self::Slowed,
            Self::Concentrating, Self::Blessed, Self::Cursed, Self::Hexed, Self::HuntersMark,
        ]
    }
}

// ---------------------------------------------------------------------------
// Tipo de combatiente
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatantKind {
    /// Personaje jugador
    Player,
    /// Enemigo (monstruo, PNJ hostil)
    Enemy,
    /// Mascota, familiar o montura controlada por un PJ
    Companion,
    /// PNJ neutral o aliado
    Npc,
}

impl CombatantKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Player    => "PJ",
            Self::Enemy     => "Enemigo",
            Self::Companion => "Compañero",
            Self::Npc       => "PNJ",
        }
    }
}

// ---------------------------------------------------------------------------
// Habilidad de combate (para enemigos/NPCs)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombatAbility {
    pub name: String,
    pub description: String,
    /// Coste de acción: "action", "bonus", "reaction", "legendary", "free"
    pub action_cost: String,
    /// Tirada de daño, e.g. "2d6+4"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_roll: Option<String>,
    /// Alcance en pies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_ft: Option<u32>,
    /// CD de salvación, si aplica
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_dc: Option<u32>,
}

// ---------------------------------------------------------------------------
// Combatiente en el tracker
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Combatant {
    pub id: Uuid,
    pub name: String,
    pub kind: CombatantKind,

    // ── Puntos de vida ────────────────────────────────────────────────────
    pub hp_current: i32,
    pub hp_max: i32,
    /// PV temporales (no se suman al máximo, se pierden antes que los normales)
    pub hp_temp: i32,

    // ── Estadísticas de combate ───────────────────────────────────────────
    pub armor_class: u32,
    /// Resultado de la tirada de iniciativa
    pub initiative: Option<i32>,
    /// Modificador de DEX (o iniciativa especial) que se suma a la tirada d20
    pub initiative_bonus: i32,

    // ── Atributos base (solo relevante para enemigos/NPCs) ────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength:     Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dexterity:    Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constitution: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wisdom:       Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charisma:     Option<i32>,

    // ── Condiciones activas ───────────────────────────────────────────────
    pub conditions: Vec<Condition>,

    // ── Habilidades (solo enemigos/NPCs) ─────────────────────────────────
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<CombatAbility>,

    // ── Metadatos ─────────────────────────────────────────────────────────
    /// Si viene de un personaje guardado, su UUID en la persistencia
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_id: Option<Uuid>,
    /// Si viene de una plantilla del vault, el path relativo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_path: Option<String>,
    /// Si está concentrando un hechizo, su nombre
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concentrating_on: Option<String>,
    /// Número de acciones legendarias restantes (si procede)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_actions: Option<u32>,
    /// Notas libres del DM
    #[serde(default)]
    pub notes: String,
}

impl Combatant {
    pub fn hp_percentage(&self) -> f32 {
        if self.hp_max <= 0 {
            return 0.0;
        }
        ((self.hp_current as f32 + self.hp_temp as f32) / self.hp_max as f32 * 100.0).min(100.0).max(0.0)
    }

    pub fn is_down(&self) -> bool {
        self.hp_current <= 0
    }

    pub fn is_dead(&self) -> bool {
        self.conditions.contains(&Condition::Unconscious) && self.hp_current <= 0
    }

    /// Estado de vida como etiqueta
    pub fn health_label(&self) -> &'static str {
        let pct = self.hp_percentage();
        if pct <= 0.0       { "Caído" }
        else if pct <= 25.0 { "Crítico" }
        else if pct <= 50.0 { "Herido" }
        else if pct <= 75.0 { "Dañado" }
        else                { "Sano" }
    }

    /// Color de barra de HP
    pub fn hp_bar_color(&self) -> &'static str {
        let pct = self.hp_percentage();
        if pct <= 0.0        { "bg-stone-700" }
        else if pct <= 25.0  { "bg-red-600" }
        else if pct <= 50.0  { "bg-orange-500" }
        else if pct <= 75.0  { "bg-yellow-400" }
        else                 { "bg-emerald-500" }
    }

    /// Modifier estándar DnD para un valor de atributo
    pub fn modifier(score: i32) -> i32 {
        (score - 10) / 2
    }

    pub fn dex_modifier(&self) -> i32 {
        self.dexterity.map(Self::modifier).unwrap_or(0)
    }
}

// ---------------------------------------------------------------------------
// Estado global del combate
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CombatState {
    /// Combatientes ordenados por iniciativa (descendente)
    pub combatants: Vec<Combatant>,
    /// Índice en `combatants` del combatiente cuyo turno es ahora
    pub current_turn_index: usize,
    /// Número de ronda (empieza en 1)
    pub round: u32,
    /// Si el combate está activo
    pub active: bool,
}

impl CombatState {
    pub fn new() -> Self {
        Self {
            combatants: Vec::new(),
            current_turn_index: 0,
            round: 1,
            active: false,
        }
    }

    /// Devuelve el combatiente activo en este turno
    pub fn current_combatant(&self) -> Option<&Combatant> {
        self.combatants.get(self.current_turn_index)
    }

    /// Ordena los combatientes por iniciativa descendente (desempate: bonus más alto)
    pub fn sort_by_initiative(&mut self) {
        self.combatants.sort_by(|a, b| {
            let ai = a.initiative.unwrap_or(-99);
            let bi = b.initiative.unwrap_or(-99);
            bi.cmp(&ai)
                .then_with(|| b.initiative_bonus.cmp(&a.initiative_bonus))
        });
        // Resetear el índice de turno al inicio
        self.current_turn_index = 0;
    }

    /// Avanza al siguiente turno; si llega al final, nueva ronda
    pub fn advance_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }
        let alive_count = self.combatants.iter().filter(|c| !c.is_down()).count();
        if alive_count == 0 {
            return;
        }
        let start = self.current_turn_index;
        loop {
            self.current_turn_index = (self.current_turn_index + 1) % self.combatants.len();
            if self.current_turn_index == 0 {
                self.round += 1;
            }
            if self.current_turn_index == start {
                // dió la vuelta completa sin encontrar ninguno activo
                break;
            }
            if !self.combatants[self.current_turn_index].is_down() {
                break;
            }
        }
    }

    pub fn combatant_by_id(&self, id: Uuid) -> Option<&Combatant> {
        self.combatants.iter().find(|c| c.id == id)
    }

    pub fn combatant_by_id_mut(&mut self, id: Uuid) -> Option<&mut Combatant> {
        self.combatants.iter_mut().find(|c| c.id == id)
    }
}

impl Default for CombatState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Plantilla de enemigo (vault / manual)
// ---------------------------------------------------------------------------

/// Plantilla reutilizable de enemigo, puede venir del vault (frontmatter)
/// o ser añadida manualmente por el DM.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnemyTemplate {
    /// Nombre del monstruo
    pub name: String,
    /// Tipo de criatura (humanoid, undead, beast…)
    #[serde(default)]
    pub creature_type: String,
    /// Challenge Rating (CR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cr: Option<String>,
    pub hp_max: i32,
    pub armor_class: u32,
    /// Velocidad en pies
    #[serde(default = "default_speed")]
    pub speed: u32,
    // ── Atributos ─────────────────────────────────────────────────────────
    #[serde(default = "default_stat")]
    pub strength:     i32,
    #[serde(default = "default_stat")]
    pub dexterity:    i32,
    #[serde(default = "default_stat")]
    pub constitution: i32,
    #[serde(default = "default_stat")]
    pub intelligence: i32,
    #[serde(default = "default_stat")]
    pub wisdom:       i32,
    #[serde(default = "default_stat")]
    pub charisma:     i32,
    // ── Descripción / lore ────────────────────────────────────────────────
    #[serde(default)]
    pub description: String,
    // ── Habilidades ───────────────────────────────────────────────────────
    #[serde(default)]
    pub abilities: Vec<CombatAbility>,
    // ── Acciones legendarias ──────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_action_count: Option<u32>,
    // ── Referencia de vault ───────────────────────────────────────────────
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_path: Option<String>,
    /// Tags del vault
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_speed() -> u32 { 30 }
fn default_stat()  -> i32 { 10 }

impl EnemyTemplate {
    /// Genera un `Combatant` a partir de esta plantilla (se puede llamar múltiples veces
    /// para añadir varias instancias del mismo enemigo en el mismo combate).
    pub fn instantiate(&self, suffix: Option<&str>) -> Combatant {
        let dex_mod = Combatant::modifier(self.dexterity);
        let name = match suffix {
            Some(s) => format!("{} {}", self.name, s),
            None    => self.name.clone(),
        };
        Combatant {
            id:               Uuid::new_v4(),
            name,
            kind:             CombatantKind::Enemy,
            hp_current:       self.hp_max,
            hp_max:           self.hp_max,
            hp_temp:          0,
            armor_class:      self.armor_class,
            initiative:       None,
            initiative_bonus: dex_mod,
            strength:         Some(self.strength),
            dexterity:        Some(self.dexterity),
            constitution:     Some(self.constitution),
            intelligence:     Some(self.intelligence),
            wisdom:           Some(self.wisdom),
            charisma:         Some(self.charisma),
            conditions:       Vec::new(),
            abilities:        self.abilities.clone(),
            character_id:     None,
            template_path:    self.vault_path.clone(),
            concentrating_on: None,
            legendary_actions: self.legendary_action_count,
            notes:            String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Requests / Responses
// ---------------------------------------------------------------------------

/// POST /combat/start — inicia el combate (vacío o con combatientes iniciales)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCombatRequest {
    /// IDs de personajes guardados para añadir automáticamente
    #[serde(default)]
    pub player_character_ids: Vec<Uuid>,
}

/// POST /combat/combatant — añade un combatiente manualmente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCombatantRequest {
    pub name: String,
    pub kind: CombatantKind,
    pub hp_max: i32,
    pub armor_class: u32,
    #[serde(default)]
    pub initiative_bonus: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength:     Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dexterity:    Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constitution: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wisdom:       Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charisma:     Option<i32>,
    #[serde(default)]
    pub abilities: Vec<CombatAbility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legendary_action_count: Option<u32>,
}

/// POST /combat/combatant/from-template — instancia desde plantilla
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFromTemplateRequest {
    pub template: EnemyTemplate,
    /// Cuántas instancias añadir (por defecto 1)
    #[serde(default = "default_count")]
    pub count: u32,
}
fn default_count() -> u32 { 1 }

/// PATCH /combat/combatant/{id}/hp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateHpRequest {
    /// Delta positivo = curación, negativo = daño
    pub delta: i32,
    /// Si true, el delta va a los PV temporales en lugar de a los normales
    #[serde(default)]
    pub temporary: bool,
    /// Si true, los PV temporales se sustituyen (no se suman)
    #[serde(default)]
    pub set_temp: bool,
}

/// PATCH /combat/combatant/{id}/initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetInitiativeRequest {
    pub value: i32,
}

/// PATCH /combat/combatant/{id}/conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConditionsRequest {
    /// Lista completa de condiciones activas (reemplaza la anterior)
    pub conditions: Vec<Condition>,
    /// Si se está concentrando, el nombre del hechizo (None = no concentra)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concentrating_on: Option<String>,
}

/// PATCH /combat/combatant/{id}/notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNotesRequest {
    pub notes: String,
}

/// POST /combat/roll-initiative — tirada de iniciativa masiva
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RollInitiativeRequest {
    /// Si true, re-tira incluso los que ya tienen iniciativa asignada
    #[serde(default)]
    pub reroll_all: bool,
    /// Resultados manuales del DM (uuid → valor), evita tirada automática para ese id
    #[serde(default)]
    pub manual_overrides: std::collections::HashMap<Uuid, i32>,
}

/// Resultado de la tirada de iniciativa de un combatiente individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiativeRollResult {
    pub combatant_id: Uuid,
    pub name: String,
    pub d20_roll: i32,
    pub bonus: i32,
    pub total: i32,
}

/// Response de POST /combat/roll-initiative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollInitiativeResponse {
    pub rolls: Vec<InitiativeRollResult>,
    pub state: CombatState,
}
