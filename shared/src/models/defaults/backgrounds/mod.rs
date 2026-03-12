use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::models::character::Player;
use crate::traits::background::Background;

fn language_options() -> Vec<SelectOption> {
    vec![
        SelectOption::bare("common",      "Común"),
        SelectOption::bare("dwarvish",    "Enáno"),
        SelectOption::bare("elvish",      "Élfico"),
        SelectOption::bare("giant",       "Gigante"),
        SelectOption::bare("gnomish",     "Gnómico"),
        SelectOption::bare("goblin",      "Goblin"),
        SelectOption::bare("halfling",    "Mediano"),
        SelectOption::bare("orc",         "Orco"),
        SelectOption::bare("abyssal",     "Abisal"),
        SelectOption::bare("celestial",   "Celestial"),
        SelectOption::bare("draconic",    "Dracónico"),
        SelectOption::bare("deep_speech", "Habla Profunda"),
        SelectOption::bare("infernal",    "Infernal"),
        SelectOption::bare("primordial",  "Primordial"),
        SelectOption::bare("sylvan",      "Silván"),
        SelectOption::bare("undercommon", "Común del Inframundo"),
    ]
}

fn tool_options() -> Vec<SelectOption> {
    vec![
        SelectOption::bare("thieves_tools",  "Herramientas de Ladrón"),
        SelectOption::bare("herbalism_kit",  "Kit de Herborista"),
        SelectOption::bare("alchemist",      "Suministros de Alquimista"),
        SelectOption::bare("smith",          "Herramientas de Herrero"),
        SelectOption::bare("mason",          "Herramientas de Cantero"),
        SelectOption::bare("woodcarver",     "Herramientas de Tallador"),
        SelectOption::bare("calligrapher",   "Suministros de Calligrafía"),
        SelectOption::bare("cartographer",   "Herramientas de Cartógrafo"),
        SelectOption::bare("cobbler",        "Herramientas de Zapatero"),
        SelectOption::bare("cook",           "Utensilios de Cocinero"),
        SelectOption::bare("glassblower",    "Herramientas de Soplador de Vidrio"),
        SelectOption::bare("jeweler",        "Herramientas de Joyero"),
        SelectOption::bare("leatherworker",  "Herramientas de Curtidor"),
        SelectOption::bare("painter",        "Suministros de Pintor"),
        SelectOption::bare("potter",         "Herramientas de Alfarero"),
        SelectOption::bare("tinker",         "Herramientas de Chapucero"),
        SelectOption::bare("weaver",         "Herramientas de Tejedor"),
        SelectOption::bare("gaming_set",     "Set de Juego (tipo)"),
        SelectOption::bare("musical_instrument", "Instrumento Musical (tipo)"),
        SelectOption::bare("navigator",      "Herramientas de Navegante"),
        SelectOption::bare("poisoner",       "Kit de Envenenador"),
        SelectOption::bare("disguise_kit",   "Kit de Disfraz"),
        SelectOption::bare("forgery_kit",    "Kit de Falsificación"),
    ]
}

// ── Acólito ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Acolyte;
impl Background for Acolyte {
    fn id(&self) -> &'static str { "acolyte" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "acolyte".into(), name: "Acólito".into(), source: "PHB2024".into(),
            description: Some("Pasaste tu vida al servicio de un templo. Competencias: Perspicacia, Religión.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "acolyte.language_1".into(), label: "Idioma adicional 1".into(),
                    options: language_options(),
                },
                ChoiceSchema::SingleSelect {
                    id: "acolyte.language_2".into(), label: "Idioma adicional 2".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["acolyte.language_1".into(), "acolyte.language_2".into()],
            traits_preview: vec!["Refugio de los Fieles".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Perspicacia, Religión. Herramienta: Suministros de Calligrafía."),
                TraitDetail::new("Refugio de los Fieles", "Puedes reclamar recursos de tu templo: alojamiento y curacón."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Artesano ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Artisan;
impl Background for Artisan {
    fn id(&self) -> &'static str { "artisan" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "artisan".into(), name: "Artesano".into(), source: "PHB2024".into(),
            description: Some("Maestro de un oficio. Competencias: Perspicacia, Persuasión. Herramienta de artesano.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "artisan.tool".into(), label: "Herramienta de artesano".into(),
                    options: tool_options(),
                },
                ChoiceSchema::SingleSelect {
                    id: "artisan.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["artisan.tool".into(), "artisan.language".into()],
            traits_preview: vec!["Posición del Gremio".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Perspicacia, Persuasión. Herramienta de artesano a elección."),
                TraitDetail::new("Posición del Gremio", "Acceso a talleres y redes comerciales de tu gremio."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Charlatán ─────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Charlatan;
impl Background for Charlatan {
    fn id(&self) -> &'static str { "charlatan" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "charlatan".into(), name: "Charlatán".into(), source: "PHB2024".into(),
            description: Some("Embaucador nato. Competencias: Engaño, Juego de Manos. Herramienta: Kit de Disfraz.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "charlatan.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["charlatan.language".into()],
            traits_preview: vec!["Falsa Identidad".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Engaño, Juego de Manos. Herramienta: Kit de Disfraz + Kit de Falsificación."),
                TraitDetail::new("Falsa Identidad", "Tienes una segunda identidad completa, con documentación y contactos."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Criminal ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Criminal;
impl Background for Criminal {
    fn id(&self) -> &'static str { "criminal" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "criminal".into(), name: "Criminal".into(), source: "PHB2024".into(),
            description: Some("Viviste al margen de la ley. Competencias: Sigilo, Engaño. Herramienta: Herramientas de Ladrón.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "criminal.specialty".into(), label: "Especialidad criminal".into(),
                    options: vec![
                        SelectOption::bare("blackmailer", "Chantajista"),
                        SelectOption::bare("burglar",     "Ladrón"),
                        SelectOption::bare("enforcer",    "Matones"),
                        SelectOption::bare("fence",       "Perista"),
                        SelectOption::bare("killer",      "Asesino"),
                        SelectOption::bare("pickpocket",  "Carterista"),
                        SelectOption::bare("smuggler",    "Contrabandista"),
                        SelectOption::bare("spy",         "Espía"),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "criminal.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["criminal.language".into()],
            traits_preview: vec!["Contacto Criminal".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Sigilo, Engaño. Herramienta: Herramientas de Ladrón + Set de Juego."),
                TraitDetail::new("Contacto Criminal", "Tienes un contacto fiable en la red criminal que puede conseguirte información."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Erudito ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Scholar;
impl Background for Scholar {
    fn id(&self) -> &'static str { "scholar" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "scholar".into(), name: "Erudito".into(), source: "PHB2024".into(),
            description: Some("Pasaste años estudiando en instituciones académicas. Competencias: Arcanos, Historia.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "scholar.specialty".into(), label: "Especialidad académica".into(),
                    options: vec![
                        SelectOption::bare("alchemist",   "Alquimia"),
                        SelectOption::bare("astronomer",  "Astronomía"),
                        SelectOption::bare("biologist",   "Biología"),
                        SelectOption::bare("cosmologist", "Cosmología"),
                        SelectOption::bare("historian",   "Historia"),
                        SelectOption::bare("magician",    "Magia"),
                        SelectOption::bare("philosopher", "Filosofía"),
                        SelectOption::bare("thaumaturgist","Taumaturgia"),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "scholar.language_1".into(), label: "Idioma adicional 1".into(),
                    options: language_options(),
                },
                ChoiceSchema::SingleSelect {
                    id: "scholar.language_2".into(), label: "Idioma adicional 2".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["scholar.language_1".into(), "scholar.language_2".into()],
            traits_preview: vec!["Investigador".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Arcanos, Historia. Dos idiomas adicionales."),
                TraitDetail::new("Investigador", "Sabes dónde buscar información. Acceso a bibliotecas y redes de eruditos."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Guardabosques ─────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Guide;
impl Background for Guide {
    fn id(&self) -> &'static str { "guide" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "guide".into(), name: "Guardabosques".into(), source: "PHB2024".into(),
            description: Some("Explorador de tierras salvajes. Competencias: Atletismo, Supervivencia. Herramienta: Herramientas de Cartográfo.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "guide.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["guide.language".into()],
            traits_preview: vec!["Vaga por la Naturaleza".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Atletismo, Supervivencia. Herramienta: Herramientas de Cartógrafo."),
                TraitDetail::new("Vaga por la Naturaleza", "Conoces tierras naturales: no puedes perderte y encuentras alimento."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Marinero ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Sailor;
impl Background for Sailor {
    fn id(&self) -> &'static str { "sailor" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "sailor".into(), name: "Marinero".into(), source: "PHB2024".into(),
            description: Some("Veterano de la vida en el mar. Competencias: Atletismo, Percepción. Herramienta: Herramientas de Navegante.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "sailor.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["sailor.language".into()],
            traits_preview: vec!["Pasaje Seguro".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Atletismo, Percepción. Herramienta: Herramientas de Navegante + Vehírculo Acuático."),
                TraitDetail::new("Pasaje Seguro", "Consigues pasaje gratuito en barcos para ti y tus compañeros."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Noble ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Noble;
impl Background for Noble {
    fn id(&self) -> &'static str { "noble" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "noble".into(), name: "Noble".into(), source: "PHB2024".into(),
            description: Some("Criado entre la aristocracia. Competencias: Historia, Persuasión. Herramienta: Set de Juego.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "noble.gaming_set".into(), label: "Tipo de Set de Juego".into(),
                    options: vec![
                        SelectOption::bare("dragonchess",   "Ajedrez Dracónico"),
                        SelectOption::bare("dice",          "Set de Dados"),
                        SelectOption::bare("cards",         "Baraja de Naipes"),
                        SelectOption::bare("three_dragon",  "Tres Dados del Dragón"),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "noble.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["noble.language".into()],
            traits_preview: vec!["Privilegio de Posición".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Historia, Persuasión. Herramienta: Set de Juego a elección."),
                TraitDetail::new("Privilegio de Posición", "La gente asume que tienes derecho a estar en cualquier lugar. Acceso a personas de alto rango."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Soldado ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Soldier;
impl Background for Soldier {
    fn id(&self) -> &'static str { "soldier" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "soldier".into(), name: "Soldado".into(), source: "PHB2024".into(),
            description: Some("Veterano de guerra. Competencias: Atletismo, Intimidación. Herramienta: Vehículo Terrestre.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "soldier.specialty".into(), label: "Especialidad militar".into(),
                    options: vec![
                        SelectOption::bare("officer",    "Oficial"),
                        SelectOption::bare("scout",      "Explorador"),
                        SelectOption::bare("infantry",   "Infantería"),
                        SelectOption::bare("cavalry",    "Caballería"),
                        SelectOption::bare("healer",     "Sanador"),
                        SelectOption::bare("quartermaster","Intendente"),
                        SelectOption::bare("standard_bearer","Abanderado"),
                        SelectOption::bare("support",    "Apoyo"),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "soldier.gaming_set".into(), label: "Set de Juego".into(),
                    options: vec![
                        SelectOption::bare("dragonchess", "Ajedrez Dracónico"),
                        SelectOption::bare("dice",        "Set de Dados"),
                        SelectOption::bare("cards",       "Baraja de Naipes"),
                    ],
                },
            ],
            required_choices: vec![],
            traits_preview: vec!["Rango Militar".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Atletismo, Intimidación. Herramienta: Vehículo Terrestre + Set de Juego."),
                TraitDetail::new("Rango Militar", "Tu rango sigue reconociéndose. Accedes a ejércitos aliados y aprovisionamiento."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Ermitaño ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Hermit;
impl Background for Hermit {
    fn id(&self) -> &'static str { "hermit" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "hermit".into(), name: "Ermitaño".into(), source: "PHB2024".into(),
            description: Some("Viviste en aislamiento y reflexión. Competencias: Medicina, Religión. Herramienta: Kit de Herborista.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "hermit.life".into(), label: "Vida de ermitaño".into(),
                    options: vec![
                        SelectOption::bare("meditation",    "Búsqueda de Iluminación"),
                        SelectOption::bare("exile",         "Exilio"),
                        SelectOption::bare("pilgrimage",    "Peregrinación"),
                        SelectOption::bare("seclusion",     "Recluido en Monasterio"),
                        SelectOption::bare("nature_watch",  "Guardia de la Naturaleza"),
                        SelectOption::bare("oracle",        "Oráculo o Profeta"),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "hermit.language".into(), label: "Idioma adicional".into(),
                    options: language_options(),
                },
            ],
            required_choices: vec!["hermit.language".into()],
            traits_preview: vec!["Descubrimiento".into()],
            traits_detail: vec![
                TraitDetail::new("Competencias", "Medicina, Religión. Herramienta: Kit de Herborista."),
                TraitDetail::new("Descubrimiento", "Tu retiro reveló una verdad única: una verdad cósmica, sobre los dioses o las fuerzas que rigen el mundo."),
            ],
            speed_m: None, size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}
