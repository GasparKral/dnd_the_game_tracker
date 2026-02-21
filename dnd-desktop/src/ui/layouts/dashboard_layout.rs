use crate::ui::Routes;
use dioxus::prelude::*;

#[component]
pub fn DashboardLayout() -> Element {
    let selectedTab: Routes = use_route();
    rsx!(
        main{
            class:"min-h-screen minw-screen flex flex-col",
            nav{
                class:"w-full flex gap-2 bg-stone-900 px-12 py-2 items-center sticky top-0 z-100",
                Link{class:"text-lg text-center px-2",to:"/lore","Lore"}
                Link{class:"text-lg text-center px-2",to:"/players","Jugadores"}
                Link{class:"text-lg text-center px-2",to:"/map","Mapa"}
            }
            Outlet::<Routes>{}
        }
    )
}
