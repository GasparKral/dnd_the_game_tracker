use crate::persistence::CampaignSummaryEntry;
use crate::states::SharedState;
use dioxus::prelude::*;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Pantalla: lista de todas las campañas guardadas
// ---------------------------------------------------------------------------

#[component]
pub fn LoadCampainMenu() -> Element {
    let nav = navigator();
    let state = use_context::<SharedState>();

    let mut campaigns: Signal<Vec<CampaignSummaryEntry>> = use_signal(|| vec![]);
    let mut status = use_signal(|| PageStatus::Loading);
    let mut confirm_delete: Signal<Option<String>> = use_signal(|| None); // filename a confirmar
    let mut error_msg = use_signal(|| String::new());

    // Cargar lista al montar
    use_effect({
        let state = state.clone();
        move || {
            let state = state.clone();
            spawn(async move {
                match state.0.persistence.list_campaigns().await {
                    Ok(list) => {
                        campaigns.set(list.clone());
                        status.set(if list.is_empty() { PageStatus::Empty } else { PageStatus::Ready });
                    }
                    Err(e) => status.set(PageStatus::Error(e.to_string())),
                }
            });
        }
    });

    rsx!(
        main {
            class: "min-w-screen min-h-screen flex flex-col items-center justify-center",

            div {
                class: "w-2xl border border-stone-700 rounded-xl p-10 flex flex-col gap-8 bg-stone-900/60",

                // ── Cabecera ──────────────────────────────────────────────
                div { class: "flex items-center justify-between",
                    h2 { class: "text-xl font-semibold", "Campañas Guardadas" }
                    button {
                        class: "px-4 py-2 text-sm border border-stone-600 rounded-lg text-stone-200 hover:bg-stone-700 hover:text-white transition-colors",
                        onclick: move |_| nav.go_back(),
                        "← Volver"
                    }
                }

                match status() {
                    PageStatus::Loading => rsx!(
                        div { class: "flex items-center gap-3 text-stone-400 justify-center py-8",
                            span { class: "animate-pulse", "⏳" }
                            span { "Buscando campañas…" }
                        }
                    ),

                    PageStatus::Error(msg) => rsx!(
                        div { class: "text-red-400 text-sm p-3 border border-red-800 rounded bg-red-950",
                            "⚠ {msg}" }
                    ),

                    PageStatus::Empty => rsx!(
                        div { class: "text-stone-400 text-center py-8",
                            p { class: "text-3xl mb-3", "🗺️" }
                            p { "No hay ninguna campaña guardada." }
                            p { class: "text-sm mt-1 text-stone-500",
                                "Crea una nueva desde el menú principal." }
                        }
                    ),

                    PageStatus::Ready => rsx!(
                        div { class: "flex flex-col gap-3",
                            for entry in campaigns.read().iter() {
                                {
                                    let entry = entry.clone();
                                    let filename = entry.filename.clone();
                                    let filename_del = filename.clone();
                                    let state_load = state.clone();
                                    let nav_load = nav.clone();

                                    rsx!(
                                        div {
                                            key: "{filename}",
                                            class: "border rounded-lg p-4 bg-stone-900 flex items-start justify-between gap-4",

                                            // Info campaña
                                            div { class: "flex flex-col gap-1 flex-1 min-w-0",
                                                h3 { class: "font-semibold text-amber-300 truncate",
                                                    "{entry.name}" }
                                                if !entry.description.is_empty() {
                                                    p { class: "text-sm text-stone-400 truncate",
                                                        "{entry.description}" }
                                                }
                                                div { class: "flex gap-4 text-xs text-stone-500 mt-1",
                                                    span { "👥 {entry.character_count} personaje(s)" }
                                                    span { "📅 {entry.updated_at.get(..10).unwrap_or(&entry.updated_at)}" }
                                                }
                                            }

                                            // Acciones
                                            div { class: "flex gap-2 flex-shrink-0 items-center",
                                                button {
                                                    class: "px-4 py-2 text-sm rounded-lg bg-amber-700 hover:bg-amber-600 text-white font-semibold shadow transition-colors",
                                                    onclick: move |_| {
                                                        let state = state_load.clone();
                                                        let nav = nav_load.clone();
                                                        let fname = filename.clone();
                                                        let mut err = error_msg;
                                                        spawn(async move {
                                                            match state.0.persistence.load_campaign(&fname).await {
                                                                Ok(campaign) => {
                                                                    // Reabrir vault si existe
                                                                    if let Some(vp) = campaign.vault_path {
                                                                        let p = PathBuf::from(&vp);
                                                                        if p.is_dir() {
                                                                            let _ = state.0.vault.open(p).await;
                                                                        }
                                                                    }
                                                                    nav.push("/lore");
                                                                }
                                                                Err(e) => err.set(e.to_string()),
                                                            }
                                                        });
                                                    },
                                                    "Cargar"
                                                }
                                                button {
                                                    class: "px-3 py-2 text-sm rounded-lg border border-red-800 text-red-400 hover:bg-red-950 hover:border-red-600 transition-colors",
                                                    onclick: move |_| confirm_delete.set(Some(filename_del.clone())),
                                                    "🗑 Borrar"
                                                }
                                            }
                                        }
                                    )
                                }
                            }
                        }
                    ),
                }

                if !error_msg.read().is_empty() {
                    span { class: "text-red-400 text-sm", "{error_msg.read()}" }
                }
            }

            // ── Modal confirmación eliminar ────────────────────────────────
            if let Some(fname) = confirm_delete.read().clone() {
                {
                    let state_del = state.clone();
                    let fname_label = fname.replace(".json", "").replace("_", " ");
                    rsx!(
                        div {
                            class: "fixed inset-0 bg-black/70 flex items-center justify-center z-50",
                            div {
                                class: "bg-stone-900 border border-stone-700 rounded-2xl p-8 w-96 flex flex-col gap-6 shadow-2xl",
                                h3 { class: "text-xl font-bold text-red-400 tracking-tight",
                                    "⚠️ ¿Eliminar campaña?" }
                                p { class: "text-sm text-stone-300 leading-relaxed",
                                    "Se eliminará permanentemente "
                                    span { class: "font-semibold text-amber-300", "\"{fname_label}\"" }
                                    ". Esta acción no se puede deshacer y perderás todos los personajes de esa campaña."
                                }
                                div { class: "flex gap-3 justify-end pt-2",
                                    button {
                                        class: "px-5 py-2.5 text-sm border border-stone-600 rounded-lg text-stone-200 hover:bg-stone-700 transition-colors",
                                        onclick: move |_| confirm_delete.set(None),
                                        "Cancelar"
                                    }
                                    button {
                                        class: "px-5 py-2.5 text-sm rounded-lg bg-red-700 hover:bg-red-600 font-semibold text-white shadow transition-colors",
                                        onclick: move |_| {
                                            let state = state_del.clone();
                                            let f = fname.clone();
                                            spawn(async move {
                                                let _ = state.0.persistence.delete_campaign(&f).await;
                                                // Recargar lista
                                                if let Ok(list) = state.0.persistence.list_campaigns().await {
                                                    campaigns.set(list.clone());
                                                    status.set(if list.is_empty() { PageStatus::Empty } else { PageStatus::Ready });
                                                }
                                            });
                                            confirm_delete.set(None);
                                        },
                                        "Eliminar"
                                    }
                                }
                            }
                        }
                    )
                }
            }
        }
    )
}

// ---------------------------------------------------------------------------
// Estado interno
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
enum PageStatus {
    Loading,
    Ready,
    Empty,
    Error(String),
}
