use crate::states::SharedState;
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn LoadCampainMenu() -> Element {
    let nav = navigator();
    let state = use_context::<SharedState>();

    // Estado de la pantalla
    let mut campaign_name = use_signal(|| String::new());
    let mut campaign_desc = use_signal(|| String::new());
    let mut campaign_chars = use_signal(|| 0usize);
    let mut campaign_updated = use_signal(|| String::new());
    let mut vault_path = use_signal(|| String::new());
    let mut new_vault_dir = use_signal(|| PathBuf::new());

    let mut status = use_signal(|| LoadStatus::Checking);
    let mut error_msg = use_signal(|| String::new());

    // Intentar leer la campaña guardada al montar el componente
    use_effect({
        let state = state.clone();
        move || {
            let state = state.clone();
            let mut campaign_name = campaign_name.clone();
            let mut campaign_desc = campaign_desc.clone();
            let mut campaign_chars = campaign_chars.clone();
            let mut campaign_updated = campaign_updated.clone();
            let mut vault_path = vault_path.clone();
            let mut status = status.clone();

            spawn(async move {
                match state.0.persistence.load().await {
                    Ok(Some(c)) => {
                        campaign_name.set(c.name.clone());
                        campaign_desc.set(c.description.clone());
                        campaign_chars.set(c.characters.len());
                        campaign_updated.set(
                            // Mostrar solo la fecha, no la hora completa
                            c.updated_at.get(..10).unwrap_or(&c.updated_at).to_string()
                        );
                        vault_path.set(
                            c.vault_path.unwrap_or_else(|| "Sin vault configurado".into())
                        );
                        status.set(LoadStatus::Found);
                    }
                    Ok(None) => status.set(LoadStatus::NotFound),
                    Err(e) => status.set(LoadStatus::Error(e.to_string())),
                }
            });
        }
    });

    rsx!(
        main {
            class: "min-w-screen min-h-screen flex flex-col items-center justify-center",

            div {
                class: "w-xl border rounded-md p-8 flex flex-col gap-6",

                h2 { class: "text-xl font-semibold", "Cargar Campaña" }

                match status() {
                    // ── Comprobando ───────────────────────────────────────
                    LoadStatus::Checking => rsx!(
                        div { class: "flex items-center gap-3 text-stone-400",
                            span { class: "animate-pulse", "⏳" }
                            span { "Buscando campaña guardada…" }
                        }
                    ),

                    // ── No hay campaña ────────────────────────────────────
                    LoadStatus::NotFound => rsx!(
                        div { class: "flex flex-col gap-4",
                            div { class: "text-stone-400 text-center py-4",
                                p { class: "text-2xl mb-2", "🗺️" }
                                p { "No hay ninguna campaña guardada." }
                                p { class: "text-sm mt-1", "Crea una nueva desde el menú principal." }
                            }
                            button {
                                class: "w-full px-4 py-2 border rounded",
                                onclick: move |_| { nav.go_back(); },
                                "Volver"
                            }
                        }
                    ),

                    // ── Error ─────────────────────────────────────────────
                    LoadStatus::Error(msg) => rsx!(
                        div { class: "flex flex-col gap-4",
                            div { class: "text-red-400 text-sm p-3 border border-red-800 rounded bg-red-950",
                                "⚠ Error al leer la campaña: {msg}"
                            }
                            button {
                                class: "w-full px-4 py-2 border rounded",
                                onclick: move |_| { nav.go_back(); },
                                "Volver"
                            }
                        }
                    ),

                    // ── Campaña encontrada ────────────────────────────────
                    LoadStatus::Found => rsx!(
                        div { class: "flex flex-col gap-5",

                            // Resumen de la campaña
                            div { class: "border rounded p-4 bg-stone-900 flex flex-col gap-2",
                                div { class: "flex items-center justify-between",
                                    h3 { class: "text-lg font-semibold", "{campaign_name.read()}" }
                                    span { class: "text-xs text-stone-500", "Guardada: {campaign_updated.read()}" }
                                }
                                if !campaign_desc.read().is_empty() {
                                    p { class: "text-sm text-stone-400", "{campaign_desc.read()}" }
                                }
                                div { class: "flex gap-6 mt-2 text-sm",
                                    span { class: "text-amber-400",
                                        "👥 {campaign_chars.read()} personaje(s)"
                                    }
                                    span { class: "text-stone-500 truncate",
                                        "📁 {vault_path.read()}"
                                    }
                                }
                            }

                            // Cambiar vault (opcional)
                            div { class: "flex flex-col gap-1",
                                label { class: "text-sm text-stone-400",
                                    "Cambiar vault de Obsidian (opcional)"
                                }
                                input {
                                    class: "w-full",
                                    r#type: "file",
                                    directory: true,
                                    oninput: move |e| {
                                        if let Some(path) = e.files().first() {
                                            new_vault_dir.set(PathBuf::from(path.clone().path()));
                                            error_msg.set(String::new());
                                        }
                                    },
                                }
                                if !new_vault_dir.read().as_os_str().is_empty() {
                                    span { class: "text-xs text-stone-500",
                                        "📁 {new_vault_dir.read().display()}"
                                    }
                                }
                            }

                            // Error inline
                            if !error_msg.read().is_empty() {
                                span { class: "text-red-400 text-sm", "{error_msg.read()}" }
                            }

                            // Botones
                            div { class: "flex gap-4 pt-2",
                                button {
                                    class: "flex-1 px-4 py-2 border rounded",
                                    onclick: move |_| { nav.go_back(); },
                                    "Volver"
                                }
                                button {
                                    class: "flex-1 px-4 py-2 rounded bg-amber-600 hover:bg-amber-500 font-semibold",
                                    onclick: {
                                        let state = state.clone();
                                        let nav = nav.clone();
                                        move |_| {
                                            let state = state.clone();
                                            let nav = nav.clone();
                                            let new_vault = new_vault_dir.read().clone();
                                            let mut error_msg = error_msg.clone();

                                            spawn(async move {
                                                // Si se eligió un nuevo vault, actualizar y abrir
                                                if !new_vault.as_os_str().is_empty() {
                                                    // Persistir la nueva ruta
                                                    if let Err(e) = state.0.persistence
                                                        .set_vault_path(new_vault.to_string_lossy().to_string())
                                                        .await
                                                    {
                                                        error_msg.set(format!("Error al guardar el vault: {e}"));
                                                        return;
                                                    }
                                                    // Abrirlo en el VaultManager
                                                    if let Err(e) = state.0.vault.open(new_vault).await {
                                                        error_msg.set(format!("Error al abrir el vault: {e}"));
                                                        return;
                                                    }
                                                } else if let Some(campaign) = state.0.persistence.current().await {
                                                    // Reabrir el vault guardado anteriormente si existe
                                                    if let Some(saved_path) = campaign.vault_path {
                                                        let p = std::path::PathBuf::from(&saved_path);
                                                        if p.is_dir() {
                                                            let _ = state.0.vault.open(p).await;
                                                        }
                                                    }
                                                }

                                                nav.push("/lore");
                                            });
                                        }
                                    },
                                    "Continuar campaña"
                                }
                            }
                        }
                    ),
                }
            }
        }
    )
}

// ---------------------------------------------------------------------------
// Estado interno de la pantalla
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
enum LoadStatus {
    Checking,
    Found,
    NotFound,
    Error(String),
}
