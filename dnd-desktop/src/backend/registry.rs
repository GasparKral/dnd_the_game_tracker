use shared::{
    api_types::catalog::{CatalogEntry, CatalogResponse, ChoiceSchema, SelectOption},
    traits::{background::Background, class::Class, feat::Feat, race::Race},
};
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

pub async fn register_phb_defaults(registry: &Registry) {
    // --- Razas ---
    registry
        .register_race(RaceEntry {
            catalog: CatalogEntry {
                id: "human".into(),
                name: "Humano".into(),
                source: "PHB2024".into(),
                description: Some(
                    "Versátiles y ambiciosos, los humanos son la raza más extendida de los Reinos Olvidados.".into(),
                ),
                image_url: None,
                choices: vec![],
                traits_preview: vec![
                    "Versátil".into(),
                    "Talento héroe".into(),
                ],
            },
            implementation: Box::new(Human),
        })
        .await;

    registry
        .register_race(RaceEntry {
            catalog: CatalogEntry {
                id: "elf".into(),
                name: "Elfo".into(),
                source: "PHB2024".into(),
                description: Some(
                    "Seres mágicos de gran belleza, los elfos viven miles de años y sienten profundamente la naturaleza.".into(),
                ),
                image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "elf.lineage".into(),
                    label: "Linaje élfico".into(),
                    options: vec![
                        SelectOption {
                            id: "high_elf".into(),
                            label: "Alto Elfo".into(),
                            description: Some("Herencia arcana innata.".into()),
                        },
                        SelectOption {
                            id: "wood_elf".into(),
                            label: "Elfo del Bosque".into(),
                            description: Some("Velocidad y sigilo aumentados.".into()),
                        },
                        SelectOption {
                            id: "drow".into(),
                            label: "Drow".into(),
                            description: Some("Magia de las profundidades.".into()),
                        },
                    ],
                }],
                traits_preview: vec![
                    "Visión en la Oscuridad".into(),
                    "Sentidos Feéricos".into(),
                    "Trance".into(),
                    "Linaje Élfico".into(),
                ],
            },
            implementation: Box::new(Elf),
        })
        .await;

    registry
        .register_race(RaceEntry {
            catalog: CatalogEntry {
                id: "dwarf".into(),
                name: "Enano".into(),
                source: "PHB2024".into(),
                description: Some(
                    "Resistentes y tenaces, los enanos son maestros artesanos que habitan en fortalezas montañosas.".into(),
                ),
                image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "dwarf.lineage".into(),
                    label: "Linaje enano".into(),
                    options: vec![
                        SelectOption {
                            id: "hill_dwarf".into(),
                            label: "Enano de las Colinas".into(),
                            description: Some("Sabiduría y puntos de golpe adicionales.".into()),
                        },
                        SelectOption {
                            id: "mountain_dwarf".into(),
                            label: "Enano de la Montaña".into(),
                            description: Some("Pericia con armaduras medias.".into()),
                        },
                    ],
                }],
                traits_preview: vec![
                    "Visión en la Oscuridad".into(),
                    "Resistencia Enana".into(),
                    "Entrenamiento de Combate Enano".into(),
                    "Velocidad de Herramienta".into(),
                    "Solidez Pétrea".into(),
                ],
            },
            implementation: Box::new(Dwarf),
        })
        .await;

    // --- Clases ---

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "barbarian".into(), name: "Bárbaro".into(), source: "PHB2024".into(),
            description: Some("Guerrero feroz que canaliza una rabia primaria para destrozar a sus enemigos.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "barbarian.primal_path".into(),
                label: "Senda primaria".into(),
                options: vec![
                    SelectOption { id: "berserker".into(), label: "Berserker".into(), description: Some("Rabia y violencia sin límites.".into()) },
                    SelectOption { id: "totem_warrior".into(), label: "Guerrero Totémico".into(), description: Some("Espíritu animal guía.".into()) },
                    SelectOption { id: "world_tree".into(), label: "Árbol del Mundo".into(), description: Some("Conexión con la red de vida.".into()) },
                ],
            }],
            traits_preview: vec!["Rabia".into(), "Defensa Sin Armadura".into(), "Ataque Descuidado".into(), "Movimiento Rápido".into()],
        },
        implementation: Box::new(Barbarian),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "bard".into(), name: "Bardo".into(), source: "PHB2024".into(),
            description: Some("Maestro de las artes y la magia, inspira a sus aliados con música y palabras.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "bard.college".into(),
                label: "Colegio bardístico".into(),
                options: vec![
                    SelectOption { id: "lore".into(), label: "Colegio del Saber".into(), description: Some("Conocimiento y habilidades extra.".into()) },
                    SelectOption { id: "valor".into(), label: "Colegio del Valor".into(), description: Some("Combate y armas marciales.".into()) },
                    SelectOption { id: "glamour".into(), label: "Colegio del Glamour".into(), description: Some("Magia feérica y carisma.".into()) },
                ],
            }],
            traits_preview: vec!["Inspiración Bárdica".into(), "Conjuros".into(), "Competencias Variadas".into()],
        },
        implementation: Box::new(Bard),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "cleric".into(), name: "Clérigo".into(), source: "PHB2024".into(),
            description: Some("Intermediario divino que canaliza el poder de su deidad para sanar y destruir.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "cleric.domain".into(),
                label: "Dominio divino".into(),
                options: vec![
                    SelectOption { id: "life".into(), label: "Vida".into(), description: Some("Sanación y vitalidad.".into()) },
                    SelectOption { id: "light".into(), label: "Luz".into(), description: Some("Fuego y revelación.".into()) },
                    SelectOption { id: "war".into(), label: "Guerra".into(), description: Some("Combate y poder marcial.".into()) },
                    SelectOption { id: "trickery".into(), label: "Engaño".into(), description: Some("Ilusión y duplicidad.".into()) },
                    SelectOption { id: "nature".into(), label: "Naturaleza".into(), description: Some("Animales y mundo natural.".into()) },
                ],
            }],
            traits_preview: vec!["Conjuros".into(), "Canal de Divinidad".into(), "Intervención Divina".into()],
        },
        implementation: Box::new(Cleric),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "druid".into(), name: "Druida".into(), source: "PHB2024".into(),
            description: Some("Guardián de la naturaleza que transforma su cuerpo y controla los elementos.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "druid.circle".into(),
                label: "Círculo druídico".into(),
                options: vec![
                    SelectOption { id: "land".into(), label: "Círculo de la Tierra".into(), description: Some("Magia del terreno natural.".into()) },
                    SelectOption { id: "moon".into(), label: "Círculo de la Luna".into(), description: Some("Formas bestiales poderosas.".into()) },
                    SelectOption { id: "stars".into(), label: "Círculo de las Estrellas".into(), description: Some("Constelaciones y cosmos.".into()) },
                ],
            }],
            traits_preview: vec!["Forma Salvaje".into(), "Conjuros".into(), "Ritual Druídico".into()],
        },
        implementation: Box::new(Druid),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "fighter".into(), name: "Guerrero".into(), source: "PHB2024".into(),
            description: Some("Maestro del combate que domina todo tipo de armas y armaduras.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "fighter.archetype".into(),
                label: "Arquetipo marcial".into(),
                options: vec![
                    SelectOption { id: "champion".into(), label: "Campeón".into(), description: Some("Críticos más frecuentes y atletismo.".into()) },
                    SelectOption { id: "battle_master".into(), label: "Maestro de Batalla".into(), description: Some("Maniobras de combate táctico.".into()) },
                    SelectOption { id: "eldritch_knight".into(), label: "Caballero Sobrenatural".into(), description: Some("Conjuros de mago combinados.".into()) },
                    SelectOption { id: "psi_warrior".into(), label: "Guerrero Psíquico".into(), description: Some("Poderes psiónico y telekinesis.".into()) },
                ],
            }],
            traits_preview: vec!["Estilo de Combate".into(), "Segundo Aliento".into(), "Oleada de Acción".into(), "Ataque Extra".into()],
        },
        implementation: Box::new(Fighter),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "monk".into(), name: "Monje".into(), source: "PHB2024".into(),
            description: Some("Artista marcial que canaliza ki para realizar proezas sobrehumanas.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "monk.tradition".into(),
                label: "Tradición monástica".into(),
                options: vec![
                    SelectOption { id: "open_hand".into(), label: "Mano Abierta".into(), description: Some("Maestría en combate desarmado.".into()) },
                    SelectOption { id: "shadow".into(), label: "Sombra".into(), description: Some("Sigilo y magia oscura.".into()) },
                    SelectOption { id: "four_elements".into(), label: "Cuatro Elementos".into(), description: Some("Control elemental con ki.".into()) },
                ],
            }],
            traits_preview: vec!["Defensa Sin Armadura".into(), "Artes Marciales".into(), "Ki".into(), "Movimiento Sin Obstáculos".into()],
        },
        implementation: Box::new(Monk),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "paladin".into(), name: "Paladín".into(), source: "PHB2024".into(),
            description: Some("Guerrero sagrado que une fuerza marcial con poder divino bajo un juramento inviolable.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "paladin.oath".into(),
                label: "Sagrado juramento".into(),
                options: vec![
                    SelectOption { id: "devotion".into(), label: "Devoción".into(), description: Some("Honor y justicia absoluta.".into()) },
                    SelectOption { id: "ancients".into(), label: "Los Antiguos".into(), description: Some("Preservar la luz y la vida.".into()) },
                    SelectOption { id: "vengeance".into(), label: "Venganza".into(), description: Some("Castigar a los malvados sin piedad.".into()) },
                    SelectOption { id: "glory".into(), label: "Gloria".into(), description: Some("Hazañas épicas y fama.".into()) },
                ],
            }],
            traits_preview: vec!["Imposición de Manos".into(), "Estilo de Combate".into(), "Sentido Divino".into(), "Smite Divino".into()],
        },
        implementation: Box::new(Paladin),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "ranger".into(), name: "Explorador".into(), source: "PHB2024".into(),
            description: Some("Rastreador salvaje que combina habilidades marciales con magia de la naturaleza.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "ranger.conclave".into(),
                label: "Conclave de exploradores".into(),
                options: vec![
                    SelectOption { id: "hunter".into(), label:"Cazador".into(), description: Some("Especialista en eliminar amenazas concretas.".into()) },
                    SelectOption { id: "beast_master".into(), label: "Maestro de Bestias".into(), description: Some("Compañero animal de combate.".into()) },
                    SelectOption { id: "gloom_stalker".into(), label: "Acechador de las Sombras".into(), description: Some("Cazador en la oscuridad.".into()) },
                ],
            }],
            traits_preview: vec!["Enemigo Favorito".into(), "Explorador Natural".into(), "Conjuros".into(), "Ataque Extra".into()],
        },
        implementation: Box::new(Ranger),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "rogue".into(), name: "Pícaro".into(), source: "PHB2024".into(),
            description: Some("Maestro del sigilo y la astucia que golpea cuando el enemigo menos lo espera.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "rogue.archetype".into(),
                label: "Arquetipo pícaro".into(),
                options: vec![
                    SelectOption { id: "thief".into(), label: "Ladrón".into(), description: Some("Robo y uso de objetos mágicos.".into()) },
                    SelectOption { id: "assassin".into(), label: "Asesino".into(), description: Some("Eliminar objetivos por sorpresa.".into()) },
                    SelectOption { id: "arcane_trickster".into(), label: "Tramposo Arcano".into(), description: Some("Conjuros de ilusión y encantamiento.".into()) },
                    SelectOption { id: "soulknife".into(), label: "Cuchilla Anímica".into(), description: Some("Hojas psíquicas y telepatía.".into()) },
                ],
            }],
            traits_preview: vec!["Ataque Furtivo".into(), "Jerga de Ladrones".into(), "Acción Astuta".into(), "Reflejos Esquivos".into()],
        },
        implementation: Box::new(Rogue),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "sorcerer".into(), name: "Hechicero".into(), source: "PHB2024".into(),
            description: Some("Lanzador nato cuyo poder mágico emana de su linaje o un evento cósmico.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "sorcerer.origin".into(),
                label: "Origen hechiceril".into(),
                options: vec![
                    SelectOption { id: "draconic".into(), label: "Linaje Dracónico".into(), description: Some("Sangre de dragón amplifica tu magia.".into()) },
                    SelectOption { id: "wild_magic".into(), label: "Magia Salvaje".into(), description: Some("Oleadas de magia impredecible.".into()) },
                    SelectOption { id: "divine_soul".into(), label: "Alma Divina".into(), description: Some("Herencia celestial o infernal.".into()) },
                    SelectOption { id: "clockwork".into(), label: "Alma de Relojería".into(), description: Some("Orden cósmico y precision.".into()) },
                ],
            }],
            traits_preview: vec!["Conjuros".into(), "Puntos de Hechicería".into(), "Metamagia".into()],
        },
        implementation: Box::new(Sorcerer),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "warlock".into(), name: "Brujo".into(), source: "PHB2024".into(),
            description: Some("Lanzador de pacto cuyo poder viene de un acuerdo con una entidad sobrenatural.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "warlock.patron".into(),
                label: "Patrón sobrenatural".into(),
                options: vec![
                    SelectOption { id: "fiend".into(), label: "El Señor Infernal".into(), description: Some("Poder de los planos del mal.".into()) },
                    SelectOption { id: "great_old_one".into(), label: "El Gran Antiguo".into(), description: Some("Entidad cósmica inescrutable.".into()) },
                    SelectOption { id: "archfey".into(), label: "El Archihada".into(), description: Some("Señor feérico caprichoso.".into()) },
                    SelectOption { id: "celestial".into(), label: "El Celestial".into(), description: Some("Ser de luz y bien absoluto.".into()) },
                ],
            }],
            traits_preview: vec!["Conjuros de Pacto".into(), "Invocaciones Sobrenaturales".into(), "Dádiva del Patrón".into()],
        },
        implementation: Box::new(Warlock),
    }).await;

    registry.register_class(ClassEntry {
        catalog: CatalogEntry {
            id: "wizard".into(), name: "Mago".into(), source: "PHB2024".into(),
            description: Some("Erudito arcano que estudia los secretos del universo para dominar la magia.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "wizard.school".into(),
                label: "Tradición arcana".into(),
                options: vec![
                    SelectOption { id: "evocation".into(), label: "Evocación".into(), description: Some("Conjuros de daño y energía.".into()) },
                    SelectOption { id: "abjuration".into(), label: "Abjuración".into(), description: Some("Protección y contramagia.".into()) },
                    SelectOption { id: "illusion".into(), label: "Ilusión".into(), description: Some("Engañar sentidos y percepción.".into()) },
                    SelectOption { id: "necromancy".into(), label: "Nigromancia".into(), description: Some("Muerte y no-muertos.".into()) },
                    SelectOption { id: "conjuration".into(), label: "Conjuración".into(), description: Some("Convocar criaturas y teletransportación.".into()) },
                    SelectOption { id: "divination".into(), label: "Adivinación".into(), description: Some("Ver el pasado, presente y futuro.".into()) },
                ],
            }],
            traits_preview: vec!["Recuperación Arcana".into(), "Libro de Conjuros".into(), "Preparar Conjuros".into()],
        },
        implementation: Box::new(Wizard),
    }).await;

    // --- Trasfondos ---

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "acolyte".into(), name: "Acólito".into(), source: "PHB2024".into(),
            description: Some("Has dedicado tu vida al servicio de un templo.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Refugio de los Fieles".into(), "Conocimiento Religioso".into()],
        },
        implementation: Box::new(Acolyte),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "artisan".into(), name: "Artesano".into(), source: "PHB2024".into(),
            description: Some("Miembro de un gremio artesanal con contactos en el mundo del comercio.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Membresía en Gremio".into(), "Conocimiento de Herramientas".into()],
        },
        implementation: Box::new(Artisan),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "charlatan".into(), name: "Charlatán".into(), source: "PHB2024".into(),
            description: Some("Maestro del engaño y la suplantación de identidades.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Identidad Falsa".into(), "Engaño y Persuasión".into()],
        },
        implementation: Box::new(Charlatan),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "criminal".into(), name: "Criminal".into(), source: "PHB2024".into(),
            description: Some("Experimentado en actividades ilegales con contactos en el bajo mundo.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Contacto Criminal".into(), "Sigilo y Juego de Manos".into()],
        },
        implementation: Box::new(Criminal),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "scholar".into(), name: "Erudito".into(), source: "PHB2024".into(),
            description: Some("Estudioso que ha pasado años investigando en archivos y academias.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Investigador".into(), "Historia y Arcanos".into()],
        },
        implementation: Box::new(Scholar),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "guide".into(), name: "Guardabosques".into(), source: "PHB2024".into(),
            description: Some("Explorador de tierras salvajes con profundo conocimiento de la naturaleza.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Viajero del Yermo".into(), "Supervivencia y Naturaleza".into()],
        },
        implementation: Box::new(Guide),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "sailor".into(), name: "Marinero".into(), source: "PHB2024".into(),
            description: Some("Veterano del mar con experiencia navegando por mares y ríos.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Pasaje en Barco".into(), "Atletismo y Percepción".into()],
        },
        implementation: Box::new(Sailor),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "noble".into(), name: "Noble".into(), source: "PHB2024".into(),
            description: Some("Criado en privilegio con influencia política y social.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Posición de Privilegio".into(), "Historia y Persuasión".into()],
        },
        implementation: Box::new(Noble),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "soldier".into(), name: "Soldado".into(), source: "PHB2024".into(),
            description: Some("Veterano de campaña con entrenamiento militar y disciplina de hierro.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Rango Militar".into(), "Atletismo e Intimidación".into()],
        },
        implementation: Box::new(Soldier),
    }).await;

    registry.register_background(BackgroundEntry {
        catalog: CatalogEntry {
            id: "hermit".into(), name: "Ermitaño".into(), source: "PHB2024".into(),
            description: Some("Viviste en reclusión, descubriendo verdades que pocos conocen.".into()),
            image_url: None, choices: vec![],
            traits_preview: vec!["Descubrimiento".into(), "Medicina y Religión".into()],
        },
        implementation: Box::new(Hermit),
    }).await;

    // --- Razas adicionales PHB 2024 ---

    registry.register_race(RaceEntry {
        catalog: CatalogEntry {
            id: "halfling".into(),
            name: "Mediano".into(),
            source: "PHB2024".into(),
            description: Some("Pequeños y afortunados, los medianos son notables por su suerte innata y su valentía sorprendente.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "halfling.lineage".into(),
                label: "Linaje de mediano".into(),
                options: vec![
                    SelectOption { id: "lightfoot".into(), label: "Pies Ligeros".into(), description: Some("Esconderse tras criaturas mayores.".into()) },
                    SelectOption { id: "stout".into(), label: "Robusto".into(), description: Some("Resistencia a veneno.".into()) },
                ],
            }],
            traits_preview: vec![
                "Suerte".into(),
                "Valentía".into(),
                "Agilidad Halfling".into(),
                "Linaje de Mediano".into(),
            ],
        },
        implementation: Box::new(Halfling),
    }).await;

    registry.register_race(RaceEntry {
        catalog: CatalogEntry {
            id: "gnome".into(),
            name: "Gnomo".into(),
            source: "PHB2024".into(),
            description: Some("Inventivos e inteligentes, los gnomos poseen una curiosidad insaciable y resistencia innata a la magia.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "gnome.lineage".into(),
                label: "Linaje gnómico".into(),
                options: vec![
                    SelectOption { id: "rock".into(), label: "Gnomo de las Rocas".into(), description: Some("Inventores con conocimiento de artilugios.".into()) },
                    SelectOption { id: "forest".into(), label: "Gnomo Silvático".into(), description: Some("Ilusiones menores y hablar con animales pequeños.".into()) },
                    SelectOption { id: "deep".into(), label: "Gnomo de las Profundidades".into(), description: Some("Camuflaje superior en entornos subterráneos.".into()) },
                ],
            }],
            traits_preview: vec![
                "Astucia Gnómica".into(),
                "Visión en la Oscuridad".into(),
                "Linaje Gnómico".into(),
            ],
        },
        implementation: Box::new(Gnome),
    }).await;

    registry.register_race(RaceEntry {
        catalog: CatalogEntry {
            id: "tiefling".into(),
            name: "Tiefling".into(),
            source: "PHB2024".into(),
            description: Some("Marcados por una herencia infernal, los tieflings poseen poderes oscuros y una apariencia que infunde recelo.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "tiefling.lineage".into(),
                label: "Linaje infernal".into(),
                options: vec![
                    SelectOption { id: "asmodeus".into(), label: "Asmodeo".into(), description: Some("Llama Infernal y Oscuridad.".into()) },
                    SelectOption { id: "zariel".into(), label: "Zariel".into(), description: Some("Fuego y fortaleza marcial.".into()) },
                    SelectOption { id: "levistus".into(), label: "Levistus".into(), description: Some("Frío y escudo de hielo.".into()) },
                    SelectOption { id: "glasya".into(), label: "Glasya".into(), description: Some("Engaño y magia de ilusión.".into()) },
                ],
            }],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Resistencia al Fuego".into(),
                "Legado Infernal".into(),
            ],
        },
        implementation: Box::new(Tiefling),
    }).await;

    registry.register_race(RaceEntry {
        catalog: CatalogEntry {
            id: "dragonborn".into(),
            name: "Draconido".into(),
            source: "PHB2024".into(),
            description: Some("Orgullosos guerreros con sangre dracónica, dotados de un arma de aliento devastadora.".into()),
            image_url: None,
            choices: vec![ChoiceSchema::SingleSelect {
                id: "dragonborn.lineage".into(),
                label: "Linaje dracónico".into(),
                options: vec![
                    SelectOption { id: "black".into(), label: "Dragón Negro (Ácido)".into(), description: Some("Resistencia y aliento de ácido.".into()) },
                    SelectOption { id: "blue".into(), label: "Dragón Azul (Relámpago)".into(), description: Some("Resistencia y aliento de relámpago.".into()) },
                    SelectOption { id: "red".into(), label: "Dragón Rojo (Fuego)".into(), description: Some("Resistencia y aliento de fuego.".into()) },
                    SelectOption { id: "white".into(), label: "Dragón Blanco (Frío)".into(), description: Some("Resistencia y aliento de frío.".into()) },
                    SelectOption { id: "green".into(), label: "Dragón Verde (Veneno)".into(), description: Some("Resistencia y aliento de veneno.".into()) },
                    SelectOption { id: "gold".into(), label: "Dragón Dorado (Fuego)".into(), description: Some("Aliento de fuego o gas debilitante.".into()) },
                    SelectOption { id: "silver".into(), label: "Dragón Plateado (Frío)".into(), description: Some("Aliento de frío o gas paralizante.".into()) },
                ],
            }],
            traits_preview: vec![
                "Arma de Aliento".into(),
                "Resistencia Dracónica".into(),
                "Instinto Dracónico".into(),
            ],
        },
        implementation: Box::new(Dragonborn),
    }).await;

    registry.register_race(RaceEntry {
        catalog: CatalogEntry {
            id: "half_elf".into(),
            name: "Semielfo".into(),
            source: "PHB2024".into(),
            description: Some("Con lo mejor de dos mundos, los semielfos combinan la adaptabilidad humana con la gracia feérica.".into()),
            image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "half_elf.elven_heritage".into(),
                    label: "Herencia élfica".into(),
                    options: vec![
                        SelectOption { id: "trance".into(), label: "Trance".into(), description: Some("Solo necesitas 4 horas de meditación.".into()) },
                        SelectOption { id: "elven_lineage".into(), label: "Linaje Élfico".into(), description: Some("Conjuros innatos de elfo.".into()) },
                        SelectOption { id: "mask_of_the_wild".into(), label: "Máscara de lo Salvaje".into(), description: Some("Esconderse en terreno natural.".into()) },
                    ],
                },
            ],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Sentidos Feéricos".into(),
                "Herencia Humana".into(),
                "Herencia Élfica".into(),
            ],
        },
        implementation: Box::new(HalfElf),
    }).await;

    registry.register_race(RaceEntry {
        catalog: CatalogEntry {
            id: "half_orc".into(),
            name: "Semiorco".into(),
            source: "PHB2024".into(),
            description: Some("Feroces y resistentes, los semiorcos poseen una tenacidad sobrenatural que les permite sobrevivir lo insoportable.".into()),
            image_url: None,
            choices: vec![],
            traits_preview: vec![
                "Visión en la Oscuridad".into(),
                "Resistencia".into(),
                "Feroz".into(),
                "Ataques Implacables".into(),
            ],
        },
        implementation: Box::new(HalfOrc),
    }).await;

    // --- Dotes PHB 2024 ---

    // Combate
    let combat_feats: Vec<FeatEntry> = vec![
        FeatEntry {
            catalog: CatalogEntry {
                id: "alert".into(), name: "Alerta".into(), source: "PHB2024".into(),
                description: Some("+5 a iniciativa, no puedes ser sorprendido, las criaturas ocultas no tienen ventaja al atacarte.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+5 Iniciativa".into(), "Inmune a Sorpresa".into()],
            },
            implementation: Box::new(Alert),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "charger".into(), name: "Cargador".into(), source: "PHB2024".into(),
                description: Some("Tras cargar 10ft en línea recta, ataque adicional con +1d8 de daño o empuja 10ft.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Ataque de Carga".into(), "Empuje de Carga".into()],
            },
            implementation: Box::new(Charger),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "crossbow_expert".into(), name: "Experto en Ballesta".into(), source: "PHB2024".into(),
                description: Some("Ignorar recarga, disparar cuerpo a cuerpo sin desventaja, ataque adicional con ballesta de mano.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Sin Recarga".into(), "Ataque Adicional".into()],
            },
            implementation: Box::new(CrossbowExpert),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "defensive_duelist".into(), name: "Duelista Defensivo".into(), source: "PHB2024".into(),
                description: Some("Req: Des 13. Reacción: +bono prof a CA vs un ataque cuerpo a cuerpo con arma de finura.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Des 13".into(), "Reacción Defensiva".into()],
            },
            implementation: Box::new(DefensiveDuelist),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "dual_wielder".into(), name: "Luchador con Dos Armas".into(), source: "PHB2024".into(),
                description: Some("+1 CA con dos armas, usar armas no ligeras de una mano, desenvainar dos armas a la vez.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 CA".into(), "Armas No Ligeras".into()],
            },
            implementation: Box::new(DualWielder),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "great_weapon_master".into(), name: "Gran Maestro de Armas".into(), source: "PHB2024".into(),
                description: Some("Ataque adicional tras derribo o crítico. Opción: -5 ataque / +10 daño con armas pesadas.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Ataque Extra".into(), "-5/+10 Pesadas".into()],
            },
            implementation: Box::new(GreatWeaponMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "grappler".into(), name: "Luchador".into(), source: "PHB2024".into(),
                description: Some("Req: Fue 13. Ventaja en ataques vs criatura agarrada, puedes inmovilizarla.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Fue 13".into(), "Ventaja vs Agarrado".into()],
            },
            implementation: Box::new(Grappler),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "mage_slayer".into(), name: "Cazador de Magos".into(), source: "PHB2024".into(),
                description: Some("Reacción para atacar a lanzadores adyacentes. Desventaja en concentración para criaturas que atacas.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Reacción Antimagia".into(), "Rompe Concentración".into()],
            },
            implementation: Box::new(MageSlayer),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "mounted_combatant".into(), name: "Combatiente Montado".into(), source: "PHB2024".into(),
                description: Some("Ventaja vs criaturas menores que la montura, redirigir ataques a la montura.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Ventaja Montado".into(), "Proteger Montura".into()],
            },
            implementation: Box::new(MountedCombatant),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "polearm_master".into(), name: "Maestro de Armas de Asta".into(), source: "PHB2024".into(),
                description: Some("Ataque adicional con el extremo opuesto (1d4). Reacción al entrar criaturas en tu alcance.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Ataque de Cola".into(), "Guardián de Alcance".into()],
            },
            implementation: Box::new(PolearmMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "savage_attacker".into(), name: "Atacante Salvaje".into(), source: "PHB2024".into(),
                description: Some("Una vez por turno con arma cuerpo a cuerpo: tirar dados de daño dos veces, usar el mayor.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Doble Tirada de Daño".into()],
            },
            implementation: Box::new(SavageAttacker),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "sentinel".into(), name: "Centinela".into(), source: "PHB2024".into(),
                description: Some("Ataques de oportunidad reducen velocidad a 0. Atacar a criaturas que golpean a otros adyacentes.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Velocidad a 0".into(), "Guardián".into()],
            },
            implementation: Box::new(Sentinel),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "sharpshooter".into(), name: "Tirador Certero".into(), source: "PHB2024".into(),
                description: Some("Sin desventaja a larga distancia, ignorar cobertura media/3-4, opción -5/+10 a distancia.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Larga Distancia".into(), "-5/+10 Ranged".into()],
            },
            implementation: Box::new(Sharpshooter),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "shield_master".into(), name: "Maestro de Escudo".into(), source: "PHB2024".into(),
                description: Some("Empujar con escudo como acción adicional, añadir CA de escudo a salvaciones Des.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Empuje de Escudo".into(), "Bonus Salvación Des".into()],
            },
            implementation: Box::new(ShieldMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "spell_sniper".into(), name: "Francotirador de Conjuros".into(), source: "PHB2024".into(),
                description: Some("Doblar alcance de conjuros con ataque, ignorar cobertura, aprender un truco de ataque.".into()),
                image_url: None, choices: vec![ChoiceSchema::SingleSelect {
                    id: "spell_sniper.cantrip_class".into(),
                    label: "Clase para el truco de ataque".into(),
                    options: vec![
                        SelectOption { id: "bard".into(), label: "Bardo".into(), description: None },
                        SelectOption { id: "cleric".into(), label: "Clérigo".into(), description: None },
                        SelectOption { id: "druid".into(), label: "Druida".into(), description: None },
                        SelectOption { id: "sorcerer".into(), label: "Hechicero".into(), description: None },
                        SelectOption { id: "warlock".into(), label: "Brujo".into(), description: None },
                        SelectOption { id: "wizard".into(), label: "Mago".into(), description: None },
                    ],
                }],
                traits_preview: vec!["Alcance Doble".into(), "Ignorar Cobertura".into()],
            },
            implementation: Box::new(SpellSniper),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "tavern_brawler".into(), name: "Peleador de Taberna".into(), source: "PHB2024".into(),
                description: Some("Golpes desarmados 1d4, objetos improvisados. Acción adicional para agarrar tras golpe.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Puño 1d4".into(), "Objetos Improvisados".into()],
            },
            implementation: Box::new(TavernBrawler),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "war_caster".into(), name: "Lanzador de Guerra".into(), source: "PHB2024".into(),
                description: Some("Req: lanzar conjuros. Ventaja en concentración, componentes somáticos ocupado, conjuro como AO.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: conjuros".into(), "Concentración Estable".into()],
            },
            implementation: Box::new(WarCaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "weapon_master".into(), name: "Maestro de Armas".into(), source: "PHB2024".into(),
                description: Some("+1 Fue o Des (máx 20), proficiencia en 4 armas a elegir.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Fue/Des".into(), "4 Proficiencias".into()],
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
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Carisma".into(), "Suplantación".into()],
            },
            implementation: Box::new(Actor),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "athlete".into(), name: "Atleta".into(), source: "PHB2024".into(),
                description: Some("+1 Fue o Des (máx 20), levantarse cuesta solo 5ft, trepar sin penalización.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Fue/Des".into(), "Trepar".into()],
            },
            implementation: Box::new(Athlete),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "dungeon_delver".into(), name: "Explorador de Mazmorras".into(), source: "PHB2024".into(),
                description: Some("Ventaja en detectar puertas secretas y trampas. Resistencia a daño de trampas.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Detectar Trampas".into(), "Resistencia Trampas".into()],
            },
            implementation: Box::new(DungeonDelver),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "durable".into(), name: "Resistente".into(), source: "PHB2024".into(),
                description: Some("+1 Con (máx 20), en descanso corto recuperar mínimo 2×mod.Con con dados de golpe.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Constitución".into(), "Dados de Golpe Mejorados".into()],
            },
            implementation: Box::new(Durable),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "heavily_armored".into(), name: "Armadura Pesada".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura media. +1 Fue (máx 20), proficiencia con armaduras pesadas.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Armadura Media".into(), "Prof. Armadura Pesada".into()],
            },
            implementation: Box::new(HeavilyArmored),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "heavy_armor_master".into(), name: "Maestro de Armadura Pesada".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura pesada. +1 Fue, reducir daño no mágico en 3 con armadura pesada.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Armadura Pesada".into(), "Reducción de Daño 3".into()],
            },
            implementation: Box::new(HeavyArmorMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "inspiring_leader".into(), name: "Líder Inspirador".into(), source: "PHB2024".into(),
                description: Some("Req: Car 13. Discurso 10min: hasta 6 criaturas ganan PG temp = nivel + mod Car.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Car 13".into(), "PG Temporales al Grupo".into()],
            },
            implementation: Box::new(InspiringLeader),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "keen_mind".into(), name: "Mente Aguda".into(), source: "PHB2024".into(),
                description: Some("+1 Int (máx 20), conocer siempre norte/hora/días, recordar todo lo visto en un mes.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Inteligencia".into(), "Memoria Perfecta".into()],
            },
            implementation: Box::new(KeenMind),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "lightly_armored".into(), name: "Armadura Ligera".into(), source: "PHB2024".into(),
                description: Some("+1 Fue o Des (máx 20), proficiencia con armaduras ligeras.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Fue/Des".into(), "Prof. Armadura Ligera".into()],
            },
            implementation: Box::new(LightlyArmored),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "lucky".into(), name: "Afortunado".into(), source: "PHB2024".into(),
                description: Some("3 puntos de suerte/día: tirar d20 extra en ataques, pruebas o salvaciones.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["3 Puntos de Suerte".into(), "Relanzar Tiradas".into()],
            },
            implementation: Box::new(Lucky),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "martial_adept".into(), name: "Adepto Marcial".into(), source: "PHB2024".into(),
                description: Some("2 maniobras de Maestro de Batalla y 1 dado de superioridad d6.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["2 Maniobras".into(), "Dado Superioridad d6".into()],
            },
            implementation: Box::new(MartialAdept),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "medium_armor_master".into(), name: "Maestro de Armadura Media".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura media. Sin desventaja en Sigilo, Des máx +3 a CA.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Armadura Media".into(), "Des +3 CA".into()],
            },
            implementation: Box::new(MediumArmorMaster),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "mobile".into(), name: "Ágil".into(), source: "PHB2024".into(),
                description: Some("+10ft velocidad, sin terreno difícil al cargar, no provocar AO vs criaturas atacadas.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+10ft Velocidad".into(), "Sin AO".into()],
            },
            implementation: Box::new(Mobile),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "moderately_armored".into(), name: "Armadura Intermedia".into(), source: "PHB2024".into(),
                description: Some("Req: prof armadura ligera. +1 Fue o Des (máx 20), prof con armadura media y escudo.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Armadura Ligera".into(), "Prof. Armadura Media + Escudo".into()],
            },
            implementation: Box::new(ModeratelyArmored),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "observant".into(), name: "Observador".into(), source: "PHB2024".into(),
                description: Some("+1 Int o Sab (máx 20), leer labios, +5 pasivo Percepción e Investigación.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+1 Int/Sab".into(), "+5 Percepción/Investigación".into()],
            },
            implementation: Box::new(Observant),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "resilient".into(), name: "Resistencia".into(), source: "PHB2024".into(),
                description: Some("Elegir atributo: +1 (máx 20) y proficiencia en salvaciones de ese atributo.".into()),
                image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "resilient.ability".into(),
                    label: "Atributo para salvación".into(),
                    options: vec![
                        SelectOption { id: "str".into(), label: "Fuerza".into(), description: None },
                        SelectOption { id: "dex".into(), label: "Destreza".into(), description: None },
                        SelectOption { id: "con".into(), label: "Constitución".into(), description: None },
                        SelectOption { id: "int".into(), label: "Inteligencia".into(), description: None },
                        SelectOption { id: "wis".into(), label: "Sabiduría".into(), description: None },
                        SelectOption { id: "cha".into(), label: "Carisma".into(), description: None },
                    ],
                }],
                traits_preview: vec!["+1 Atributo".into(), "Nueva Salvación".into()],
            },
            implementation: Box::new(Resilient),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "skilled".into(), name: "Hábil".into(), source: "PHB2024".into(),
                description: Some("Proficiencia en 3 habilidades o herramientas a elegir.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["3 Proficiencias".into()],
            },
            implementation: Box::new(Skilled),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "skulker".into(), name: "Merodeador".into(), source: "PHB2024".into(),
                description: Some("Req: Des 13. Esconderse en luz tenue, fallar ataque no revela posición.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Des 13".into(), "Sigilo Mejorado".into()],
            },
            implementation: Box::new(Skulker),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "tough".into(), name: "Fornido".into(), source: "PHB2024".into(),
                description: Some("+2 PG máximos por nivel (actual y futuro).".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["+2 PG por Nivel".into()],
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
                image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "elemental_adept.damage_type".into(),
                    label: "Tipo de daño elemental".into(),
                    options: vec![
                        SelectOption { id: "acid".into(), label: "Ácido".into(), description: None },
                        SelectOption { id: "cold".into(), label: "Frío".into(), description: None },
                        SelectOption { id: "fire".into(), label: "Fuego".into(), description: None },
                        SelectOption { id: "lightning".into(), label: "Relámpago".into(), description: None },
                        SelectOption { id: "thunder".into(), label: "Trueno".into(), description: None },
                    ],
                }],
                traits_preview: vec!["Req: conjuros".into(), "Ignorar Resistencia".into()],
            },
            implementation: Box::new(ElementalAdept),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "healer".into(), name: "Sanador".into(), source: "PHB2024".into(),
                description: Some("Usar botiquín gratis para estabilizar. Curar 1d6+4+DG PG una vez por criatura por descanso.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Estabilizar Gratis".into(), "Botiquín Mejorado".into()],
            },
            implementation: Box::new(Healer),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "magic_initiate".into(), name: "Iniciado en la Magia".into(), source: "PHB2024".into(),
                description: Some("Elegir clase: aprender 2 trucos y 1 conjuro de nivel 1. Lanzar el conjuro 1/día gratis.".into()),
                image_url: None,
                choices: vec![ChoiceSchema::SingleSelect {
                    id: "magic_initiate.class".into(),
                    label: "Clase de la que aprenderás".into(),
                    options: vec![
                        SelectOption { id: "bard".into(), label: "Bardo".into(), description: None },
                        SelectOption { id: "cleric".into(), label: "Clérigo".into(), description: None },
                        SelectOption { id: "druid".into(), label: "Druida".into(), description: None },
                        SelectOption { id: "sorcerer".into(), label: "Hechicero".into(), description: None },
                        SelectOption { id: "warlock".into(), label: "Brujo".into(), description: None },
                        SelectOption { id: "wizard".into(), label: "Mago".into(), description: None },
                    ],
                }],
                traits_preview: vec!["2 Trucos".into(), "Conjuro Nivel 1 Gratis".into()],
            },
            implementation: Box::new(MagicInitiate),
        },
        FeatEntry {
            catalog: CatalogEntry {
                id: "ritual_caster".into(), name: "Lanzador de Rituales".into(), source: "PHB2024".into(),
                description: Some("Req: Int o Sab 13. Libro con 2 rituales de nivel 1. Lanzarlos como ritual, copiar más.".into()),
                image_url: None, choices: vec![],
                traits_preview: vec!["Req: Int/Sab 13".into(), "Libro de Rituales".into()],
            },
            implementation: Box::new(RitualCaster),
        },
    ];
    for feat in magic_feats { registry.register_feat(feat).await; }
}
