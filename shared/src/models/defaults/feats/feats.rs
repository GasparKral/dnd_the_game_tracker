use crate::{
    api_types::catalog::{ChoiceSchema, SelectOption},
    models::character::{
        ArmorProf, DamageKind, Player, SavingThrowProf, SkillProf,
        SpecialTrait, WeaponProf,
    },
    traits::feat::simple_feat,
};

// ===========================================================================
// DOTES DE COMBATE
// ===========================================================================

#[derive(Debug)] pub struct Alert;
simple_feat!(Alert, "alert", "Alerta",
    description: "+5 a iniciativa. No puedes ser sorprendido. Las criaturas ocultas no tienen ventaja al atacarte.",
    traits_preview: ["Sin Sorpresa", "+5 Iniciativa"],
    traits_detail: [
        ("+5 Iniciativa",  "Tu bono de iniciativa aumenta permanentemente en 5."),
        ("Sin Sorpresa",   "No puedes ser sorprendido mientras estés consciente."),
        ("Sin Ventaja Ocultos", "Las criaturas ocultas no obtienen ventaja en ataques contra ti."),
    ],
);
impl Alert { pub fn apply_effect(c: &mut Player) {
    c.entity.iniciative = c.entity.iniciative.saturating_add(5);
    c.add_trait(SpecialTrait { id: "alert", name: "Alerta",
        description: "+5 iniciativa; no sorprendido; ocultos sin ventaja." });
}}

#[derive(Debug)] pub struct Charger;
simple_feat!(Charger, "charger", "Cargador",
    description: "Tras mover 10ft en línea recta y atacar: +1d8 daño extra o empujar 10ft al objetivo.",
    traits_preview: ["Carga +1d8", "Empuje 3m"],
    traits_detail: [
        ("Carga +1d8",  "Si mueves al menos 10 ft en línea recta antes de atacar, el ataque inflige +1d8."),
        ("Empuje 3m",   "Puedes empujar al objetivo 10 ft en lugar del daño extra."),
    ],
);

#[derive(Debug)] pub struct CrossbowExpert;
simple_feat!(CrossbowExpert, "crossbow_expert", "Experto en Ballesta",
    description: "Ignorar recarga en ballestas. Sin desventaja CaC. Ataque de bonificación con ballesta de mano.",
    traits_preview: ["Sin Recarga", "Ataque Bonus Ballesta"],
    traits_detail: [
        ("Sin Recarga",           "Ignoras la propiedad Recarga en las ballestas."),
        ("Sin Desventaja CaC",    "No tienes desventaja al atacar a distancia con criaturas adyacentes."),
        ("Ataque Bonus Ballesta", "Puedes hacer un ataque adicional con una ballesta de mano como acción adicional."),
    ],
);

#[derive(Debug)] pub struct DefensiveDuelist;
simple_feat!(DefensiveDuelist, "defensive_duelist", "Duelista Defensivo",
    description: "Req: Des 13. Reacción al ser atacado CaC con arma de finura: añadir bono de prof a CA contra ese ataque.",
    traits_preview: ["Req: Des 13", "Reacción +CA"],
    traits_detail: [
        ("Reacción Defensiva", "Como reacción, añades tu bono de proficiencia a la CA contra un ataque cuerpo a cuerpo."),
    ],
);

#[derive(Debug)] pub struct DualWielder;
simple_feat!(DualWielder, "dual_wielder", "Combatiente con Dos Armas",
    description: "+1 CA al empuñar dos armas. Usar armas no ligeras a dos manos. Desenvainar/envainar dos a la vez.",
    traits_preview: ["+1 CA", "Armas No Ligeras"],
    traits_detail: [
        ("+1 CA",             "+1 de bonus a CA mientras empuñas un arma en cada mano."),
        ("Armas No Ligeras", "Puedes usar armas de una mano no ligeras en la lucha con dos armas."),
        ("Doble Desenvaine",  "Puedes desenvainar o envainar dos armas a la vez cuando normalmente solo harías una."),
    ],
);

#[derive(Debug)] pub struct GreatWeaponMaster;
simple_feat!(GreatWeaponMaster, "great_weapon_master", "Gran Maestro de Armas",
    description: "Al derribar/crítico: ataque adicional de bonificación. Opción: -5 ataque / +10 daño con armas pesadas.",
    traits_preview: ["Ataque Extra", "-5/+10"],
    traits_detail: [
        ("Ataque Extra tras Crítico", "Cuando derrибas o logras un crítico, puedes atacar de nuevo como acción adicional."),
        ("-5/+10 Arma Pesada",        "Antes de atacar con arma pesada: -5 ataque para obtener +10 de daño si impactas."),
    ],
);

#[derive(Debug)] pub struct Grappler;
simple_feat!(Grappler, "grappler", "Luchador",
    description: "Req: Fue 13. Ventaja atacando a criaturas agarradas. Puedes inmovilizarlas.",
    traits_preview: ["Req: Fue 13", "Ventaja en Agarres"],
    traits_detail: [
        ("Ventaja vs Agarrados",   "Tienes ventaja en ataques contra criaturas que tienes agarradas."),
        ("Inmovilizar Agarrados",  "Como acción, puedes intentar inmovilizar a una criatura que ya tienes agarrada."),
    ],
);

#[derive(Debug)] pub struct MageSlayer;
simple_feat!(MageSlayer, "mage_slayer", "Asesino de Magos",
    description: "Reacción: atacar al lanzador adyacente. Desventaja en concentración. Ventaja en ST vs criaturas adyacentes.",
    traits_preview: ["Reacción vs Conjuros", "Anticoncentración"],
    traits_detail: [
        ("Reacción Anticasting",  "Reacción: atacar a una criatura adyacente cuando lanza un conjuro."),
        ("Desventaja Concentración", "Las criaturas a las que atacas tienen desventaja en sus tiradas de concentración."),
        ("Ventaja ST Adyacentes", "Ventaja en salvaciones contra conjuros de criaturas a menos de 5 ft."),
    ],
);

#[derive(Debug)] pub struct MountedCombatant;
simple_feat!(MountedCombatant, "mounted_combatant", "Combatiente Montado",
    description: "Ventaja CaC vs criaturas sin montura de talla menor. Proteger montura. Redirigir ataques.",
    traits_preview: ["Ventaja Montado", "Proteger Montura"],
    traits_detail: [
        ("Ventaja Montado",     "Ventaja en ataques CaC vs criaturas sin montura más pequeñas que tu montura."),
        ("Redirigir Ataque",    "Puedes hacer que un ataque dirigido a tu montura te impacte a ti en su lugar."),
        ("Salto Acrobático",    "Si tu montura falla una ST de Destreza, puedes saltar a un espacio adyacente y el daño te afecta a ti en lugar de a ella."),
    ],
);

#[derive(Debug)] pub struct PolearmMaster;
simple_feat!(PolearmMaster, "polearm_master", "Maestro de Armas de Asta",
    description: "Ataque de bonificación con extremo del asta (1d4 contundente). AO cuando una criatura entra a tu alcance.",
    traits_preview: ["Ataque 1d4 Extra", "AO al Acercarse"],
    traits_detail: [
        ("Extremo del Asta",   "Acción adicional: ataque con el extremo romo del asta por 1d4 + mod FUE contundente."),
        ("AO al Entrar",       "Puedes hacer un ataque de oportunidad cuando una criatura entra a tu alcance."),
    ],
);

#[derive(Debug)] pub struct SavageAttacker;
simple_feat!(SavageAttacker, "savage_attacker", "Atacante Salvaje",
    description: "1/turno: lanzas dos veces los dados de daño de un ataque CaC y usas el resultado mayor.",
    traits_preview: ["Relanzar Daño 1/turno"],
    traits_detail: [
        ("Relanzar Daño", "Una vez por turno, puedes tirar los dados de daño de arma dos veces y quedarte el mayor."),
    ],
);

#[derive(Debug)] pub struct Sentinel;
simple_feat!(Sentinel, "sentinel", "Centinela",
    description: "AO reduce velocidad a 0. AO vs criaturas que atacan aliados. Desengancharse no te detiene.",
    traits_preview: ["AO Detiene", "Guardia de Aliados"],
    traits_detail: [
        ("AO Detiene",          "Tus ataques de oportunidad reducen la velocidad del objetivo a 0 hasta el final de su turno."),
        ("Guardia de Aliados",  "Puedes hacer un AO contra criaturas que atacan a aliados adyacentes a ti."),
        ("Sin Desvío Seguro",   "Las criaturas no pueden usar la acción Desengancharse para evitar tus AO."),
    ],
);

#[derive(Debug)] pub struct Sharpshooter;
simple_feat!(Sharpshooter, "sharpshooter", "Francotirador",
    description: "Sin desventaja rango largo. Ignorar cobertura media y 3/4. Opción: -5 ataque / +10 daño.",
    traits_preview: ["Rango Largo", "-5/+10 Distancia"],
    traits_detail: [
        ("Rango Largo",       "No tienes desventaja en ataques a distancia contra objetivos en alcance largo."),
        ("Ignorar Cobertura", "Los ataques con armas a distancia ignoran la cobertura media y los 3/4."),
        ("-5/+10 Distancia", "Antes de atacar con arma a distancia: -5 ataque para ganar +10 daño si impactas."),
    ],
);

#[derive(Debug)] pub struct ShieldMaster;
simple_feat!(ShieldMaster, "shield_master", "Maestro del Escudo",
    description: "Empujar como acción adicional tras atacar. CA de escudo a ST Des. Sin daño si superas ST.",
    traits_preview: ["Empuje Bonus", "CA a ST Des"],
    traits_detail: [
        ("Empuje de Escudo",   "Si atacas en tu turno, puedes usar acción adicional para empujar con el escudo."),
        ("Escudo a Salvación", "Añades el bonus de CA del escudo a tus salvaciones de Destreza."),
        ("Sin Daño al Superar","Si fallas una ST de Des pero la hubieras superado sin el escudo, no recibes daño."),
    ],
);

#[derive(Debug)] pub struct SpellSniper;
simple_feat!(SpellSniper, "spell_sniper", "Franco Conjurador",
    description: "Doblar alcance de conjuros con tirada de ataque. Ignorar cobertura parcial. Aprender un truco.",
    traits_preview: ["Alcance x2", "Ignorar Cobertura"],
    traits_detail: [
        ("Alcance Doble",      "Los conjuros con tirada de ataque tienen el doble de alcance."),
        ("Ignorar Cobertura",  "Ignoras cobertura media y 3/4 con conjuros de ataque."),
        ("Truco de Ataque",    "Aprendes un truco de ataque de cualquier lista de conjuros."),
    ],
);

#[derive(Debug)] pub struct TavernBrawler;
simple_feat!(TavernBrawler, "tavern_brawler", "Bravucón de Taberna",
    description: "Golpes desarmados y objetos improvisados usan d4. Acción adicional: agarrar tras golpe.",
    traits_preview: ["Puño 1d4", "Agarrar Bonus"],
    traits_detail: [
        ("Puño 1d4",         "Tus ataques desarmados y con objetos improvisados infligen 1d4 + mod FUE."),
        ("Agarrar Bonus",    "Tras un golpe desarmado o con objeto, puedes intentar agarrar como acción adicional."),
    ],
);

#[derive(Debug)] pub struct WarCaster;
simple_feat!(WarCaster, "war_caster", "Lanzador de Guerra",
    description: "Req: lanzar conjuros. Ventaja en concentración. Componentes somáticos con manos ocupadas. Conjuro como AO.",
    traits_preview: ["Req: Conjuros", "Concentración Estable"],
    traits_detail: [
        ("Ventaja Concentración",   "Ventaja en ST de CON para mantener concentración al recibir daño."),
        ("Somático Sin Manos",      "Puedes realizar componentes somáticos aunque tengas las manos ocupadas."),
        ("Conjuro como AO",         "Puedes lanzar un conjuro con tiempo de lanzamiento de 1 acción como ataque de oportunidad."),
    ],
);

#[derive(Debug)] pub struct WeaponMaster;
simple_feat!(WeaponMaster, "weapon_master", "Maestro de Armas",
    description: "+1 Fue o Des (máx 20). Proficiencia en 4 armas a elegir.",
    traits_preview: ["+1 Fue/Des", "4 Proficiencias"],
    traits_detail: [
        ("+1 Atributo",       "+1 a Fuerza o Destreza (máx 20), a elegir al tomar el don."),
        ("4 Proficiencias",   "Ganas proficiencia con 4 armas a tu elección."),
    ],
);

// ===========================================================================
// DOTES DE HABILIDAD Y EXPLORACIÓN
// ===========================================================================

#[derive(Debug)] pub struct Actor;
simple_feat!(Actor, "actor", "Actor",
    description: "+1 Car (máx 20). Ventaja al suplantar identidades. Imitar voces.",
    traits_preview: ["+1 Carisma", "Suplantación"],
    traits_detail: [
        ("+1 Carisma",      "+1 a Carisma (máx 20)."),
        ("Suplantación",    "Ventaja en Engaño e Interpretación al hacerse pasar por otra persona."),
        ("Imitar Voces",    "Puedes imitar voces de criaturas que hayas oído al menos 1 minuto."),
    ],
);

#[derive(Debug)] pub struct Athlete;
simple_feat!(Athlete, "athlete", "Atleta",
    description: "+1 Fue o Des (máx 20). Levantarse cuesta 5ft. Trepar sin penalización.",
    traits_preview: ["+1 Fue/Des", "Trepar"],
    traits_detail: [
        ("+1 Fue/Des",       "+1 a Fuerza o Destreza (máx 20), a elegir."),
        ("Levantarse Rápido","Ponerse en pie cuesta solo 5 ft de movimiento."),
        ("Trepar Libre",     "Trepar no tiene coste adicional de movimiento."),
    ],
);

#[derive(Debug)] pub struct DungeonDelver;
simple_feat!(DungeonDelver, "dungeon_delver", "Explorador de Mazmorras",
    description: "Ventaja en detectar puertas secretas y trampas. Resistencia a daño de trampas.",
    traits_preview: ["Detectar Trampas", "Resistencia Trampas"],
    traits_detail: [
        ("Detectar Secretos",  "Ventaja en Percepción e Investigación para detectar puertas secretas y trampas."),
        ("ST vs Trampas",      "Ventaja en salvaciones contra trampas."),
        ("Resistencia Trampa", "Resistencia al daño infligido por trampas."),
        ("Sin Lentitud",       "Puedes buscar trampas a velocidad normal sin penalización."),
    ],
);

#[derive(Debug)] pub struct Durable;
simple_feat!(Durable, "durable", "Resistente",
    description: "+1 Con (máx 20). Al gastar Dado de Golpe en desc. corto: recuperar mínimo 2×mod.Con.",
    traits_preview: ["+1 Constitución", "Dados de Golpe Mejorados"],
    traits_detail: [
        ("+1 Constitución",   "+1 a Constitución (máx 20)."),
        ("Dados de Golpe",    "Al gastar un Dado de Golpe en descanso corto, recuperas al menos 2 × tu mod. de Constitución."),
    ],
);

#[derive(Debug)] pub struct HeavilyArmored;
simple_feat!(HeavilyArmored, "heavily_armored", "Armadura Pesada",
    description: "Req: prof. armadura media. +1 Fue (máx 20). Proficiencia con armaduras pesadas.",
    traits_preview: ["Req: Armadura Media", "Prof. Armadura Pesada"],
    traits_detail: [
        ("+1 Fuerza",          "+1 a Fuerza (máx 20)."),
        ("Prof. Pesada",       "Ganas proficiencia con armaduras pesadas."),
    ],
);
impl HeavilyArmored { pub fn apply_effect(c: &mut Player) {
    c.add_armor_prof(ArmorProf::Heavy);
}}

#[derive(Debug)] pub struct HeavyArmorMaster;
simple_feat!(HeavyArmorMaster, "heavy_armor_master", "Maestro de Armadura Pesada",
    description: "Req: prof. armadura pesada. +1 Fue. Con armadura pesada: reducir daño no mágico en 3.",
    traits_preview: ["Req: Armadura Pesada", "Reducción de Daño 3"],
    traits_detail: [
        ("+1 Fuerza",        "+1 a Fuerza (máx 20)."),
        ("Reducción 3",      "Con armadura pesada, reduces en 3 el daño no mágico cortante, perforante y contundente."),
    ],
);

#[derive(Debug)] pub struct InspiringLeader;
simple_feat!(InspiringLeader, "inspiring_leader", "Líder Inspirador",
    description: "Req: Car 13. Discurso 10min: hasta 6 criaturas ganan PG temporales = nivel + mod CAR.",
    traits_preview: ["Req: Car 13", "PG Temporales"],
    traits_detail: [
        ("Discurso Inspirador", "Inviertes 10 min en un discurso. Hasta 6 criaturas (incluido tú) ganan PG temporales = nivel + mod CAR. Se recarga con descanso corto o largo."),
    ],
);

#[derive(Debug)] pub struct KeenMind;
simple_feat!(KeenMind, "keen_mind", "Mente Aguda",
    description: "+1 Int (máx 20). Siempre sabes norte/hora/días. Memoria de 30 días.",
    traits_preview: ["+1 Inteligencia", "Memoria Perfecta"],
    traits_detail: [
        ("+1 Inteligencia", "+1 a Inteligencia (máx 20)."),
        ("Orientación",     "Siempre sabes qué dirección es el norte, cuántas horas faltan para el amanecer/ocaso y cuántos días han pasado."),
        ("Memoria 30 Días", "Puedes recordar con detalle todo lo que hayas visto u oído en el último mes."),
    ],
);

#[derive(Debug)] pub struct LightlyArmored;
simple_feat!(LightlyArmored, "lightly_armored", "Armadura Ligera",
    description: "+1 Fue o Des (máx 20). Proficiencia con armaduras ligeras y escudo.",
    traits_preview: ["+1 Fue/Des", "Prof. Armadura Ligera"],
    traits_detail: [
        ("+1 Fue/Des",      "+1 a Fuerza o Destreza (máx 20), a elegir."),
        ("Prof. Ligera",    "Ganas proficiencia con armaduras ligeras."),
        ("Prof. Escudo",    "Ganas proficiencia con escudos."),
    ],
);
impl LightlyArmored { pub fn apply_effect(c: &mut Player) {
    c.add_armor_prof(ArmorProf::Light);
    c.add_armor_prof(ArmorProf::Shield);
}}

#[derive(Debug)] pub struct Lucky;
simple_feat!(Lucky, "lucky", "Afortunado",
    description: "3 puntos de suerte/desc.largo. Tirada extra en cualquier tirada propia o del rival.",
    traits_preview: ["3 Puntos de Suerte"],
    traits_detail: [
        ("Puntos de Suerte", "3 puntos por descanso largo. Gastar 1: tira d20 extra en ataque, prueba o ST (tuya o del rival), elige cuál usar."),
    ],
);

#[derive(Debug)] pub struct MartialAdept;
simple_feat!(MartialAdept, "martial_adept", "Adepto Marcial",
    description: "Aprende 2 maniobras de guerrero. Ganas 1 dado de superioridad d6 (se recupera en desc.).",
    traits_preview: ["2 Maniobras", "Dado d6"],
    traits_detail: [
        ("2 Maniobras",     "Aprendes 2 maniobras de la lista del Maestro de Batalla."),
        ("Dado d6",         "Ganas 1 dado de superioridad d6. Se recupera en descanso corto o largo."),
    ],
);

#[derive(Debug)] pub struct MediumArmorMaster;
simple_feat!(MediumArmorMaster, "medium_armor_master", "Maestro de Armadura Media",
    description: "Req: armadura media. Sin desventaja Sigilo. DES máximo +3 a CA en lugar de +2.",
    traits_preview: ["Req: Armadura Media", "Sigilo + Des +3"],
    traits_detail: [
        ("Sin Desventaja Sigilo", "Llevar armadura media no impone desventaja en Sigilo."),
        ("Des +3 a CA",           "Con armadura media puedes añadir hasta +3 (no +2) de tu mod. DES a la CA."),
    ],
);

#[derive(Debug)] pub struct Mobile;
simple_feat!(Mobile, "mobile", "Móvil",
    description: "+3m velocidad. Al Cargar no hay terreno difícil. No provocas AO de criaturas que atacas.",
    traits_preview: ["+3m Velocidad", "Sin AO Atacados"],
    traits_detail: [
        ("+3m Velocidad",   "Tu velocidad aumenta permanentemente en 3 metros."),
        ("Carga Libre",     "Cuando usas la acción Cargar, el terreno difícil no te penaliza en ese movimiento."),
        ("Sin AO",          "En el turno en que atacas a una criatura, no provoca ataques de oportunidad de esa criatura."),
    ],
);
impl Mobile { pub fn apply_effect(c: &mut Player) {
    c.entity.speed = c.entity.speed.saturating_add(3);
    c.add_trait(SpecialTrait { id: "mobile", name: "Móvil",
        description: "+3m velocidad; Cargar sin terreno difícil; sin AO de objetivos atacados." });
}}

#[derive(Debug)] pub struct ModeratelyArmored;
simple_feat!(ModeratelyArmored, "moderately_armored", "Armadura Media",
    description: "Req: armadura ligera. +1 Fue o Des (máx 20). Proficiencia con armadura media y escudo.",
    traits_preview: ["Req: Armadura Ligera", "Prof. Armadura Media"],
    traits_detail: [
        ("+1 Fue/Des",     "+1 a Fuerza o Destreza (máx 20), a elegir."),
        ("Prof. Media",    "Ganas proficiencia con armaduras medias."),
        ("Prof. Escudo",   "Ganas proficiencia con escudos."),
    ],
);
impl ModeratelyArmored { pub fn apply_effect(c: &mut Player) {
    c.add_armor_prof(ArmorProf::Medium);
    c.add_armor_prof(ArmorProf::Shield);
}}

#[derive(Debug)] pub struct Observant;
simple_feat!(Observant, "observant", "Observador",
    description: "+1 Int o Sab (máx 20). Leer labios. +5 a Percepción e Investigación pasivas.",
    traits_preview: ["+1 Int/Sab", "+5 Percepción Pasiva"],
    traits_detail: [
        ("+1 Int/Sab",         "+1 a Inteligencia o Sabiduría (máx 20), a elegir."),
        ("Leer Labios",        "Si puedes ver la boca de una criatura, puedes entender lo que dice sin oírla."),
        ("+5 Percep. Pasiva",  "+5 a tu puntuación de Percepción pasiva e Investigación pasiva."),
    ],
);
impl Observant { pub fn apply_effect(c: &mut Player) {
    c.entity.perceptions = c.entity.perceptions.saturating_add(5);
}}

#[derive(Debug)] pub struct Resilient;
simple_feat!(Resilient, "resilient", "Tenaz",
    description: "+1 al atributo elegido (máx 20). Proficiencia en salvaciones de ese atributo.",
    traits_preview: ["+1 Atributo", "Nueva Salvación"],
    traits_detail: [
        ("+1 Atributo",    "+1 al atributo elegido (máx 20)."),
        ("Nueva Salvación","Ganas proficiencia en las salvaciones del atributo elegido."),
    ],
    choices: [
        ChoiceSchema::SingleSelect {
            id: "resilient.ability".into(),
            label: "Atributo para salvación".into(),
            options: vec![
                SelectOption::bare("str", "Fuerza"),
                SelectOption::bare("dex", "Destreza"),
                SelectOption::bare("con", "Constitución"),
                SelectOption::bare("int", "Inteligencia"),
                SelectOption::bare("wis", "Sabiduría"),
                SelectOption::bare("cha", "Carisma"),
            ],
        }
    ],
    required_choices: ["resilient.ability"],
);

#[derive(Debug)] pub struct Skilled;
simple_feat!(Skilled, "skilled", "Hábil",
    description: "Ganas proficiencia en 3 habilidades o herramientas a tu elección.",
    traits_preview: ["3 Proficiencias"],
    traits_detail: [
        ("3 Proficiencias", "Elige 3 habilidades o herramientas. Ganas proficiencia en cada una."),
    ],
);

#[derive(Debug)] pub struct Skulker;
simple_feat!(Skulker, "skulker", "Merodeador",
    description: "Req: Des 13. Esconderse en luz tenue. Fallar ataque no revela posición. Sin penalización en oscuridad leve.",
    traits_preview: ["Req: Des 13", "Sigilo Mejorado"],
    traits_detail: [
        ("Esconderse en Penumbra",  "Puedes intentar ocultarte cuando solo estás ligeramente oscurecido."),
        ("Posición Oculta",         "Fallar un ataque a distancia no revela tu posición."),
        ("Sin Penalización Oscuridad", "No tienes desventaja en Percepción en penumbra."),
    ],
);

#[derive(Debug)] pub struct Tough;
simple_feat!(Tough, "tough", "Fornido",
    description: "+2 PG máximos por nivel (incluyendo niveles pasados y futuros).",
    traits_preview: ["+2 PG por Nivel"],
    traits_detail: [
        ("+2 PG/Nivel", "Tus PG máximos aumentan en 2 por cada nivel que tengas o vayas adquiriendo."),
    ],
);

// ===========================================================================
// DOTES MÁGICOS
// ===========================================================================

#[derive(Debug)] pub struct ElementalAdept;
simple_feat!(ElementalAdept, "elemental_adept", "Adepto Elemental",
    description: "Req: conjuros. Ignorar resistencia al tipo de daño elegido. Los 1s en dados de daño cuentan como 2s.",
    traits_preview: ["Req: Conjuros", "Ignorar Resistencia"],
    traits_detail: [
        ("Ignorar Resistencia", "Tus conjuros del tipo de daño elegido ignoran la resistencia de las criaturas."),
        ("1s = 2s",             "Cuando tiras dados de daño de ese tipo, tratas los resultados de 1 como si fueran 2."),
    ],
    choices: [
        ChoiceSchema::SingleSelect {
            id: "elemental_adept.damage_type".into(),
            label: "Tipo de daño elemental".into(),
            options: vec![
                SelectOption::bare("acid",      "Ácido"),
                SelectOption::bare("cold",      "Frío"),
                SelectOption::bare("fire",      "Fuego"),
                SelectOption::bare("lightning", "Relámpago"),
                SelectOption::bare("thunder",   "Trueno"),
            ],
        }
    ],
    required_choices: ["elemental_adept.damage_type"],
);

#[derive(Debug)] pub struct Healer;
simple_feat!(Healer, "healer", "Sanador",
    description: "Estabilizar gratis con botiquín. Curar 1d6+4+DG 1 vez por criatura por descanso.",
    traits_preview: ["Estabilizar Gratis", "Botiquín Mejorado"],
    traits_detail: [
        ("Estabilizar Gratis", "Usar un botiquín de sanador para estabilizar a alguien a 0 PG no requiere tirada."),
        ("Curación Mejorada",  "Gastar un botiquín: objetivo recupera 1d6 + 4 + máx dados de golpe PG. 1 vez por criatura por descanso."),
    ],
);

#[derive(Debug)] pub struct MagicInitiate;
simple_feat!(MagicInitiate, "magic_initiate", "Iniciado en la Magia",
    description: "Elige clase: 2 trucos + 1 conjuro nv1. Lanzar el conjuro 1/día sin espacio.",
    traits_preview: ["2 Trucos", "1 Conjuro Gratis"],
    traits_detail: [
        ("2 Trucos",          "Aprendes 2 trucos de la lista de la clase elegida."),
        ("1 Conjuro Gratis",  "Aprendes 1 conjuro de nivel 1 de esa lista. Puedes lanzarlo una vez por descanso largo sin gastar espacio."),
    ],
    choices: [
        ChoiceSchema::SingleSelect {
            id: "magic_initiate.class".into(),
            label: "Clase de la que aprenderás".into(),
            options: vec![
                SelectOption::bare("bard",     "Bardo"),
                SelectOption::bare("cleric",   "Clérigo"),
                SelectOption::bare("druid",    "Druida"),
                SelectOption::bare("sorcerer", "Hechicero"),
                SelectOption::bare("warlock",  "Brujo"),
                SelectOption::bare("wizard",   "Mago"),
            ],
        }
    ],
    required_choices: ["magic_initiate.class"],
);

#[derive(Debug)] pub struct RitualCaster;
simple_feat!(RitualCaster, "ritual_caster", "Lanzador de Rituales",
    description: "Req: Int o Sab 13. Libro de rituales con 2 conjuros nv1. Puedes copiar más rituales.",
    traits_preview: ["Req: Int/Sab 13", "Libro de Rituales"],
    traits_detail: [
        ("Libro de Rituales", "Empieza con 2 conjuros de ritual de nivel 1 de la clase elegida. Puedes copiar más rituales al libro."),
        ("Lanzar Rituales",   "Puedes lanzar esos conjuros como rituales (10 min extra, sin gastar espacio)."),
    ],
    choices: [
        ChoiceSchema::SingleSelect {
            id: "ritual_caster.class".into(),
            label: "Clase para rituales".into(),
            options: vec![
                SelectOption::bare("bard",   "Bardo"),
                SelectOption::bare("cleric", "Clérigo"),
                SelectOption::bare("druid",  "Druida"),
                SelectOption::bare("wizard", "Mago"),
            ],
        }
    ],
    required_choices: ["ritual_caster.class"],
);
