use crate::models::character::Player;
use crate::traits::background::Background;

// ── Acólito ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Acolyte;
impl Background for Acolyte {
    fn id(&self) -> &'static str { "acolyte" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Artesano ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Artisan;
impl Background for Artisan {
    fn id(&self) -> &'static str { "artisan" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Charlatán ─────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Charlatan;
impl Background for Charlatan {
    fn id(&self) -> &'static str { "charlatan" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Criminal ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Criminal;
impl Background for Criminal {
    fn id(&self) -> &'static str { "criminal" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Erudito ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Scholar;
impl Background for Scholar {
    fn id(&self) -> &'static str { "scholar" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Guardabosques ─────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Guide;
impl Background for Guide {
    fn id(&self) -> &'static str { "guide" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Marinero ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Sailor;
impl Background for Sailor {
    fn id(&self) -> &'static str { "sailor" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Noble ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Noble;
impl Background for Noble {
    fn id(&self) -> &'static str { "noble" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Soldado ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Soldier;
impl Background for Soldier {
    fn id(&self) -> &'static str { "soldier" }
    fn apply(&self, _character: &mut Player) {}
}

// ── Ermitaño ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Hermit;
impl Background for Hermit {
    fn id(&self) -> &'static str { "hermit" }
    fn apply(&self, _character: &mut Player) {}
}
