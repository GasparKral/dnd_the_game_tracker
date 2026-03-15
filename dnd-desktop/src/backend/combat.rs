// =============================================================================
// backend/combat.rs — CombatManager: estado del combate en memoria
// =============================================================================

use shared::api_types::combat::{
    AddCombatantRequest, Combatant, CombatState, Condition,
    EnemyTemplate, InitiativeRollResult, UpdateConditionsRequest, UpdateHpRequest,
};
use tokio::sync::RwLock;
use uuid::Uuid;

// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct CombatManager {
    state: RwLock<CombatState>,
}

impl CombatManager {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(CombatState::new()),
        }
    }

    // ── Lectura ──────────────────────────────────────────────────────────────

    pub async fn snapshot(&self) -> CombatState {
        self.state.read().await.clone()
    }

    // ── Ciclo de combate ─────────────────────────────────────────────────────

    /// Inicia el combate. Si ya había un combate activo lo resetea.
    pub async fn start(&self) {
        let mut s = self.state.write().await;
        s.active = true;
        s.round = 1;
        s.current_turn_index = 0;
        // Conservamos los combatientes que pudieran estar ya añadidos
    }

    /// Termina el combate pero conserva la lista de combatientes para revisar.
    pub async fn end(&self) {
        let mut s = self.state.write().await;
        s.active = false;
    }

    /// Limpia todos los combatientes y reinicia el tracker.
    pub async fn reset(&self) {
        let mut s = self.state.write().await;
        *s = CombatState::new();
    }

    // ── Combatientes ─────────────────────────────────────────────────────────

    /// Añade un combatiente manualmente.
    pub async fn add_combatant(&self, req: AddCombatantRequest) -> Combatant {
        let dex_mod = req
            .dexterity
            .map(Combatant::modifier)
            .unwrap_or(req.initiative_bonus);

        let c = Combatant {
            id:               Uuid::new_v4(),
            name:             req.name,
            kind:             req.kind,
            hp_current:       req.hp_max,
            hp_max:           req.hp_max,
            hp_temp:          0,
            armor_class:      req.armor_class,
            initiative:       None,
            initiative_bonus: dex_mod,
            strength:         req.strength,
            dexterity:        req.dexterity,
            constitution:     req.constitution,
            intelligence:     req.intelligence,
            wisdom:           req.wisdom,
            charisma:         req.charisma,
            conditions:       Vec::new(),
            abilities:        req.abilities,
            character_id:     req.character_id,
            template_path:    None,
            concentrating_on: None,
            legendary_actions: req.legendary_action_count,
            notes:            String::new(),
        };

        let mut s = self.state.write().await;
        s.combatants.push(c.clone());
        c
    }

    /// Instancia uno o varios enemigos desde una plantilla.
    pub async fn add_from_template(&self, template: EnemyTemplate, count: u32) -> Vec<Combatant> {
        let mut s = self.state.write().await;
        let mut added = Vec::new();

        for i in 1..=count {
            let suffix = if count > 1 { Some(to_roman(i)) } else { None };
            let suffix_str = suffix.as_deref();
            let c = template.instantiate(suffix_str);
            s.combatants.push(c.clone());
            added.push(c);
        }
        added
    }

    /// Elimina un combatiente por id.
    pub async fn remove_combatant(&self, id: Uuid) -> bool {
        let mut s = self.state.write().await;
        let before = s.combatants.len();
        let active_id = s.combatants.get(s.current_turn_index).map(|c| c.id);
        s.combatants.retain(|c| c.id != id);
        // Ajustar índice si eliminamos uno antes o en el turno activo
        if s.combatants.len() < before {
            if let Some(aid) = active_id {
                if aid == id {
                    // El combatiente eliminado era el activo → quedamos en el mismo índice (el siguiente)
                    if s.current_turn_index >= s.combatants.len() && !s.combatants.is_empty() {
                        s.current_turn_index = 0;
                    }
                } else if let Some(new_idx) = s.combatants.iter().position(|c| c.id == aid) {
                    s.current_turn_index = new_idx;
                }
            }
            true
        } else {
            false
        }
    }

    // ── HP ───────────────────────────────────────────────────────────────────

    /// Aplica un cambio de HP a un combatiente.
    /// Devuelve el combatiente actualizado, o None si no se encontró.
    pub async fn update_hp(&self, id: Uuid, req: UpdateHpRequest) -> Option<Combatant> {
        let mut s = self.state.write().await;
        let c = s.combatants.iter_mut().find(|c| c.id == id)?;

        if req.temporary {
            if req.set_temp {
                // Reemplazar PV temporales solo si el nuevo valor es mayor
                c.hp_temp = req.delta.max(c.hp_temp);
            } else {
                c.hp_temp = (c.hp_temp + req.delta).max(0);
            }
        } else {
            // Los PV temporales absorben el daño primero
            let damage = -req.delta;
            if damage > 0 {
                let absorbed = damage.min(c.hp_temp);
                c.hp_temp -= absorbed;
                let remaining_damage = damage - absorbed;
                c.hp_current = (c.hp_current - remaining_damage).max(-(c.hp_max)); // muerte instantánea si dobla el máximo
                // Si cae a 0 o menos, añadir condición Inconsciente
                if c.hp_current <= 0 && !c.conditions.contains(&Condition::Unconscious) {
                    c.conditions.push(Condition::Unconscious);
                }
            } else {
                // Curación: no puede superar el máximo
                c.hp_current = (c.hp_current + req.delta).min(c.hp_max);
                // Si recupera PV, quitar Inconsciente (si no hay otra causa)
                if c.hp_current > 0 {
                    c.conditions.retain(|cond| *cond != Condition::Unconscious);
                }
            }
        }

        Some(c.clone())
    }

    // ── Condiciones ──────────────────────────────────────────────────────────

    pub async fn update_conditions(
        &self,
        id: Uuid,
        req: UpdateConditionsRequest,
    ) -> Option<Combatant> {
        let mut s = self.state.write().await;
        let c = s.combatants.iter_mut().find(|c| c.id == id)?;
        c.conditions = req.conditions;
        c.concentrating_on = req.concentrating_on;
        Some(c.clone())
    }

    // ── Iniciativa ───────────────────────────────────────────────────────────

    /// Fija la iniciativa de un combatiente manualmente.
    pub async fn set_initiative(&self, id: Uuid, value: i32) -> Option<Combatant> {
        let mut s = self.state.write().await;
        let c = s.combatants.iter_mut().find(|c| c.id == id)?;
        c.initiative = Some(value);
        Some(c.clone())
    }

    /// Tirada de iniciativa masiva: 1d20 + bonus para todos los que no la tienen,
    /// a menos que `reroll_all` sea true. Los overrides manuales se aplican directamente.
    pub async fn roll_initiative(
        &self,
        reroll_all: bool,
        overrides: std::collections::HashMap<Uuid, i32>,
    ) -> (Vec<InitiativeRollResult>, CombatState) {
        let mut s = self.state.write().await;
        let mut rolls = Vec::new();

        for c in s.combatants.iter_mut() {
            if let Some(&manual) = overrides.get(&c.id) {
                c.initiative = Some(manual);
                rolls.push(InitiativeRollResult {
                    combatant_id: c.id,
                    name:         c.name.clone(),
                    d20_roll:     manual - c.initiative_bonus,
                    bonus:        c.initiative_bonus,
                    total:        manual,
                });
            } else if reroll_all || c.initiative.is_none() {
                let d20: i32 = rand::random_range(1i32..=20i32);
                let total = d20 + c.initiative_bonus;
                c.initiative = Some(total);
                rolls.push(InitiativeRollResult {
                    combatant_id: c.id,
                    name:         c.name.clone(),
                    d20_roll:     d20,
                    bonus:        c.initiative_bonus,
                    total,
                });
            }
        }

        // Ordenar por iniciativa descendente
        s.sort_by_initiative();
        (rolls, s.clone())
    }

    // ── Flujo de turnos ──────────────────────────────────────────────────────

    /// Avanza al siguiente turno.
    pub async fn next_turn(&self) -> CombatState {
        let mut s = self.state.write().await;
        s.advance_turn();
        s.clone()
    }

    /// Salta al turno de un combatiente específico (uso de legendarias, etc.)
    pub async fn set_turn(&self, id: Uuid) -> Option<CombatState> {
        let mut s = self.state.write().await;
        let idx = s.combatants.iter().position(|c| c.id == id)?;
        s.current_turn_index = idx;
        Some(s.clone())
    }

    // ── Notas ────────────────────────────────────────────────────────────────

    pub async fn update_notes(&self, id: Uuid, notes: String) -> Option<Combatant> {
        let mut s = self.state.write().await;
        let c = s.combatants.iter_mut().find(|c| c.id == id)?;
        c.notes = notes;
        Some(c.clone())
    }
}

// ---------------------------------------------------------------------------
// Utilidades internas
// ---------------------------------------------------------------------------

fn to_roman(n: u32) -> String {
    match n {
        1 => "I".into(), 2 => "II".into(), 3 => "III".into(),
        4 => "IV".into(), 5 => "V".into(), 6 => "VI".into(),
        7 => "VII".into(), 8 => "VIII".into(), 9 => "IX".into(),
        10 => "X".into(), _ => n.to_string(),
    }
}
