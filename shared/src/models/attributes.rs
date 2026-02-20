use super::dice::DiceRoll;

#[derive(Debug, Clone, Hash)]
pub struct HitPoints {
    pub actual: u16,
    pub temporals: u16,
    pub max: u16,
    pub hit_dice: DiceRoll,
}

#[derive(Debug, Clone, Hash)]
pub enum SkillKind {
    Atletic,
    Acrobatics,
    HandsGame,
    Stealth,
    Arcane,
    History,
    Investigation,
    Naturalist,
    Religion,
    AnimalTreatment,
    Insight,
    Medicine,
    Perception,
    Supervivence,
    Trickness,
    Intimidation,
    Interpretation,
    Persuasion,
    Salvation,
}

#[derive(Debug, Clone, Hash)]
pub struct Skill {
    pub kind: SkillKind,
    pub proeficient: bool,
    pub master: bool,
    pub modifier: u32,
}

#[derive(Debug, Clone, Hash)]
pub enum AttributeType {
    Strength,
    Dexterity,
    Constitution,
    Inteligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, Clone, Hash)]
pub struct Attribute {
    pub _type: AttributeType,
    pub value: u32,
    pub modifier: u32,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Hash)]
pub struct MagicStats {
    pub magical_aptitude: AttributeType,
    pub magic_mod: i8,
    pub magic_cd: i8,
    pub incantation_bonus: i8,
}
