use crate::traits::feat::Feat;

// ---------------------------------------------------------------------------
// Dotes de Combate
// ---------------------------------------------------------------------------

#[derive(Debug)] pub struct Alert;
impl Feat for Alert {
    fn id(&self) -> &'static str { "alert" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +5 a iniciativa, no puedes ser sorprendido mientras estés consciente,
        // las criaturas ocultas no obtienen ventaja al atacarte.
    }
}

#[derive(Debug)] pub struct Charger;
impl Feat for Charger {
    fn id(&self) -> &'static str { "charger" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Tras Cargar (mover 10ft en línea recta), ataque bonificado +1d8
        // o empujar al objetivo 10ft.
    }
}

#[derive(Debug)] pub struct CrossbowExpert;
impl Feat for CrossbowExpert {
    fn id(&self) -> &'static str { "crossbow_expert" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Ignorar el coste de acción de recargar, sin desventaja cuerpo a cuerpo,
        // ataque extra de ballesta de mano como acción adicional.
    }
}

#[derive(Debug)] pub struct DefensiveDuelist;
impl Feat for DefensiveDuelist {
    fn id(&self) -> &'static str { "defensive_duelist" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: Des 13. Reacción: +bono de proficiencia a CA vs un ataque
        // cuerpo a cuerpo mientras empuñas un arma de finura.
    }
}

#[derive(Debug)] pub struct DualWielder;
impl Feat for DualWielder {
    fn id(&self) -> &'static str { "dual_wielder" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 CA al empuñar dos armas, usar armas de una mano no ligeras
        // para lucha con dos armas, desenvainar/envainar dos armas a la vez.
    }
}

#[derive(Debug)] pub struct GreatWeaponMaster;
impl Feat for GreatWeaponMaster {
    fn id(&self) -> &'static str { "great_weapon_master" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Al derribar o lograr crítico: ataque adicional de bonificación.
        // Opción: -5 ataque / +10 daño con armas pesadas.
    }
}

#[derive(Debug)] pub struct Grappler;
impl Feat for Grappler {
    fn id(&self) -> &'static str { "grappler" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: Fue 13. Ventaja en ataques vs criatura que agarras,
        // puedes intentar inmovilizar a una criatura agarrada.
    }
}

#[derive(Debug)] pub struct MageSlayer;
impl Feat for MageSlayer {
    fn id(&self) -> &'static str { "mage_slayer" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Reacción: atacar a lanzador adyacente. Desventaja en CD de concentración
        // para criaturas que atacas, ventaja en salvaciones vs conjuros de criaturas
        // adyacentes.
    }
}

#[derive(Debug)] pub struct MountedCombatant;
impl Feat for MountedCombatant {
    fn id(&self) -> &'static str { "mounted_combatant" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Ventaja en ataques cuerpo a cuerpo vs criaturas menores que tu montura,
        // redirigir ataques a la montura hacia ti, montura evita daño con
        // salvación exitosa (mitad en fallo).
    }
}

#[derive(Debug)] pub struct PolearmMaster;
impl Feat for PolearmMaster {
    fn id(&self) -> &'static str { "polearm_master" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Ataque de bonificación con extremo opuesto (1d4 contundente).
        // Reacción: atacar a criaturas que entran en tu alcance de 10ft.
    }
}

#[derive(Debug)] pub struct SavageAttacker;
impl Feat for SavageAttacker {
    fn id(&self) -> &'static str { "savage_attacker" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Una vez por turno al atacar con arma cuerpo a cuerpo:
        // tirar dados de daño dos veces y usar el resultado mayor.
    }
}

#[derive(Debug)] pub struct Sentinel;
impl Feat for Sentinel {
    fn id(&self) -> &'static str { "sentinel" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Ataques de oportunidad reducen velocidad a 0.
        // Ataque de reacción vs criaturas que atacan a otros adyacentes.
        // Sin desventaja por alejarse con Desvío.
    }
}

#[derive(Debug)] pub struct Sharpshooter;
impl Feat for Sharpshooter {
    fn id(&self) -> &'static str { "sharpshooter" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Sin desventaja en alcance largo, ignorar cobertura media y 3/4,
        // opción: -5 ataque / +10 daño con armas a distancia.
    }
}

#[derive(Debug)] pub struct ShieldMaster;
impl Feat for ShieldMaster {
    fn id(&self) -> &'static str { "shield_master" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Acción adicional: empujar con escudo. Añadir CA de escudo a
        // salvaciones de Des. Si fallas salvación de Des: sin daño si pasas.
    }
}

#[derive(Debug)] pub struct SpellSniper;
impl Feat for SpellSniper {
    fn id(&self) -> &'static str { "spell_sniper" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Doblar alcance de conjuros con tirada de ataque, ignorar
        // cobertura media y 3/4, aprender un truco de ataque.
    }
}

#[derive(Debug)] pub struct TavernBrawler;
impl Feat for TavernBrawler {
    fn id(&self) -> &'static str { "tavern_brawler" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Proficiencia en golpes desarmados (1d4), objetos improvisados como armas.
        // Acción adicional: intentar agarrar tras golpe desarmado o con objeto.
    }
}

#[derive(Debug)] pub struct WarCaster;
impl Feat for WarCaster {
    fn id(&self) -> &'static str { "war_caster" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: capacidad de lanzar al menos un conjuro. Ventaja en CD de
        // concentración, componentes somáticos con manos ocupadas,
        // lanzar conjuro como ataque de oportunidad.
    }
}

#[derive(Debug)] pub struct WeaponMaster;
impl Feat for WeaponMaster {
    fn id(&self) -> &'static str { "weapon_master" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 a Fue o Des (máx 20), proficiencia en 4 armas a elegir.
    }
}

// ---------------------------------------------------------------------------
// Dotes de Habilidad y Exploración
// ---------------------------------------------------------------------------

#[derive(Debug)] pub struct Athlete;
impl Feat for Athlete {
    fn id(&self) -> &'static str { "athlete" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 Fue o Des (máx 20), ponerse en pie cuesta solo 5ft de mov,
        // trepar sin penalización, carrera larga sin vuelta de atrás.
    }
}

#[derive(Debug)] pub struct Actor;
impl Feat for Actor {
    fn id(&self) -> &'static str { "actor" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 Car (máx 20), ventaja en Engaño/Interpretación al hacerse pasar
        // por otro, imitar voz de criatura oída al menos 1 minuto.
    }
}

#[derive(Debug)] pub struct DungeonDelver;
impl Feat for DungeonDelver {
    fn id(&self) -> &'static str { "dungeon_delver" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Ventaja en Percepción/Investigación para detectar puertas secretas,
        // ventaja en salvaciones vs trampas, resistencia a daño de trampas,
        // no reducir velocidad en terreno difícil al buscar trampas.
    }
}

#[derive(Debug)] pub struct Durable;
impl Feat for Durable {
    fn id(&self) -> &'static str { "durable" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 Con (máx 20). Al gastar Dado de Golpe en descanso corto:
        // recuperar mínimo 2 × mod. Con PG.
    }
}

#[derive(Debug)] pub struct HeavilyArmored;
impl Feat for HeavilyArmored {
    fn id(&self) -> &'static str { "heavily_armored" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: proficiencia con armadura media. +1 Fue (máx 20),
        // proficiencia con armaduras pesadas.
    }
}

#[derive(Debug)] pub struct HeavyArmorMaster;
impl Feat for HeavyArmorMaster {
    fn id(&self) -> &'static str { "heavy_armor_master" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: prof con armadura pesada. +1 Fue (máx 20).
        // Con armadura pesada: reducir daño no mágico contundente/perforante/cortante en 3.
    }
}

#[derive(Debug)] pub struct InspiringLeader;
impl Feat for InspiringLeader {
    fn id(&self) -> &'static str { "inspiring_leader" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: Car 13. Discurso de 10 min: hasta 6 criaturas ganan PG temp
        // igual a nivel + mod Car. Usable tras descanso corto/largo.
    }
}

#[derive(Debug)] pub struct KeenMind;
impl Feat for KeenMind {
    fn id(&self) -> &'static str { "keen_mind" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 Int (máx 20), siempre saber norte/hora/días transcurridos,
        // recordar todo lo visto/oído en el último mes.
    }
}

#[derive(Debug)] pub struct LightlyArmored;
impl Feat for LightlyArmored {
    fn id(&self) -> &'static str { "lightly_armored" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 Fue o Des (máx 20), proficiencia con armaduras ligeras.
    }
}

#[derive(Debug)] pub struct Lucky;
impl Feat for Lucky {
    fn id(&self) -> &'static str { "lucky" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // 3 puntos de suerte por día (recuperan en descanso largo).
        // Gastar 1: tirar d20 extra en ataque, prueba o salvación (elegir cuál usar),
        // o forzar a relanzar tirada de ataque vs ti.
    }
}

#[derive(Debug)] pub struct MartialAdept;
impl Feat for MartialAdept {
    fn id(&self) -> &'static str { "martial_adept" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Aprender 2 maniobras de Maestro de Batalla, ganar 1 dado de superioridad
        // d6 (se recupera en descanso corto/largo).
    }
}

#[derive(Debug)] pub struct MediumArmorMaster;
impl Feat for MediumArmorMaster {
    fn id(&self) -> &'static str { "medium_armor_master" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: prof con armadura media. Sin desventaja en Sigilo con armadura
        // media, Des máx +3 a CA en lugar de +2.
    }
}

#[derive(Debug)] pub struct Mobile;
impl Feat for Mobile {
    fn id(&self) -> &'static str { "mobile" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +10ft de velocidad, Cargar no provoca terreno difícil, no provocar
        // ataques de oportunidad de criaturas a las que atacas en el turno.
    }
}

#[derive(Debug)] pub struct ModeratelyArmored;
impl Feat for ModeratelyArmored {
    fn id(&self) -> &'static str { "moderately_armored" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: prof con armadura ligera. +1 Fue o Des (máx 20),
        // proficiencia con armadura media y escudo.
    }
}

#[derive(Debug)] pub struct Observant;
impl Feat for Observant {
    fn id(&self) -> &'static str { "observant" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +1 Int o Sab (máx 20), leer labios si ves la boca hablar,
        // +5 pasivo Percepción e Investigación.
    }
}

#[derive(Debug)] pub struct Resilient;
impl Feat for Resilient {
    fn id(&self) -> &'static str { "resilient" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Elegir un atributo: +1 (máx 20) y proficiencia en salvaciones
        // de ese atributo.
    }
}

#[derive(Debug)] pub struct Skilled;
impl Feat for Skilled {
    fn id(&self) -> &'static str { "skilled" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Proficiencia en 3 habilidades o herramientas a elegir.
    }
}

#[derive(Debug)] pub struct Skulker;
impl Feat for Skulker {
    fn id(&self) -> &'static str { "skulker" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: Des 13. Esconderse con visibilidad levemente oscurecida,
        // fallar ataque a distancia no revela posición, sin penalización
        // en Percepción en oscuridad leve.
    }
}

#[derive(Debug)] pub struct Tough;
impl Feat for Tough {
    fn id(&self) -> &'static str { "tough" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // +2 PG máx por nivel (pasado y futuro).
    }
}

// ---------------------------------------------------------------------------
// Dotes Mágicos
// ---------------------------------------------------------------------------

#[derive(Debug)] pub struct ElementalAdept;
impl Feat for ElementalAdept {
    fn id(&self) -> &'static str { "elemental_adept" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: lanzar al menos un conjuro. Elegir tipo de daño (ácido/frío/fuego/
        // relámpago/trueno): conjuros de ese tipo ignoran resistencia,
        // tratar 1s en dados de daño como 2s.
    }
}

#[derive(Debug)] pub struct Healer;
impl Feat for Healer {
    fn id(&self) -> &'static str { "healer" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Usar botiquín sin acción para estabilizar a 0 PG.
        // Curar con botiquín: recuperar 1d6 + 4 + max_dados_golpe PG
        // (una vez por criatura por descanso corto/largo).
    }
}

#[derive(Debug)] pub struct MagicInitiate;
impl Feat for MagicInitiate {
    fn id(&self) -> &'static str { "magic_initiate" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Elegir clase (Bardo/Clérigo/Druida/Hechicero/Brujo/Mago):
        // aprender 2 trucos y 1 conjuro de nivel 1 de esa lista.
        // Lanzar el conjuro de nivel 1 una vez/día gratis (también con espacios).
    }
}

#[derive(Debug)] pub struct RitualCaster;
impl Feat for RitualCaster {
    fn id(&self) -> &'static str { "ritual_caster" }
    fn apply(&self, _c: &mut crate::models::character::Player) {
        // Req: Int o Sab 13. Libro de rituales con 2 conjuros de ritual nivel 1.
        // Lanzarlos como ritual, copiar más rituales encontrados.
    }
}
