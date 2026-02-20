#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct DiceRoll {
    pub count: u8,
    pub dice: Dice,
}

impl DiceRoll {
    pub fn roll_all(&self) -> Vec<u8> {
        self.dice.roll_many(self.count)
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
