mod load_campain;
mod lore;
mod main_menu;
mod new_campain;
mod options_menu;

use super::layouts::dashboard_layout::DashboardLayout;
use dioxus::prelude::*;
use load_campain::*;
use lore::*;
use main_menu::*;
use new_campain::*;
use options_menu::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Routes {
    #[route("/")]
    MainMenu,
    #[route("/options")]
    OptionMenu,
    #[route("/new_campain")]
    NewCampainMenu,
    #[route("/load_campain")]
    LoadCampainMenu,

    #[layout(DashboardLayout)]
    #[route("/lore")]
    Lore,
}
