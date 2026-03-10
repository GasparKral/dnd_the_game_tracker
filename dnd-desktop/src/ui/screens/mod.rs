mod items;
mod items_form;
mod load_campain;
mod lore;
mod main_menu;
mod new_campain;
mod options_menu;
mod players;
pub mod player_inventory;
pub mod player_spells;

use super::layouts::dashboard_layout::DashboardLayout;
use dioxus::prelude::*;
use items::*;
use items_form::CreateItemModal;
use load_campain::*;
use lore::*;
use main_menu::*;
use new_campain::*;
use options_menu::*;
use players::*;

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
    #[route("/players")]
    Players,
    #[route("/items")]
    ItemsScreen,
}
