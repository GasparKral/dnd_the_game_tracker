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
                Link{class:"text-lg text-center px-2",to:"/map","Mapa"}
                // Separador flexible
                div { class: "flex-1" }
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
