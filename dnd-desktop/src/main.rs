mod backend;
mod states;
mod ui;

use dioxus::prelude::*;
use std::sync::Arc;

pub fn main() {
    let state = Arc::new(states::AppState::new());

    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .expect("Failed to create Tokio Runtime")
            .block_on(backend::run(state));
    });

    dioxus::launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet{href:asset!("/assets/tailwind.css")}
        Router::<ui::Routes>{}
    }
}
