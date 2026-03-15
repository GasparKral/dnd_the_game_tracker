use crate::ui::components::DiceRoller;
use crate::ui::{CloudflareTunnelButton, Routes};
use dioxus::prelude::*;

#[component]
pub fn DashboardLayout() -> Element {
    rsx!(
        main{
            class:"min-h-screen minw-screen flex flex-col",
            nav{
                class:"w-full flex gap-2 bg-stone-900 px-12 py-2 items-center fixed top-0 z-100",
                Link{class:"text-lg text-center px-2",to:"/lore","Lore"}
                Link{class:"text-lg text-center px-2",to:"/players","Jugadores"}
                Link{class:"text-lg text-center px-2",to:"/items","Objetos"}
                Link{class:"text-lg text-center px-2",to:"/combat","⚔ Combate"}
                Link{class:"text-lg text-center px-2",to:"/map","Mapa"}
                // Separador flexible
                div { class: "flex-1" }
                // Botón para desplegar/ocultar el panel de dados
                DiceRollerToggle {}
                CloudflareTunnelButton {}
                Link{
                    class:"text-sm text-stone-400 hover:text-stone-100 px-3 py-1 border border-stone-700 rounded",
                    to:"/",
                    "↩ Inicio"
                }
            }
            div{
                class:"mt-12",
                Outlet::<Routes>{}
            }
        }
    )
}

/// Botón flotante que muestra/oculta el DiceRoller como panel superpuesto.
#[component]
fn DiceRollerToggle() -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { style: "position:relative;",
            {{
                let btn_bg = if *open.read() { "#292524" } else { "#111110" };
                rsx! {
                    button {
                        onclick: move |_| { *open.write() ^= true; },
                        style: "background:{btn_bg}; border:1px solid #b45309;
                                 border-radius:8px; color:#fbbf24;
                                 padding:4px 10px; font-size:0.8rem; cursor:pointer;",
                        "🎲 Dados"
                    }
                }
            }}
            if *open.read() {
                div {
                    style: "position:absolute; top:calc(100% + 8px); right:0; z-index:200;",
                    DiceRoller {}
                }
            }
        }
    }
}
