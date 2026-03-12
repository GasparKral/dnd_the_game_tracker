#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Dice {
    D2,
    D4,
    D6,
    D8,
    D10,
    D12,
    #[default]
    D20,
    D100,
}

impl Dice {
    pub fn faces(&self) -> u8 {
        match self {
            Dice::D2 => 2,
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
            Dice::D100 => 100,
        }
    }

    pub fn roll(&self) -> u8 {
        rand::random_range(1..=self.faces())
    }

    pub fn roll_many(&self, count: u8) -> Vec<u8> {
        (0..count).map(|_| self.roll()).collect()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DiceRoll {
    pub count: u8,
    pub dice: Dice,
}

impl DiceRoll {
    pub fn roll_all(&self) -> Vec<u8> {
        self.dice.roll_many(self.count)
    }
}

impl std::fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.dice.faces())
    }
}

impl std::str::FromStr for DiceRoll {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"^(\d+)d(\d+)$").unwrap();
        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Formato inválido: '{s}'"))?;

        let count = caps[1].parse::<u8>().map_err(|e| e.to_string())?;
        let faces = caps[2].parse::<u8>().map_err(|e| e.to_string())?;

        let dice = match faces {
            2 => Dice::D2,
            4 => Dice::D4,
            6 => Dice::D6,
            8 => Dice::D8,
            10 => Dice::D10,
            12 => Dice::D12,
            20 => Dice::D20,
            100 => Dice::D100,
            _ => return Err(format!("Dado d{faces} no existe en DnD")),
        };

        Ok(DiceRoll { count, dice })
    }
}

// ─── Tipos de tirada DnD ────────────────────────────────────────────────────

/// Modo de la tirada — aplica solo cuando hay un d20 en la lista.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollMode {
    #[default]
    Normal,
    Advantage,
    Disadvantage,
}

/// Petición de tirada: dados + modificador + modo.
/// Usa las structs `DiceRoll` y `Dice` ya existentes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RollRequest {
    /// Lista de grupos de dados: ej. [2d6, 1d4]
    pub rolls: Vec<DiceRoll>,
    /// Modificador entero, puede ser negativo
    pub modifier: i32,
    /// Solo relevante cuando hay al menos un d20
    pub mode: RollMode,
    /// Etiqueta opcional para el log ("Ataque espada", "Salvación SAB"…)
    pub label: Option<String>,
}

/// Resultado de ejecutar un `RollRequest`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RollResult {
    pub request: RollRequest,
    /// Valores individuales por cada `DiceRoll` en `request.rolls`
    pub individual_rolls: Vec<Vec<u8>>,
    /// El d20 descartado cuando se aplicó ventaja/desventaja
    pub discarded_d20: Option<u8>,
    /// Total final (suma de todos los dados + modificador)
    pub total: i32,
    /// Epoch UNIX en segundos
    pub timestamp: i64,
}

impl RollRequest {
    /// Ejecuta la tirada localmente usando el RNG de `Dice`.
    pub fn execute(&self) -> RollResult {
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut individual_rolls: Vec<Vec<u8>> = self
            .rolls
            .iter()
            .map(|dr| dr.roll_all())
            .collect();

        // Ventaja/desventaja: aplica al primer d20 encontrado
        let mut discarded_d20: Option<u8> = None;
        if self.mode != RollMode::Normal {
            for (i, dr) in self.rolls.iter().enumerate() {
                if dr.dice == Dice::D20 {
                    // Tiramos un d20 extra y elegimos según el modo
                    let extra = Dice::D20.roll();
                    let current = individual_rolls[i][0]; // asumimos 1d20
                    let (keep, discard) = match self.mode {
                        RollMode::Advantage    => {
                            if extra >= current { (extra, current) } else { (current, extra) }
                        }
                        RollMode::Disadvantage => {
                            if extra <= current { (extra, current) } else { (current, extra) }
                        }
                        RollMode::Normal => unreachable!(),
                    };
                    individual_rolls[i][0] = keep;
                    discarded_d20 = Some(discard);
                    break;
                }
            }
        }

        let sum: i32 = individual_rolls
            .iter()
            .flat_map(|v| v.iter())
            .map(|&x| x as i32)
            .sum();

        let total = sum + self.modifier;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        RollResult {
            request: self.clone(),
            individual_rolls,
            discarded_d20,
            total,
            timestamp,
        }
    }
}
