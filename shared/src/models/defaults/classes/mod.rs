use crate::models::character::Player;
use crate::traits::class::Class;

// ── Bárbaro ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Barbarian;
impl Class for Barbarian {
    fn id(&self) -> &'static str { "barbarian" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Bardo ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Bard;
impl Class for Bard {
    fn id(&self) -> &'static str { "bard" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Clérigo ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Cleric;
impl Class for Cleric {
    fn id(&self) -> &'static str { "cleric" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Druida ────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Druid;
impl Class for Druid {
    fn id(&self) -> &'static str { "druid" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Guerrero ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Fighter;
impl Class for Fighter {
    fn id(&self) -> &'static str { "fighter" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Monje ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Monk;
impl Class for Monk {
    fn id(&self) -> &'static str { "monk" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Paladín ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Paladin;
impl Class for Paladin {
    fn id(&self) -> &'static str { "paladin" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Explorador ────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Ranger;
impl Class for Ranger {
    fn id(&self) -> &'static str { "ranger" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Pícaro ────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Rogue;
impl Class for Rogue {
    fn id(&self) -> &'static str { "rogue" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Hechicero ─────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Sorcerer;
impl Class for Sorcerer {
    fn id(&self) -> &'static str { "sorcerer" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Brujo ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Warlock;
impl Class for Warlock {
    fn id(&self) -> &'static str { "warlock" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Mago ──────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Wizard;
impl Class for Wizard {
    fn id(&self) -> &'static str { "wizard" }
    fn sub_class(&self) -> &'static str { "" }
    fn apply(&self, _character: &mut Player) {}
}
