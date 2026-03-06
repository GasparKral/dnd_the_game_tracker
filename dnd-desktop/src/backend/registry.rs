use shared::{
    api_types::catalog::{CatalogEntry, CatalogResponse, ChoiceSchema, SelectOption},
    traits::{background::Background, class::Class, race::Race},
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

// ---------------------------------------------------------------------------
// Registry central
// ---------------------------------------------------------------------------

/// Almacena todas las razas, clases y trasfondos disponibles en runtime.
/// Se pobla al arrancar con los defaults del PHB y con homebrew cargado del vault.
#[derive(Debug, Default)]
pub struct Registry {
    races: RwLock<HashMap<String, RaceEntry>>,
    classes: RwLock<HashMap<String, ClassEntry>>,
    backgrounds: RwLock<HashMap<String, BackgroundEntry>>,
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
use shared::models::defaults::races::{Dwarf, Elf, Human};

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
}
