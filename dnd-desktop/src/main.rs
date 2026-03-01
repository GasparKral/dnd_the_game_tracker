mod backend;
mod cli;
mod states;
mod ui;

use cli::{CliArgs, DebugMode};
use dioxus::prelude::*;
use std::sync::Arc;
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

pub fn main() {
    let args = CliArgs::parse();

    // Configurar tracing según --debug
    let filter = match &args.debug {
        None => EnvFilter::new("warn"),
        Some(DebugMode::Server) => EnvFilter::new("axum=debug,tower_http=debug,warn"),
        Some(DebugMode::Info) => EnvFilter::new("dnd_desktop::backend::game_log=info,info"),
        Some(DebugMode::All) => EnvFilter::new("debug"),
    };

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(true))
        .with(filter)
        .init();

    let shared_state = states::SharedState(Arc::new(states::AppState::new()));

    let backend_state = shared_state.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(backend::run(backend_state));
    });

    dioxus::LaunchBuilder::new()
        .with_context(shared_state)
        .launch(app); // app es fn pointer, sin closure
}

fn app() -> Element {
    rsx! {
        document::Stylesheet{href:asset!("/assets/tailwind.css")}
        Router::<ui::Routes>{}
    }
}
