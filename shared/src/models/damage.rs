#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DamageType {
    Elemental(ElementalDamage),
    Physic(PhysicDamage),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ElementalDamage {
    Electric,
    Poison,
    Mental,
    Ice,
    Fire,
    Necrotic,
    Corrosive,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PhysicDamage {
    Cutting,
    Piercing,
    Stinging,
    Overwhelming,
}
