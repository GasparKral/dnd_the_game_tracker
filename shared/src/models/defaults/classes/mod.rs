use crate::api_types::catalog::{CatalogEntry, ChoiceSchema, SelectOption, TraitDetail};
use crate::models::character::Player;
use crate::traits::class::Class;


// ── Bárbaro ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Barbarian;
impl Class for Barbarian {
    fn id(&self) -> &'static str { "barbarian" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "barbarian".into(), name: "Bárbaro".into(), source: "PHB2024".into(),
            description: Some("Guerrero feroz impulsado por rabia primaria. DG: d12. Salvaciones: FUE, CON.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "barbarian.primal_path".into(), label: "Senda Primaria".into(),
                    options: vec![
                        SelectOption::new("berserker",     "Berserker",          "Frenesí: ataque CaC adicional durante rabia a costa de agotamiento."),
                        SelectOption::new("totem_warrior", "Guerrero Totémico",  "Espíritu animal que otorga poderes especiales durante la rabia."),
                        SelectOption::new("world_tree",    "Árbol del Mundo",    "Conexión con Yggdrasil: movilidad táctica y protección a aliados."),
                        SelectOption::new("wild_magic",    "Magia Salvaje",       "Oleadas de magia impredecible al entrar en rabia."),
                        SelectOption::new("zealot",        "Fanatic",             "Poder divino en ataques; eres resucitado sin coste de material."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "barbarian.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("animal_handling", "Trato Animales"),
                        SelectOption::bare("athletics",       "Atletismo"),
                        SelectOption::bare("intimidation",    "Intimidación"),
                        SelectOption::bare("nature",          "Naturaleza"),
                        SelectOption::bare("perception",      "Percepción"),
                        SelectOption::bare("survival",        "Supervivencia"),
                    ],
                },
            ],
            required_choices: vec!["barbarian.skills".into()],
            traits_preview: vec!["Rabia".into(), "Defensa Sin Armadura".into(), "Ataque Descuidado".into()],
            traits_detail: vec![
                TraitDetail::new("Rabia", "Bono de daño CaC + resistencia a daño físico. Usos/desc.largo según nivel."),
                TraitDetail::new("Defensa Sin Armadura", "Sin armadura: CA = 10 + mod Des + mod Con."),
                TraitDetail::new("Ataque Descuidado", "Atacas con ventaja, pero los ataques contra ti también hasta tu siguiente turno."),
                TraitDetail::new("Ataque Extra", "Dos ataques al tomar la acción Atacar (nivel 5)."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Bardo ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Bard;
impl Class for Bard {
    fn id(&self) -> &'static str { "bard" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "bard".into(), name: "Bardo".into(), source: "PHB2024".into(),
            description: Some("Maestro de las artes y la magia. DG: d8. Salvaciones: DES, CAR.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "bard.college".into(), label: "Colegio Bárdico".into(),
                    options: vec![
                        SelectOption::new("lore",    "Colegio del Saber",    "Conocimiento y habilidades adicionales; contradecir rivales."),
                        SelectOption::new("valor",   "Colegio del Valor",    "Armas marciales, armadura media, inspiración en combate."),
                        SelectOption::new("glamour", "Colegio del Glamur",   "Magia féerica y dominio del carisma."),
                        SelectOption::new("swords",  "Colegio de las Espadas","Maniobras de combate con armas."),
                        SelectOption::new("spirits", "Colegio de los Espíritus","Comunión con espíritus para efectos aleatorios."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "bard.skills".into(), label: "Habilidades de clase (elige 3)".into(), min: 3, max: 3,
                    options: vec![
                        SelectOption::bare("acrobatics",      "Acrobacias"),
                        SelectOption::bare("animal_handling", "Trato Animales"),
                        SelectOption::bare("arcana",          "Arcanos"),
                        SelectOption::bare("athletics",       "Atletismo"),
                        SelectOption::bare("deception",       "Engaño"),
                        SelectOption::bare("history",         "Historia"),
                        SelectOption::bare("insight",         "Perspicacia"),
                        SelectOption::bare("intimidation",    "Intimidación"),
                        SelectOption::bare("investigation",   "Investigación"),
                        SelectOption::bare("medicine",        "Medicina"),
                        SelectOption::bare("nature",          "Naturaleza"),
                        SelectOption::bare("perception",      "Percepción"),
                        SelectOption::bare("performance",     "Interpretación"),
                        SelectOption::bare("persuasion",      "Persuasión"),
                        SelectOption::bare("religion",        "Religión"),
                        SelectOption::bare("sleight_of_hand", "Juego de Manos"),
                        SelectOption::bare("stealth",         "Sigilo"),
                        SelectOption::bare("survival",        "Supervivencia"),
                    ],
                },
            ],
            required_choices: vec!["bard.skills".into()],
            traits_preview: vec!["Inspiración Bárdica".into(), "Conjuros".into(), "Competencias Variadas".into()],
            traits_detail: vec![
                TraitDetail::new("Inspiración Bárdica", "Otorgas dado de inspiración (dXX) a un aliado para mejorar sus tiradas."),
                TraitDetail::new("Competencias Variadas", "Nivel 3: +2 competencias con dado de proficiencia."),
                TraitDetail::new("Contracanción", "Acción adicional: ventaja en ST vs. miedo/hechizo para aliados adyacentes."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Clérigo ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Cleric;
impl Class for Cleric {
    fn id(&self) -> &'static str { "cleric" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "cleric".into(), name: "Clérigo".into(), source: "PHB2024".into(),
            description: Some("Intermediario divino. DG: d8. Salvaciones: SAB, CAR.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "cleric.domain".into(), label: "Dominio Divino".into(),
                    options: vec![
                        SelectOption::new("life",     "Vida",       "Sanación y vitalidad; armadura pesada."),
                        SelectOption::new("light",    "Luz",        "Fuego y revelación; conjuros radiantes."),
                        SelectOption::new("war",      "Guerra",     "Poder marcial; ataque adicional."),
                        SelectOption::new("trickery", "Engaño",     "Ilusión, duplicidad y sigilo."),
                        SelectOption::new("nature",   "Naturaleza", "Animales y mundo natural; armadura pesada."),
                        SelectOption::new("tempest",  "Tormenta",   "Trueno y relámpago; armadura pesada."),
                        SelectOption::new("grave",    "Tumba",      "Frontera entre vida y muerte."),
                        SelectOption::new("order",    "Orden",      "Disciplina y ley; compulsión a aliados."),
                        SelectOption::new("knowledge","Conocimiento","Conocimiento arcano ampliado."),
                        SelectOption::new("twilight", "Crepúscolo",  "Protección en la oscuridad; tiendas reconfortantes."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "cleric.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("history",    "Historia"),
                        SelectOption::bare("insight",    "Perspicacia"),
                        SelectOption::bare("medicine",   "Medicina"),
                        SelectOption::bare("persuasion", "Persuasión"),
                        SelectOption::bare("religion",   "Religión"),
                    ],
                },
            ],
            required_choices: vec!["cleric.domain".into(), "cleric.skills".into()],
            traits_preview: vec!["Conjuros".into(), "Canal de Divinidad".into(), "Intervención Divina".into()],
            traits_detail: vec![
                TraitDetail::new("Canal de Divinidad", "Efectos poderosos (Turn Undead + los del Dominio). Usos según nivel."),
                TraitDetail::new("Conjuros de Dominio", "Lista de conjuros extra siempre preparados según dominio."),
                TraitDetail::new("Intervención Divina", "Nivel 10: pides la intervención de tu deidad (probabilidad = nivel %)."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Druida ────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Druid;
impl Class for Druid {
    fn id(&self) -> &'static str { "druid" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "druid".into(), name: "Druida".into(), source: "PHB2024".into(),
            description: Some("Guardián de la naturaleza y la transformación. DG: d8. Salvaciones: INT, SAB.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "druid.circle".into(), label: "Círculo Druídico".into(),
                    options: vec![
                        SelectOption::new("land",     "Círculo de la Tierra",      "Magia potenciada por terreno natural."),
                        SelectOption::new("moon",     "Círculo de la Luna",        "Formas bestiales poderosas desde nivel 2."),
                        SelectOption::new("stars",    "Círculo de las Estrellas",   "Constelaciones que otorgan poderes únicos."),
                        SelectOption::new("spores",   "Círculo de las Esporas",    "Muerte y renacimiento; infección fúngica."),
                        SelectOption::new("wildfire", "Círculo del Fuego Salvaje", "Fuego y renovación; familiar de llamas."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "druid.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("arcana",          "Arcanos"),
                        SelectOption::bare("animal_handling", "Trato Animales"),
                        SelectOption::bare("insight",         "Perspicacia"),
                        SelectOption::bare("medicine",        "Medicina"),
                        SelectOption::bare("nature",          "Naturaleza"),
                        SelectOption::bare("perception",      "Percepción"),
                        SelectOption::bare("religion",        "Religión"),
                        SelectOption::bare("survival",        "Supervivencia"),
                    ],
                },
            ],
            required_choices: vec!["druid.skills".into()],
            traits_preview: vec!["Forma Salvaje".into(), "Conjuros".into(), "Ritual Druídico".into()],
            traits_detail: vec![
                TraitDetail::new("Forma Salvaje", "Transformación en bestia. CR máx y opciones mejoran con nivel."),
                TraitDetail::new("Ritual Druídico", "Puedes lanzar como ritual conjuros con la etiqueta ritual."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Guerrero ──────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Fighter;
impl Class for Fighter {
    fn id(&self) -> &'static str { "fighter" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "fighter".into(), name: "Guerrero".into(), source: "PHB2024".into(),
            description: Some("Maestro del combate con todo tipo de armas y armaduras. DG: d10. Salvaciones: FUE, CON.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "fighter.style".into(), label: "Estilo de Combate".into(),
                    options: vec![
                        SelectOption::new("archery",         "Arquería",          "+2 a tiradas de ataque a distancia."),
                        SelectOption::new("defense",         "Defensa",            "+1 CA mientras usas armadura."),
                        SelectOption::new("dueling",         "Duelo",              "+2 al daño con arma a una mano y la otra libre."),
                        SelectOption::new("great_weapon",    "Gran Arma",          "Relanza 1s y 2s en dados de daño a dos manos."),
                        SelectOption::new("protection",      "Protección",         "Reacción: desventaja a ataque vs. aliado adyacente."),
                        SelectOption::new("two_weapon",      "Combate Dos Armas",  "Añades mod de atributo al daño del segundo ataque."),
                        SelectOption::new("blind_fighting",  "Combate a Ciegas",   "Visión ciega 3 m."),
                        SelectOption::new("interception",    "Interceptación",     "Reacción: reduce daño a aliado adyacente en 1d10+prof."),
                        SelectOption::new("unarmed",         "Combate Desarmado",  "Golpes desarmados infligen 1d6+Fue."),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "fighter.archetype".into(), label: "Arquetipo Marcial (nivel 3)".into(),
                    options: vec![
                        SelectOption::new("champion",        "Campeón",               "Críticos con 19-20; mayor atletismo."),
                        SelectOption::new("battle_master",   "Maestro de Batalla",     "Maniobras tácticas con dados de superioridad."),
                        SelectOption::new("eldritch_knight", "Caballero Sobrenatural", "Conjuros de abjuración y evocación de mago."),
                        SelectOption::new("psi_warrior",     "Guerrero Psíquico",     "Poderes psiónicos: telekinesis y escudos mentales."),
                        SelectOption::new("rune_knight",     "Caballero de Runas",     "Runas gigantes que otorgan poderes mágicos."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "fighter.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("acrobatics",     "Acrobacias"),
                        SelectOption::bare("animal_handling","Trato Animales"),
                        SelectOption::bare("athletics",      "Atletismo"),
                        SelectOption::bare("history",        "Historia"),
                        SelectOption::bare("insight",        "Perspicacia"),
                        SelectOption::bare("intimidation",   "Intimidación"),
                        SelectOption::bare("perception",     "Percepción"),
                        SelectOption::bare("survival",       "Supervivencia"),
                    ],
                },
            ],
            required_choices: vec!["fighter.style".into(), "fighter.skills".into()],
            traits_preview: vec!["Segundo Aliento".into(), "Oleada de Acción".into(), "Ataque Extra".into()],
            traits_detail: vec![
                TraitDetail::new("Segundo Aliento", "Acción adicional: recuperas 1d10+nivel PG. 1 uso/desc.corto."),
                TraitDetail::new("Oleada de Acción", "Acción adicional extra en tu turno. 1 uso/desc.corto."),
                TraitDetail::new("Ataque Extra", "Dos ataques al Atacar (nivel 5). Tres en nivel 11. Cuatro en nivel 20."),
                TraitDetail::new("Indomable", "Repetir una salvación fallida. 1 uso/desc.largo."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Monje ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Monk;
impl Class for Monk {
    fn id(&self) -> &'static str { "monk" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "monk".into(), name: "Monje".into(), source: "PHB2024".into(),
            description: Some("Artista marcial que canaliza ki. DG: d8. Salvaciones: FUE, DES.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "monk.tradition".into(), label: "Tradición Monástica (nivel 3)".into(),
                    options: vec![
                        SelectOption::new("open_hand",   "Mano Abierta",       "Maestría del combate desarmado: voltear, empujar, paralizar."),
                        SelectOption::new("shadow",      "Sombra",             "Magia oscura y sigilo; teleportación entre sombras."),
                        SelectOption::new("four_elements","Cuatro Elementos",  "Control elemental a través del ki."),
                        SelectOption::new("mercy",       "Misericordia",       "Sanar y dañar con el ki."),
                        SelectOption::new("astral_self", "Ser Astral",         "Proyección astral: brazos y visión etérea."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "monk.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("acrobatics", "Acrobacias"),
                        SelectOption::bare("athletics",  "Atletismo"),
                        SelectOption::bare("history",    "Historia"),
                        SelectOption::bare("insight",    "Perspicacia"),
                        SelectOption::bare("religion",   "Religión"),
                        SelectOption::bare("stealth",    "Sigilo"),
                    ],
                },
            ],
            required_choices: vec!["monk.skills".into()],
            traits_preview: vec!["Artes Marciales".into(), "Ki".into(), "Movimiento Sin Obstáculos".into()],
            traits_detail: vec![
                TraitDetail::new("Defensa Sin Armadura", "Sin armadura ni escudo: CA = 10 + Des + Sab."),
                TraitDetail::new("Artes Marciales", "Golpes desarmados infligen 1dX según nivel; ataques con ki."),
                TraitDetail::new("Ki", "Puntos = nivel. Activa Golpe Vertiginoso, Defensa de Paciente, Paso del Viento."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Paladín ───────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Paladin;
impl Class for Paladin {
    fn id(&self) -> &'static str { "paladin" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "paladin".into(), name: "Paladín".into(), source: "PHB2024".into(),
            description: Some("Guerrero sagrado bajo juramento inviolable. DG: d10. Salvaciones: SAB, CAR.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "paladin.oath".into(), label: "Sagrado Juramento (nivel 3)".into(),
                    options: vec![
                        SelectOption::new("devotion",   "Devoción",   "Honor y justicia; consagrar arma."),
                        SelectOption::new("ancients",   "Los Antiguos","Preservar la luz; resistencia a conjuros."),
                        SelectOption::new("vengeance",  "Venganza",    "Castigar a los malvados."),
                        SelectOption::new("glory",      "Gloria",      "Hazañas épicas; inspiración heroía."),
                        SelectOption::new("conquest",   "Conquista",   "Dominar el campo con miedo."),
                        SelectOption::new("redemption", "Redención",   "Convertir enemigos; evitar violencia."),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "paladin.style".into(), label: "Estilo de Combate".into(),
                    options: vec![
                        SelectOption::new("defense",      "Defensa",       "+1 CA con armadura."),
                        SelectOption::new("dueling",      "Duelo",         "+2 daño con arma una mano."),
                        SelectOption::new("great_weapon", "Gran Arma",     "Relanza 1s y 2s a dos manos."),
                        SelectOption::new("protection",   "Protección",    "Desventaja a ataques vs aliado."),
                        SelectOption::new("blind_fighting","Combate a Ciegas","Visión ciega 3 m."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "paladin.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("athletics",   "Atletismo"),
                        SelectOption::bare("insight",     "Perspicacia"),
                        SelectOption::bare("intimidation","Intimidación"),
                        SelectOption::bare("medicine",    "Medicina"),
                        SelectOption::bare("persuasion",  "Persuasión"),
                        SelectOption::bare("religion",    "Religión"),
                    ],
                },
            ],
            required_choices: vec!["paladin.oath".into(), "paladin.style".into(), "paladin.skills".into()],
            traits_preview: vec!["Imposición de Manos".into(), "Smite Divino".into(), "Sentido Divino".into()],
            traits_detail: vec![
                TraitDetail::new("Imposición de Manos", "Piscina PG = 5×nivel. Curas o eliminas enfermedades/venenos."),
                TraitDetail::new("Smite Divino", "Al acertar: gasta espacios para +2d8 por nivel del espacio."),
                TraitDetail::new("Aura de Protección", "Nivel 6: tú y aliados a 3 m suman mod Car a salvaciones."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Explorador ────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Ranger;
impl Class for Ranger {
    fn id(&self) -> &'static str { "ranger" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "ranger".into(), name: "Explorador".into(), source: "PHB2024".into(),
            description: Some("Rastreador que combina combate marcial con magia natural. DG: d10. Salvaciones: FUE, DES.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "ranger.enemy".into(), label: "Enemigo Favorito".into(),
                    options: vec![
                        SelectOption::bare("aberrations",  "Aberraciones"),
                        SelectOption::bare("beasts",       "Bestias"),
                        SelectOption::bare("dragons",      "Dragones"),
                        SelectOption::bare("elementals",   "Elementales"),
                        SelectOption::bare("fey",          "Hadas"),
                        SelectOption::bare("fiends",       "Infernales"),
                        SelectOption::bare("giants",       "Gigantes"),
                        SelectOption::bare("monstrosities","Monstruosidades"),
                        SelectOption::bare("undead",       "No-Muertos"),
                        SelectOption::new("two_humanoids", "Dos razas de Humanoides", "Ej: orcos y gnolls."),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "ranger.terrain".into(), label: "Explorador Natural (terreno)".into(),
                    options: vec![
                        SelectOption::bare("arctic",    "Ártico"),
                        SelectOption::bare("coast",     "Costa"),
                        SelectOption::bare("desert",    "Desierto"),
                        SelectOption::bare("forest",    "Bosque"),
                        SelectOption::bare("grassland", "Llanura"),
                        SelectOption::bare("mountain",  "Montaña"),
                        SelectOption::bare("swamp",     "Pantano"),
                        SelectOption::bare("underdark", "Inframundo"),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "ranger.conclave".into(), label: "Conclave de Exploradores (nivel 3)".into(),
                    options: vec![
                        SelectOption::new("hunter",        "Cazador",              "Especialista en eliminar amenazas específicas."),
                        SelectOption::new("beast_master",  "Maestro de Bestias",   "Compañero animal que combate a tu lado."),
                        SelectOption::new("gloom_stalker", "Acechador de Sombras", "Cazador en la oscuridad."),
                        SelectOption::new("fey_wanderer",  "Vagabundo Féerico",    "Magia feérica y resistencia mental."),
                        SelectOption::new("swarmkeeper",   "Guardián del Enjambre","Control con enjambre de criaturas diminutas."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "ranger.skills".into(), label: "Habilidades de clase (elige 3)".into(), min: 3, max: 3,
                    options: vec![
                        SelectOption::bare("animal_handling","Trato Animales"),
                        SelectOption::bare("athletics",      "Atletismo"),
                        SelectOption::bare("insight",        "Perspicacia"),
                        SelectOption::bare("investigation",  "Investigación"),
                        SelectOption::bare("nature",         "Naturaleza"),
                        SelectOption::bare("perception",     "Percepción"),
                        SelectOption::bare("stealth",        "Sigilo"),
                        SelectOption::bare("survival",       "Supervivencia"),
                    ],
                },
            ],
            required_choices: vec!["ranger.enemy".into(), "ranger.terrain".into(), "ranger.skills".into()],
            traits_preview: vec!["Enemigo Favorito".into(), "Explorador Natural".into(), "Ataque Extra".into()],
            traits_detail: vec![
                TraitDetail::new("Enemigo Favorito", "Ventaja en ST Sab (Supervivencia) para rastrear y pruebas de Int sobre el tipo elegido."),
                TraitDetail::new("Explorador Natural", "En terreno favorito: doble proficiencia en Naturaleza y Supervivencia."),
                TraitDetail::new("Ataque Extra", "Dos ataques al tomar la acción Atacar (nivel 5)."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Pícaro ────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Rogue;
impl Class for Rogue {
    fn id(&self) -> &'static str { "rogue" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "rogue".into(), name: "Pícaro".into(), source: "PHB2024".into(),
            description: Some("Maestro del sigilo y la astucia. DG: d8. Salvaciones: DES, INT.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "rogue.archetype".into(), label: "Arquetipo Pícaro (nivel 3)".into(),
                    options: vec![
                        SelectOption::new("thief",            "Ladrón",           "Robo, escalar rápido y usar objetos mágicos."),
                        SelectOption::new("assassin",         "Asesino",            "Eliminar objetivos por sorpresa con daño masivo."),
                        SelectOption::new("arcane_trickster", "Tramposo Arcano",    "Conjuros de ilusión y encantamiento."),
                        SelectOption::new("soulknife",        "Cuchilla Anímica",   "Hojas psíquicas y telepatía."),
                        SelectOption::new("swashbuckler",     "Fanfarrón",          "Duelo en solitario con encanto."),
                        SelectOption::new("inquisitive",      "Inquisidor",         "Detectar mentiras y vulnerabilidades."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "rogue.skills".into(), label: "Habilidades de clase (elige 4)".into(), min: 4, max: 4,
                    options: vec![
                        SelectOption::bare("acrobatics",      "Acrobacias"),
                        SelectOption::bare("athletics",       "Atletismo"),
                        SelectOption::bare("deception",       "Engaño"),
                        SelectOption::bare("insight",         "Perspicacia"),
                        SelectOption::bare("intimidation",    "Intimidación"),
                        SelectOption::bare("investigation",   "Investigación"),
                        SelectOption::bare("perception",      "Percepción"),
                        SelectOption::bare("performance",     "Interpretación"),
                        SelectOption::bare("persuasion",      "Persuasión"),
                        SelectOption::bare("sleight_of_hand", "Juego de Manos"),
                        SelectOption::bare("stealth",         "Sigilo"),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "rogue.expertise".into(), label: "Maestría — dobla proficiencia (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("acrobatics",      "Acrobacias"),
                        SelectOption::bare("athletics",       "Atletismo"),
                        SelectOption::bare("deception",       "Engaño"),
                        SelectOption::bare("insight",         "Perspicacia"),
                        SelectOption::bare("intimidation",    "Intimidación"),
                        SelectOption::bare("investigation",   "Investigación"),
                        SelectOption::bare("perception",      "Percepción"),
                        SelectOption::bare("persuasion",      "Persuasión"),
                        SelectOption::bare("sleight_of_hand", "Juego de Manos"),
                        SelectOption::bare("stealth",         "Sigilo"),
                        SelectOption::bare("thieves_tools",   "Herramientas de Ladrón"),
                    ],
                },
            ],
            required_choices: vec!["rogue.skills".into(), "rogue.expertise".into()],
            traits_preview: vec!["Ataque Furtivo".into(), "Jerga de Ladrones".into(), "Acción Astuta".into()],
            traits_detail: vec![
                TraitDetail::new("Ataque Furtivo", "1 vez/turno: +1d6 por cada 2 niveles al atacar con ventaja o con aliado adyacente al objetivo."),
                TraitDetail::new("Acción Astuta", "Acción adicional: Esconderse, Desengancharse o Correr."),
                TraitDetail::new("Reflejos Esquivos", "Nivel 7: fallas ST Des → la superas; la superas → daño a la mitad."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Hechicero ─────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Sorcerer;
impl Class for Sorcerer {
    fn id(&self) -> &'static str { "sorcerer" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "sorcerer".into(), name: "Hechicero".into(), source: "PHB2024".into(),
            description: Some("Lanzador nato cuyo poder emana de su linaje. DG: d6. Salvaciones: CON, CAR.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "sorcerer.origin".into(), label: "Origen Hechiceril".into(),
                    options: vec![
                        SelectOption::new("draconic",    "Linaje Dracónico",   "Sangre de dragón; escamas en piel, PG adicionales."),
                        SelectOption::new("wild_magic",  "Magia Salvaje",      "Oleadas de magia impredecible; Puntos de Suerte."),
                        SelectOption::new("divine_soul", "Alma Divina",        "Herencia celestial o infernal; conjuros de clérigo."),
                        SelectOption::new("clockwork",   "Alma de Relojería",  "Orden cósmico; neutralizar ventaja/desventaja."),
                        SelectOption::new("shadow",      "Magia de las Sombras","Plano de Sombras; resistencia al frío y necrosis."),
                        SelectOption::new("aberrant",    "Mente Aberrante",    "Conexión con Far Realm; psicosis en rabia psíquica."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "sorcerer.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("arcana",       "Arcanos"),
                        SelectOption::bare("deception",    "Engaño"),
                        SelectOption::bare("insight",      "Perspicacia"),
                        SelectOption::bare("intimidation", "Intimidación"),
                        SelectOption::bare("persuasion",   "Persuasión"),
                        SelectOption::bare("religion",     "Religión"),
                    ],
                },
            ],
            required_choices: vec!["sorcerer.origin".into(), "sorcerer.skills".into()],
            traits_preview: vec!["Conjuros".into(), "Puntos de Hechicería".into(), "Metamagia".into()],
            traits_detail: vec![
                TraitDetail::new("Puntos de Hechicería", "= nivel. Convierte espacios a puntos y viceversa."),
                TraitDetail::new("Metamagia", "Modifica conjuros: distante, gemelo, extendido, potenciado, sutil, etc."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Brujo ─────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Warlock;
impl Class for Warlock {
    fn id(&self) -> &'static str { "warlock" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "warlock".into(), name: "Brujo".into(), source: "PHB2024".into(),
            description: Some("Lanzador de pacto con entidad sobrenatural. DG: d8. Salvaciones: SAB, CAR.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "warlock.patron".into(), label: "Patrón Sobrenatural".into(),
                    options: vec![
                        SelectOption::new("fiend",         "El Señor Infernal", "Poder del mal; fuego y arma infernal."),
                        SelectOption::new("great_old_one", "El Gran Antiguo",   "Entidad cósmica; telepatía y susurros."),
                        SelectOption::new("archfey",       "El Archihada",      "Hada caprichosa; encantamiento y miedo."),
                        SelectOption::new("celestial",     "El Celestial",      "Ser de luz; sanación y conjuros radiantes."),
                        SelectOption::new("undying",       "El Inmortal",       "Resistencia a la muerte; no-muertos."),
                        SelectOption::new("fathomless",    "Lo Insondable",     "Poder oceánico; tentáculo espectral."),
                    ],
                },
                ChoiceSchema::SingleSelect {
                    id: "warlock.pact".into(), label: "Dádiva del Pacto (nivel 3)".into(),
                    options: vec![
                        SelectOption::new("blade",   "Pacto de la Hoja",   "Invocas un arma mágica que siempre está contigo."),
                        SelectOption::new("chain",   "Pacto de la Cadena", "Familiar especial (quasit, pseudodragon, sprite...)."),
                        SelectOption::new("tome",    "Pacto del Tomo",     "Libro de Sombras: 3 trucos adicionales de cualquier lista."),
                        SelectOption::new("talisman","Pacto del Talismán", "Amuleto: añade d4 a pruebas de habilidad fallidas."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "warlock.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("arcana",       "Arcanos"),
                        SelectOption::bare("deception",    "Engaño"),
                        SelectOption::bare("history",      "Historia"),
                        SelectOption::bare("intimidation", "Intimidación"),
                        SelectOption::bare("investigation","Investigación"),
                        SelectOption::bare("nature",       "Naturaleza"),
                        SelectOption::bare("religion",     "Religión"),
                    ],
                },
            ],
            required_choices: vec!["warlock.patron".into(), "warlock.pact".into(), "warlock.skills".into()],
            traits_preview: vec!["Conjuros de Pacto".into(), "Invocaciones Sobrenaturales".into(), "Dádiva del Patrón".into()],
            traits_detail: vec![
                TraitDetail::new("Conjuros de Pacto", "Pocos espacios (1–4) que se recuperan en descanso corto."),
                TraitDetail::new("Invocaciones Sobrenaturales", "Poderes mágicos permanentes. Elige 2 en nivel 2."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}

// ── Mago ──────────────────────────────────────────────────────────────────────

#[derive(Debug)] pub struct Wizard;
impl Class for Wizard {
    fn id(&self) -> &'static str { "wizard" }
    fn catalog_entry(&self) -> CatalogEntry {
        CatalogEntry {
            id: "wizard".into(), name: "Mago".into(), source: "PHB2024".into(),
            description: Some("Erudito arcano que estudia los secretos del universo. DG: d6. Salvaciones: INT, SAB.".into()),
            lore: None, image_url: None,
            choices: vec![
                ChoiceSchema::SingleSelect {
                    id: "wizard.school".into(), label: "Tradición Arcana (nivel 2)".into(),
                    options: vec![
                        SelectOption::new("evocation",    "Evocación",     "Conjuros de daño y energía; no dañar aliados."),
                        SelectOption::new("abjuration",   "Abjuración",    "Protección y contramagia; barrera arcana."),
                        SelectOption::new("illusion",     "Ilusión",        "Engañar sentidos y percepción."),
                        SelectOption::new("necromancy",   "Nigromancia",    "Muerte, no-muertos y oscuridad."),
                        SelectOption::new("conjuration",  "Conjuración",    "Convocar criaturas y teletransportación."),
                        SelectOption::new("divination",   "Adivinación",    "Ver el pasado, presente y futuro; Presagio."),
                        SelectOption::new("enchantment",  "Encantamiento",  "Controlar mentes y emociones."),
                        SelectOption::new("transmutation","Transmutación",  "Alterar materia y forma."),
                        SelectOption::new("chronurgy",    "Cronurgia",      "Manipular el tiempo; iniciativa y reacciones."),
                        SelectOption::new("graviturgy",   "Graviturgia",    "Controlar gravedad y peso."),
                        SelectOption::new("scribes",      "Escribas",       "Copiar conjuros con facilidad; cálamo vivo."),
                    ],
                },
                ChoiceSchema::MultiSelect {
                    id: "wizard.skills".into(), label: "Habilidades de clase (elige 2)".into(), min: 2, max: 2,
                    options: vec![
                        SelectOption::bare("arcana",       "Arcanos"),
                        SelectOption::bare("history",      "Historia"),
                        SelectOption::bare("insight",      "Perspicacia"),
                        SelectOption::bare("investigation","Investigación"),
                        SelectOption::bare("medicine",     "Medicina"),
                        SelectOption::bare("religion",     "Religión"),
                    ],
                },
            ],
            required_choices: vec!["wizard.school".into(), "wizard.skills".into()],
            traits_preview: vec!["Recuperación Arcana".into(), "Libro de Conjuros".into(), "Preparar Conjuros".into()],
            traits_detail: vec![
                TraitDetail::new("Recuperación Arcana", "1/día en descanso corto: recupera espacios cuya suma de niveles ≤ mitad del nivel de mago."),
                TraitDetail::new("Libro de Conjuros", "Empieza con 6 conjuros. Aprende 2 nuevos por nivel. Puede copiar pergaminos."),
                TraitDetail::new("Dominio de Conjuros", "Nivel 18: lanza 2 conjuros de nivel 1 y 2 sin gastar espacios."),
            ],
            speed_m: Some(9), size: None,
        }
    }
    fn apply(&self, _character: &mut Player) {}
}
