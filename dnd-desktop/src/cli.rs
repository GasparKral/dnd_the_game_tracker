use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum DebugMode {
    Server, // solo logs del backend/WS/Axum
    Info,   // logs de eventos de juego (combate, conexiones, tiradas)
    All,    // todo lo anterior + verbose interno
}

pub struct CliArgs {
    pub debug: Option<DebugMode>,
}

impl CliArgs {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        let debug = args.windows(2).find_map(|w| {
            if w[0] == "--debug" {
                match w[1].as_str() {
                    "server" => Some(DebugMode::Server),
                    "info" => Some(DebugMode::Info),
                    "all" => Some(DebugMode::All),
                    _ => None,
                }
            } else {
                None
            }
        });
        Self { debug }
    }
}
