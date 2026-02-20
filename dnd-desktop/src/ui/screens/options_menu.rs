use dioxus::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum OptionTab {
    #[default]
    General,
    Graphics,
    Sound,
}

#[component]
pub fn OptionMenu() -> Element {
    let mut tab = use_signal(|| OptionTab::default());
    let nav = navigator();

    rsx!(
        div{
            class:"flex flex-col items-center justify-center min-h-screen min-w-screen",
            div{
                class:"flex flex-col w-2xl h-1/2 rounded-md border",
                nav{
                    class:"flex bg-dungeon items-center",
                    button{class:"border-r border-b px-1.5 text-center py-0.5 w-full hover:text-aurum hover:bg-gold",onclick: move|_|{tab.set(OptionTab::General)},"Generales"}
                    button{class:"border-r border-b px-1.5 text-center py-0.5 w-full hover:text-aurum hover:bg-gold",onclick: move|_|{tab.set(OptionTab::Graphics)},"Graficos"}
                    button{class:"border-r border-b px-1.5 text-center py-0.5 w-full hover:text-aurum hover:bg-gold",onclick: move|_|{tab.set(OptionTab::Sound)},"Sonido"}
                }
                div{class:"px-8 pb-4",
                    if tab() == OptionTab::General{
                        GeneralOptions {  }
                    } else if tab() == OptionTab::Graphics{
                        GraphicsOptions {  }
                    } else {
                        SoundOptions {  }
                    }
                }
                footer{
                    class:"flex items-center justify-center",
                    button{
                        onclick: move|_|{nav.go_back()},
                        class: "w-full",
                        "Salir"
                    }
                }
            }
        }
    )
}

#[component]
fn GeneralOptions() -> Element {
    rsx!()
}

#[component]
fn GraphicsOptions() -> Element {
    rsx!()
}

#[component]
fn SoundOptions() -> Element {
    rsx!()
}
