use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn NewCampainMenu() -> Element {
    let nav = navigator();
    let mut c_name = use_signal(|| "".to_string());
    let mut error_msg = use_signal(|| "".to_string());
    let mut directory = use_signal(|| PathBuf::new());

    rsx!(
        main{
            class:"min-w-screen min-h-screen flex flex-col items-center justify-center",
            form{
                class:"w-xl border rounded-md p-4 flex flex-col gap-2",
                fieldset{
                    class:"flex flex-col relative",
                    legend{class:"absolute top-[-2.5rem] left-[-1rem]","crea una nueva campaña"},
                    div{
                        class:"flex flex-col gap-2",
                        label{
                            class:"flex gap-2",
                            "nombre de la campaña:"
                            input{
                                class:"w-xs",
                                placeholder:"caída de un continente..",
                                type:"text",
                                oninput: move|event|{c_name.set(event.value());},
                                value: c_name()
                            }
                        },
                        label{
                            class:"flex gap-2",
                            "donde reside el mundo:"
                            input{
                                class:"w-xs",
                                type:"file",
                                directory:true,
                                oninput: move|event|{
                                  directory.set(PathBuf::from(event.files()[0].clone().path()));
                                },
                            }
                        }
                    },
                }
                span{
                    class:"text-blood self-center",
                    "{error_msg.read()}"
                }
            }
            div{
                class:"flex gap-8",
                button{
                    class:"w-full px-4 py-2",
                    onclick: move|_|{nav.go_back();},
                    "Volver"
                }
                button{
                    class:"w-full px-4 py-2",
                    onclick: move|_|{
                        if c_name.read().is_empty(){
                            error_msg.set("todas las grandes historias tienen un nombre".to_string());
                            return;
                        }
                        if !directory.read().exists() || !directory.read().is_dir(){
                            error_msg.set("en algun sitio debe de existir".to_string());
                            return;
                        }
                        error_msg.set("".to_string());
                        nav.push("/lore");
                    },
                    "Comenzar"
                }
            }
        }
    )
}
