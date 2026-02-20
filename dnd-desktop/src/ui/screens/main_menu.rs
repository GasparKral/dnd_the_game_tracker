use dioxus::prelude::*;
use dioxus_desktop::use_window;

#[component]
pub fn MainMenu() -> Element {
    let nav = navigator();
    rsx!(
        div{
            class: "flex flex-col items-center justify-center min-h-screen min-w-screen ",
            h1{"Dungeons & Dragons"},
            h2{"The Game Tracker"},
            div{
                class:"flex flex-col mt-12",
                button{
                    onclick: move|_| {nav.push("/new_campain");},
                    "Nueva Campaña"
                }
                button{
                    onclick: move|_| {nav.push("/load_campain");},
                    "Cargar Campaña"
                }
                button{
                    onclick: move|_| {nav.push("/options");},
                    "Opciones"}
                button{
                    onclick: move|_| use_window().close(),
                    "Salir"
                }
            }
        }
    )
}
