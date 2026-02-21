mod backend;
mod states;
mod ui;

use dioxus::prelude::*;
use std::sync::Arc;

pub fn main() {
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
