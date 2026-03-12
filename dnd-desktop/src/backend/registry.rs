use shared::api_types::catalog::CatalogResponse;
use std::collections::HashMap;
use tokio::sync::RwLock;

// ---------------------------------------------------------------------------
// Registro de entidades del dominio
// ---------------------------------------------------------------------------

/// Entrada del registro: metadata para la API + trait object para aplicar efectos.
pub struct RaceEntry {
    pub catalog: CatalogEntry,
    pub implementation: Box<dyn Race + Send + Sync>,
}

pub struct ClassEntry {
    pub catalog: CatalogEntry,
    pub implementation: Box<dyn Class + Send + Sync>,
}

pub struct BackgroundEntry {
    pub catalog: CatalogEntry,
    pub implementation: Box<dyn Background + Send + Sync>,
}

pub struct FeatEntry {
    pub catalog: CatalogEntry,
    pub implementation: Box<dyn Feat + Send + Sync>,
}

// ---------------------------------------------------------------------------
// Registry central
// ---------------------------------------------------------------------------

/// Almacena todas las razas, clases, trasfondos y dotes disponibles en runtime.
/// Se pobla al arrancar con los defaults del PHB y con homebrew cargado del vault.
#[derive(Debug, Default)]
pub struct Registry {
    races: RwLock<HashMap<String, RaceEntry>>,
    classes: RwLock<HashMap<String, ClassEntry>>,
    backgrounds: RwLock<HashMap<String, BackgroundEntry>>,
    feats: RwLock<HashMap<String, FeatEntry>>,
}

impl std::fmt::Debug for RaceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RaceEntry({})", self.catalog.id)
    }
}
impl std::fmt::Debug for ClassEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClassEntry({})", self.catalog.id)
    }
}
impl std::fmt::Debug for BackgroundEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BackgroundEntry({})", self.catalog.id)
    }
}
impl std::fmt::Debug for FeatEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FeatEntry({})", self.catalog.id)
    }
}

impl Registry {
    pub fn new() -> Self {
        let registry = Self::default();
        registry
    }

    // -----------------------------------------------------------------------
    // Races
    // -----------------------------------------------------------------------

    pub async fn register_race(&self, entry: RaceEntry) {
        self.races
            .write()
            .await
            .insert(entry.catalog.id.clone(), entry);
    }

    pub async fn get_race(&self, id: &str) -> Option<CatalogEntry> {
        self.races
            .read()
            .await
            .get(id)
            .map(|e| e.catalog.clone())
    }

    pub async fn races_catalog(&self) -> CatalogResponse {
        CatalogResponse {
            entries: self
                .races
                .read()
                .await
                .values()
                .map(|e| e.catalog.clone())
                .collect(),
        }
    }

    // -----------------------------------------------------------------------
    // Classes
    // -----------------------------------------------------------------------

    pub async fn register_class(&self, entry: ClassEntry) {
        self.classes
            .write()
            .await
            .insert(entry.catalog.id.clone(), entry);
    }

    pub async fn get_class(&self, id: &str) -> Option<CatalogEntry> {
        self.classes
            .read()
            .await
            .get(id)
            .map(|e| e.catalog.clone())
    }

    pub async fn classes_catalog(&self) -> CatalogResponse {
        CatalogResponse {
            entries: self
                .classes
                .read()
                .await
                .values()
                .map(|e| e.catalog.clone())
                .collect(),
        }
    }

    // -----------------------------------------------------------------------
    // Backgrounds
    // -----------------------------------------------------------------------

    pub async fn register_background(&self, entry: BackgroundEntry) {
        self.backgrounds
            .write()
            .await
            .insert(entry.catalog.id.clone(), entry);
    }

    pub async fn get_background(&self, id: &str) -> Option<CatalogEntry> {
        self.backgrounds
            .read()
            .await
            .get(id)
            .map(|e| e.catalog.clone())
    }

    pub async fn backgrounds_catalog(&self) -> CatalogResponse {
        CatalogResponse {
            entries: self
                .backgrounds
                .read()
                .await
                .values()
                .map(|e| e.catalog.clone())
                .collect(),
        }
    }

    // -----------------------------------------------------------------------
    // Feats (Dotes)
    // -----------------------------------------------------------------------

    pub async fn register_feat(&self, entry: FeatEntry) {
        self.feats
            .write()
            .await
            .insert(entry.catalog.id.clone(), entry);
    }

    pub async fn get_feat(&self, id: &str) -> Option<CatalogEntry> {
        self.feats
            .read()
            .await
            .get(id)
            .map(|e| e.catalog.clone())
    }

    pub async fn feats_catalog(&self) -> CatalogResponse {
        let mut entries: Vec<CatalogEntry> = self
            .feats
            .read()
            .await
            .values()
            .map(|e| e.catalog.clone())
            .collect();
        // Ordenar alfabéticamente por nombre para el wizard
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        CatalogResponse { entries }
    }
}

// ---------------------------------------------------------------------------
// Defaults PHB 2024 — poblamos el registro al arrancar
//
// La fuente de verdad de nombres, descripciones, choices y rasgos vive en
// los métodos `catalog_entry()` de cada implementación (trait Race / Class /
// Background / Feat). Aquí simplemente instanciamos y registramos.
// ---------------------------------------------------------------------------

use shared::models::defaults::backgrounds::{
    Acolyte, Artisan, Charlatan, Criminal, Guide,
    Hermit, Noble, Sailor, Scholar, Soldier,
};
use shared::models::defaults::classes::{
    Barbarian, Bard, Cleric, Druid, Fighter,
    Monk, Paladin, Ranger, Rogue, Sorcerer, Warlock, Wizard,
};
use shared::models::defaults::feats::{
    // Combate
    Alert, Charger, CrossbowExpert, DefensiveDuelist, DualWielder,
    GreatWeaponMaster, Grappler, MageSlayer, MountedCombatant, PolearmMaster,
    SavageAttacker, Sentinel, Sharpshooter, ShieldMaster, SpellSniper,
    TavernBrawler, WarCaster, WeaponMaster,
    // Habilidad y Exploración
    Actor, Athlete, DungeonDelver, Durable, HeavilyArmored, HeavyArmorMaster,
    InspiringLeader, KeenMind, LightlyArmored, Lucky, MartialAdept,
    MediumArmorMaster, Mobile, ModeratelyArmored, Observant, Resilient,
    Skilled, Skulker, Tough,
    // Mágicos
    ElementalAdept, Healer, MagicInitiate, RitualCaster,
};
use shared::models::defaults::races::{
    Dragonborn, Dwarf, Elf, Gnome, HalfElf, HalfOrc, Halfling, Human, Tiefling,
};
use shared::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption};
use shared::traits::{background::Background, class::Class, feat::Feat, race::Race};

/// Macro interna: registra una raza delegando catalog_entry() a la implementación.
macro_rules! race {
    ($registry:expr, $imp:expr) => {{
        let imp: Box<dyn Race + Send + Sync> = Box::new($imp);
        let catalog = imp.catalog_entry();
        $registry.register_race(RaceEntry { catalog, implementation: imp }).await;
    }};
}

/// Macro interna: registra una clase delegando catalog_entry() a la implementación.
macro_rules! class {
    ($registry:expr, $imp:expr) => {{
        let imp: Box<dyn Class + Send + Sync> = Box::new($imp);
        let catalog = imp.catalog_entry();
        $registry.register_class(ClassEntry { catalog, implementation: imp }).await;
    }};
}

/// Macro interna: registra un trasfondo delegando catalog_entry() a la implementación.
macro_rules! bg {
    ($registry:expr, $imp:expr) => {{
        let imp: Box<dyn Background + Send + Sync> = Box::new($imp);
        let catalog = imp.catalog_entry();
        $registry.register_background(BackgroundEntry { catalog, implementation: imp }).await;
    }};
}

pub async fn register_phb_defaults(registry: &Registry) {
    // ── Razas ────────────────────────────────────────────────────────────────
    race!(registry, Human);
    race!(registry, Elf);
    race!(registry, Dwarf);
    race!(registry, Halfling);
    race!(registry, Gnome);
    race!(registry, HalfElf);
    race!(registry, HalfOrc);
    race!(registry, Tiefling);
    race!(registry, Dragonborn);

    // ── Clases ───────────────────────────────────────────────────────────────
    class!(registry, Barbarian);
    class!(registry, Bard);
    class!(registry, Cleric);
    class!(registry, Druid);
    class!(registry, Fighter);
    class!(registry, Monk);
    class!(registry, Paladin);
    class!(registry, Ranger);
    class!(registry, Rogue);
    class!(registry, Sorcerer);
    class!(registry, Warlock);
    class!(registry, Wizard);

    // ── Trasfondos ───────────────────────────────────────────────────────────
    bg!(registry, Acolyte);
    bg!(registry, Artisan);
    bg!(registry, Charlatan);
    bg!(registry, Criminal);
    bg!(registry, Scholar);
    bg!(registry, Guide);
    bg!(registry, Sailor);
    bg!(registry, Noble);
    bg!(registry, Soldier);
    bg!(registry, Hermit);

    // ── Dotes PHB 2024 ───────────────────────────────────────────────────────
    // Los dotes no tienen un trait análogo con catalog_entry(), así que
    // se construyen explícitamente aquí. Están agrupados por categoría.

    // Combate
    let combat_feats: Vec<FeatEntry> = vec![
        FeatEntry {
            catalog: CatalogEntry {
                id: "alert".into(), name: "Alerta".into(), source: "PHB2024".into(),
                description: Some("+5 a iniciativa, no puedes ser sorprendido, las criaturas ocultas no tienen ventaja al atacarte.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+5 Iniciativa".into(), "Inmune a Sorpresa".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Alert),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "charger".into(), name: "Cargador".into(), source: "PHB2024".into(),
                description: Some("Tras cargar 10ft en línea recta, ataque adicional con +1d8 de daño o empuja 10ft.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Ataque de Carga".into(), "Empuje de Carga".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Charger),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "crossbow_expert".into(), name: "Experto en Ballesta".into(), source: "PHB2024".into(),
                description: Some("Ignorar recarga, disparar cuerpo a cuerpo sin desventaja, ataque adicional con ballesta de mano.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Sin Recarga".into(), "Ataque Adicional".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(CrossbowExpert),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "defensive_duelist".into(), name: "Duelista Defensivo".into(), source: "PHB2024".into(),
                description: Some("Req: Des 13. Reacción: +bono prof a CA vs un ataque cuerpo a cuerpo con arma de finura.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Des 13".into(), "Reacción Defensiva".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(DefensiveDuelist),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "dual_wielder".into(), name: "Luchador con Dos Armas".into(), source: "PHB2024".into(),
                description: Some("+1 CA con dos armas, usar armas no ligeras de una mano, desenvainar dos armas a la vez.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 CA".into(), "Armas No Ligeras".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(DualWielder),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "great_weapon_master".into(), name: "Gran Maestro de Armas".into(), source: "PHB2024".into(),
                description: Some("Ataque adicional tras derribo o crítico. Opción: -5 ataque / +10 daño con armas pesadas.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Ataque Extra".into(), "-5/+10 Pesadas".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(GreatWeaponMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "grappler".into(), name: "Luchador".into(), source: "PHB2024".into(),
                description: Some("Req: Fue 13. Ventaja en ataques vs criatura agarrada, puedes inmovilizarla.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Fue 13".into(), "Ventaja vs Agarrado".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Grappler),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "mage_slayer".into(), name: "Cazador de Magos".into(), source: "PHB2024".into(),
                description: Some("Reacción para atacar a lanzadores adyacentes. Desventaja en concentración para criaturas que atacas.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Reacción Antimagia".into(), "Rompe Concentración".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(MageSlayer),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "mounted_combatant".into(), name: "Combatiente Montado".into(), source: "PHB2024".into(),
                description: Some("Ventaja vs criaturas menores que la montura, redirigir ataques a la montura.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Ventaja Montado".into(), "Proteger Montura".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(MountedCombatant),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "polearm_master".into(), name: "Maestro de Armas de Asta".into(), source: "PHB2024".into(),
                description: Some("Ataque adicional con el extremo opuesto (1d4). Reacción al entrar criaturas en tu alcance.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Ataque de Cola".into(), "Guardián de Alcance".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(PolearmMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "savage_attacker".into(), name: "Atacante Salvaje".into(), source: "PHB2024".into(),
                description: Some("Una vez por turno con arma cuerpo a cuerpo: tirar dados de daño dos veces, usar el mayor.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Doble Tirada de Daño".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(SavageAttacker),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "sentinel".into(), name: "Centinela".into(), source: "PHB2024".into(),
                description: Some("Ataques de oportunidad reducen velocidad a 0. Atacar a criaturas que golpean a otros adyacentes.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Velocidad a 0".into(), "Guardián".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Sentinel),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "sharpshooter".into(), name: "Tirador Certero".into(), source: "PHB2024".into(),
                description: Some("Sin desventaja a larga distancia, ignorar cobertura media/3-4, opción -5/+10 a distancia.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Larga Distancia".into(), "-5/+10 Ranged".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Sharpshooter),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "shield_master".into(), name: "Maestro de Escudo".into(), source: "PHB2024".into(),
                description: Some("Empujar con escudo como acción adicional, añadir CA de escudo a salvaciones Des.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Empuje de Escudo".into(), "Bonus Salvación Des".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(ShieldMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "spell_sniper".into(), name: "Francotirador de Conjuros".into(), source: "PHB2024".into(),
                description: Some("Doblar alcance de conjuros con ataque, ignorar cobertura, aprender un truco de ataque.".into()),
                lore: None, image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "spell_sniper.cantrip_class".into(),
                    label: "Clase para el truco de ataque".into(),
                    options: vec![
                        SelectOption::bare("bard",     "Bardo"),
                        SelectOption::bare("cleric",   "Clérigo"),
                        SelectOption::bare("druid",    "Druida"),
                        SelectOption::bare("sorcerer", "Hechicero"),
                        SelectOption::bare("warlock",  "Brujo"),
                        SelectOption::bare("wizard",   "Mago"),
                    ],
                }],
                required_choices: vec!["spell_sniper.cantrip_class".into()],
                traits_preview: vec!["Alcance Doble".into(), "Ignorar Cobertura".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(SpellSniper),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "tavern_brawler".into(), name: "Peleador de Taberna".into(), source: "PHB2024".into(),
                description: Some("Golpes desarmados 1d4, objetos improvisados. Acción adicional para agarrar tras golpe.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Puño 1d4".into(), "Objetos Improvisados".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(TavernBrawler),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "war_caster".into(), name: "Lanzador de Guerra".into(), source: "PHB2024".into(),
                description: Some("Req: lanzar conjuros. Ventaja en concentración, componentes somáticos ocupado, conjuro como AO.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: conjuros".into(), "Concentración Estable".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(WarCaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "weapon_master".into(), name: "Maestro de Armas".into(), source: "PHB2024".into(),
                description: Some("+1 Fue o Des (máx 20), proficiencia en 4 armas a elegir.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Fue/Des".into(), "4 Proficiencias".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(WeaponMaster),
        },
    ];
    for feat in combat_feats { registry.register_feat(feat).await; }

    // Habilidad y Exploración
    let utility_feats: Vec<FeatEntry> = vec![
        FeatEntry {
            catalog: CatalogEntry {
                id: "actor".into(), name: "Actor".into(), source: "PHB2024".into(),
                description: Some("+1 Car (máx 20), ventaja al suplantar identidades, imitar voces de criaturas.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Carisma".into(), "Suplantación".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Actor),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "athlete".into(), name: "Atleta".into(), source: "PHB2024".into(),
                description: Some("+1 Fue o Des (máx 20), levantarse cuesta solo 5ft, trepar sin penalización.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Fue/Des".into(), "Trepar".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Athlete),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "dungeon_delver".into(), name: "Explorador de Mazmorras".into(), source: "PHB2024".into(),
                description: Some("Ventaja en detectar puertas secretas y trampas. Resistencia a daño de trampas.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Detectar Trampas".into(), "Resistencia Trampas".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(DungeonDelver),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "durable".into(), name: "Resistente".into(), source: "PHB2024".into(),
                description: Some("+1 Con (máx 20), en descanso corto recuperar mínimo 2×mod.Con con dados de golpe.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Constitución".into(), "Dados de Golpe Mejorados".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Durable),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "heavily_armored".into(), name: "Armadura Pesada".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura media. +1 Fue (máx 20), proficiencia con armaduras pesadas.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Armadura Media".into(), "Prof. Armadura Pesada".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(HeavilyArmored),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "heavy_armor_master".into(), name: "Maestro de Armadura Pesada".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura pesada. +1 Fue, reducir daño no mágico en 3 con armadura pesada.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Armadura Pesada".into(), "Reducción de Daño 3".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(HeavyArmorMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "inspiring_leader".into(), name: "Líder Inspirador".into(), source: "PHB2024".into(),
                description: Some("Req: Car 13. Discurso 10min: hasta 6 criaturas ganan PG temp = nivel + mod Car.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Car 13".into(), "PG Temporales al Grupo".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(InspiringLeader),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "keen_mind".into(), name: "Mente Aguda".into(), source: "PHB2024".into(),
                description: Some("+1 Int (máx 20), conocer siempre norte/hora/días, recordar todo lo visto en un mes.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Inteligencia".into(), "Memoria Perfecta".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(KeenMind),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "lightly_armored".into(), name: "Armadura Ligera".into(), source: "PHB2024".into(),
                description: Some("+1 Fue o Des (máx 20), proficiencia con armaduras ligeras.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Fue/Des".into(), "Prof. Armadura Ligera".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(LightlyArmored),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "lucky".into(), name: "Afortunado".into(), source: "PHB2024".into(),
                description: Some("3 puntos de suerte/día: tirar d20 extra en ataques, pruebas o salvaciones.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["3 Puntos de Suerte".into(), "Relanzar Tiradas".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Lucky),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "martial_adept".into(), name: "Adepto Marcial".into(), source: "PHB2024".into(),
                description: Some("2 maniobras de Maestro de Batalla y 1 dado de superioridad d6.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["2 Maniobras".into(), "Dado Superioridad d6".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(MartialAdept),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "medium_armor_master".into(), name: "Maestro de Armadura Media".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura media. Sin desventaja en Sigilo, Des máx +3 a CA.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Armadura Media".into(), "Des +3 CA".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(MediumArmorMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "mobile".into(), name: "Ágil".into(), source: "PHB2024".into(),
                description: Some("+10ft velocidad, sin terreno difícil al cargar, no provocar AO vs criaturas atacadas.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+10ft Velocidad".into(), "Sin AO".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Mobile),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "moderately_armored".into(), name: "Armadura Intermedia".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura ligera. +1 Fue o Des (máx 20), prof con armadura media y escudo.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Armadura Ligera".into(), "Prof. Armadura Media + Escudo".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(ModeratelyArmored),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "observant".into(), name: "Observador".into(), source: "PHB2024".into(),
                description: Some("+1 Int o Sab (máx 20), leer labios, +5 pasivo Percepción e Investigación.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+1 Int/Sab".into(), "+5 Percepción/Investigación".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Observant),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "resilient".into(), name: "Resistencia".into(), source: "PHB2024".into(),
                description: Some("Elegir atributo: +1 (máx 20) y proficiencia en salvaciones de ese atributo.".into()),
                lore: None, image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
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
                }],
                required_choices: vec!["resilient.ability".into()],
                traits_preview: vec!["+1 Atributo".into(), "Nueva Salvación".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Resilient),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "skilled".into(), name: "Hábil".into(), source: "PHB2024".into(),
                description: Some("Proficiencia en 3 habilidades o herramientas a elegir.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["3 Proficiencias".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Skilled),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "skulker".into(), name: "Merodeador".into(), source: "PHB2024".into(),
                description: Some("Req: Des 13. Esconderse en luz tenue, fallar ataque no revela posición.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Des 13".into(), "Sigilo Mejorado".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Skulker),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "tough".into(), name: "Fornido".into(), source: "PHB2024".into(),
                description: Some("+2 PG máximos por nivel (actual y futuro).".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["+2 PG por Nivel".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Tough),
        },
    ];
    for feat in utility_feats { registry.register_feat(feat).await; }

    // Mágicos
    let magic_feats: Vec<FeatEntry> = vec![
        FeatEntry {
            catalog: CatalogEntry {
                id: "elemental_adept".into(), name: "Adepto Elemental".into(), source: "PHB2024".into(),
                description: Some("Req: conjuros. Ignorar resistencia a un tipo de daño, tratar 1s en dados como 2s.".into()),
                lore: None, image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "elemental_adept.damage_type".into(),
                    label: "Tipo de daño elemental".into(),
                    options: vec![
                        SelectOption::bare("acid",      "Ácido"),
                        SelectOption::bare("cold",      "Frío"),
                        SelectOption::bare("fire",      "Fuego"),
                        SelectOption::bare("lightning", "Relámpago"),
                        SelectOption::bare("thunder",   "Trueno"),
                    ],
                }],
                required_choices: vec!["elemental_adept.damage_type".into()],
                traits_preview: vec!["Req: conjuros".into(), "Ignorar Resistencia".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(ElementalAdept),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "healer".into(), name: "Sanador".into(), source: "PHB2024".into(),
                description: Some("Usar botiquín gratis para estabilizar. Curar 1d6+4+DG PG una vez por criatura por descanso.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Estabilizar Gratis".into(), "Botiquín Mejorado".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(Healer),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "magic_initiate".into(), name: "Iniciado en la Magia".into(), source: "PHB2024".into(),
                description: Some("Elegir clase: aprender 2 trucos y 1 conjuro de nivel 1. Lanzar el conjuro 1/día gratis.".into()),
                lore: None, image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
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
                }],
                required_choices: vec!["magic_initiate.class".into()],
                traits_preview: vec!["2 Trucos".into(), "Conjuro Nivel 1 Gratis".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(MagicInitiate),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "ritual_caster".into(), name: "Lanzador de Rituales".into(), source: "PHB2024".into(),
                description: Some("Req: Int o Sab 13. Libro con 2 rituales de nivel 1. Lanzarlos como ritual, copiar más.".into()),
                lore: None, image_url: None, choices: vec![],
                required_choices: vec![],
                traits_preview: vec!["Req: Int/Sab 13".into(), "Libro de Rituales".into()],
                traits_detail: vec![], speed_m: None, size: None,
            },
            implementation: Box::new(RitualCaster),
        },
    ];
    for feat in magic_feats { registry.register_feat(feat).await; }
}
