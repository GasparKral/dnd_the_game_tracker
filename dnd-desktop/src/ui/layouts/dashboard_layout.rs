use crate::ui::Routes;
use dioxus::prelude::*;

#[component]
pub fn DashboardLayout() -> Element {
    let selectedTab: Routes = use_route();
    rsx!(
        main{
            class:"min-h-screen minw-screen flex flex-col",
            nav{
                class:"w-full",
                Link{to:"/lore","Lore"}
                Link{to:"/players","Jugadores"}
                Link{to:"/map","Mapa"}
            }
            Outlet::<Routes>{}
        }
    )
}
