// ═══════════════════════════════════════════════════════════════════════════
// new_campain.rs — Pantalla de creación de nueva campaña
//
// Bug corregido:
//  • Después de crear la campaña en persistence.create(), si el usuario eligió
//    un vault se llama también a persistence.set_vault_path() para que el campo
//    vault_path quede persistido en el JSON (antes solo se abría el VaultManager
//    en memoria pero nunca se guardaba la ruta en disco).
// ═══════════════════════════════════════════════════════════════════════════

use crate::states::SharedState;
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn NewCampainMenu() -> Element {
    let nav = navigator();
    let state = use_context::<SharedState>();

    let mut c_name    = use_signal(String::new);
    let mut c_desc    = use_signal(String::new);
    let mut directory = use_signal(|| PathBuf::new());
    let mut error_msg = use_signal(String::new);
    let mut is_loading = use_signal(|| false);

    rsx!(
        main {
            class: "min-w-screen min-h-screen flex flex-col items-center justify-center",

            div {
                class: "w-xl border rounded-md p-8 flex flex-col gap-6",

                h2 { class: "text-xl font-semibold", "Nueva Campaña" }

                // ── Nombre ────────────────────────────────────────────────
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm text-stone-400", "Nombre de la campaña *" }
                    input {
                        class: "w-full px-3 py-2 border rounded bg-stone-900",
                        placeholder: "La Caída de un Continente…",
                        r#type: "text",
                        value: c_name(),
                        oninput: move |e| {
                            c_name.set(e.value());
                            error_msg.set(String::new());
                        },
                    }
                }

                // ── Descripción ───────────────────────────────────────────
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm text-stone-400", "Descripción (opcional)" }
                    textarea {
                        class: "w-full px-3 py-2 border rounded bg-stone-900 resize-none",
                        rows: "3",
                        placeholder: "Una breve sinopsis de la aventura…",
                        value: c_desc(),
                        oninput: move |e| c_desc.set(e.value()),
                    }
                }

                // ── Vault ─────────────────────────────────────────────────
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm text-stone-400", "Vault de Obsidian (opcional)" }
                    input {
                        class: "w-full",
                        r#type: "file",
                        directory: true,
                        oninput: move |e| {
                            if let Some(path) = e.files().first() {
                                directory.set(PathBuf::from(path.clone().path()));
                            }
                        },
                    }
                    if !directory.read().as_os_str().is_empty() {
                        span { class: "text-xs text-stone-500",
                            "📁 {directory.read().display()}"
                        }
                    }
                }

                // ── Error ─────────────────────────────────────────────────
                if !error_msg.read().is_empty() {
                    span { class: "text-red-400 text-sm", "{error_msg.read()}" }
                }

                // ── Botones ───────────────────────────────────────────────
                div { class: "flex gap-4 pt-2",
                    button {
                        class: "flex-1 px-4 py-2 border rounded",
                        disabled: is_loading(),
                        onclick: move |_| { nav.go_back(); },
                        "Volver"
                    }
                    button {
                        class: "flex-1 px-4 py-2 rounded bg-amber-600 hover:bg-amber-500 font-semibold",
                        disabled: is_loading(),
                        onclick: {
                            let state = state.clone();
                            let nav   = nav.clone();
                            move |_| {
                                let name = c_name.read().trim().to_string();
                                if name.is_empty() {
                                    error_msg.set("Todas las grandes historias tienen un nombre.".into());
                                    return;
                                }

                                let desc = c_desc.read().trim().to_string();
                                let vault_path = {
                                    let p = directory.read().clone();
                                    if p.as_os_str().is_empty() || !p.is_dir() {
                                        None
                                    } else {
                                        Some(p)
                                    }
                                };

                                let state      = state.clone();
                                let nav        = nav.clone();
                                let mut em     = error_msg.clone();
                                let mut loading = is_loading.clone();

                                spawn(async move {
                                    loading.set(true);

                                    // 1. Crear campaña en disco
                                    match state.0.persistence.create(name, desc).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            em.set(format!("Error al crear la campaña: {e}"));
                                            loading.set(false);
                                            return;
                                        }
                                    }

                                    // 2. Configurar vault (si se eligió uno)
                                    //    FIX: se persiste la ruta en el JSON con set_vault_path()
                                    //    ADEMÁS de abrir el VaultManager en memoria.
                                    if let Some(path) = vault_path {
                                        // Guardar ruta en el JSON de la campaña
                                        let path_str = path.to_string_lossy().to_string();
                                        if let Err(e) = state.0.persistence.set_vault_path(path_str).await {
                                            em.set(format!("Campaña creada pero error guardando ruta del vault: {e}"));
                                            loading.set(false);
                                            return;
                                        }

                                        // Abrir el VaultManager en memoria
                                        if let Err(e) = state.0.vault.open(path).await {
                                            em.set(format!("Campaña creada pero error abriendo el vault: {e}"));
                                            loading.set(false);
                                            return;
                                        }
                                    }

                                    loading.set(false);
                                    nav.push("/lore");
                                });
                            }
                        },
                        if is_loading() { "Creando…" } else { "Comenzar" }
                    }
                }
            }
        }
    )
}
