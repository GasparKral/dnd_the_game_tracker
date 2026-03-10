use crate::states::SharedState;
use crate::vault::entry::VaultEntry;
use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::OnceLock;

// ── Helpers de UI ────────────────────────────────────────────────────────────

fn resolve_obsidian_links(input: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap());
    re.replace_all(input, |caps: &regex::Captures| {
        let target = &caps[1];
        let label = caps.get(2).map_or(target, |m| m.as_str());
        let encoded = target.replace(' ', "%20");
        format!(
            "<a href='obsidian://{encoded}' class='obsidian-link text-amber-400 underline hover:text-amber-200 cursor-pointer'>{label}</a>"
        )
    })
    .to_string()
}

fn md_to_html(path: &PathBuf) -> String {
    let Ok(source) = std::fs::read_to_string(path) else {
        return "<p class='text-red-400'>No se pudo leer el archivo.</p>".to_string();
    };
    // Separamos el frontmatter del cuerpo antes de renderizar
    let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
    let content = matter.parse(&source).content;
    let content = resolve_obsidian_links(&content);
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(&content, opts);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

fn all_ancestors_expanded(entry: &VaultEntry, expanded: &HashSet<PathBuf>, root: &PathBuf) -> bool {
    let mut current = entry.path.parent();
    while let Some(p) = current {
        if p == root {
            break;
        }
        if !expanded.contains(p) {
            return false;
        }
        current = p.parent();
    }
    true
}

fn compute_filtered_expanded(entries: &[VaultEntry], filter: &str) -> HashSet<PathBuf> {
    let filter_lower = filter.to_lowercase();
    let mut visible_dirs: HashSet<PathBuf> = HashSet::new();
    for entry in entries.iter().filter(|e| !e.path.is_dir()) {
        if entry.display_name().to_lowercase().contains(&filter_lower) {
            let mut current = entry.path.parent();
            while let Some(p) = current {
                if visible_dirs.contains(p) {
                    break;
                }
                visible_dirs.insert(p.to_path_buf());
                current = p.parent();
            }
        }
    }
    visible_dirs
}

// ── Subcomponente: árbol de archivos ─────────────────────────────────────────

#[component]
fn VaultTree(
    entries: Vec<VaultEntry>,
    root: PathBuf,
    filter: String,
    selected: Signal<Option<PathBuf>>,
    expanded: Signal<HashSet<PathBuf>>,
) -> Element {
    if entries.is_empty() {
        return rsx! {
            div { class: "p-4 text-stone-500 text-sm", "El vault está vacío o no se ha configurado." }
        };
    }

    let is_filtering = !filter.is_empty();

    let effective_expanded: HashSet<PathBuf> = if is_filtering {
        compute_filtered_expanded(&entries, &filter)
    } else {
        expanded.read().clone()
    };

    let has_matches = if is_filtering {
        entries.iter().any(|e| {
            e.display_name()
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
    } else {
        true
    };

    // Construimos una vista de árbol a partir de los VaultEntry planos.
    // Un entry es "directorio virtual" si tiene hijos con la misma ruta padre.
    // Usamos relative_path para calcular profundidad.
    rsx! {
        nav { class: "flex flex-col py-2",
            if !has_matches {
                div {
                    class: "p-4 text-stone-500 text-sm italic",
                    "Sin resultados para \"{filter}\""
                }
            } else {
                for entry in entries.iter() {
                    {
                        let entry = entry.clone();
                        let depth = entry.relative_path.components().count().saturating_sub(1);
                        let eff_exp = effective_expanded.clone();
                        let root = root.clone();

                        let is_file = entry.path.is_file();
                        let path = entry.path.clone();
                        let name = entry.display_name().to_string();
                        let matches_filter = name.to_lowercase().contains(&filter.to_lowercase());

                        if is_file {
                            let is_active = selected.read().as_ref() == Some(&path);
                            let visible = if is_filtering {
                                matches_filter && all_ancestors_expanded(&entry, &eff_exp, &root)
                            } else {
                                all_ancestors_expanded(&entry, &eff_exp, &root)
                            };

                            if visible {
                                rsx! {
                                    button {
                                        key: "{path:?}",
                                        class: if is_active {
                                            "w-full text-left py-1 text-sm bg-stone-800 text-amber-300 truncate flex items-center gap-1"
                                        } else {
                                            "w-full text-left py-1 text-sm hover:bg-stone-800 hover:text-stone-100 truncate flex items-center gap-1"
                                        },
                                        style: "padding-left: {depth * 12 + 4}px",
                                        onclick: move |_| selected.set(Some(path.clone())),
                                        span { class: "text-stone-600 w-3", "·" }
                                        "{name}"
                                    }
                                }
                            } else { rsx! {} }
                        } else {
                            // Directorio
                            let dir_path = path.clone();
                            let is_open = eff_exp.contains(&dir_path);
                            let visible = if is_filtering {
                                eff_exp.contains(&dir_path)
                                    || eff_exp.iter().any(|p| p.starts_with(&dir_path))
                            } else {
                                depth == 0 || all_ancestors_expanded(&entry, &eff_exp, &root)
                            };

                            if visible && depth > 0 {
                                rsx! {
                                    button {
                                        key: "{dir_path:?}",
                                        class: "w-full text-left py-1 text-xs uppercase tracking-wider text-stone-500 mt-1 hover:text-stone-300 flex items-center gap-1",
                                        style: "padding-left: {depth * 12}px",
                                        onclick: move |_| {
                                            if !is_filtering {
                                                let mut set = expanded.write();
                                                if set.contains(&dir_path) { set.remove(&dir_path); }
                                                else { set.insert(dir_path.clone()); }
                                            }
                                        },
                                        span { class: "text-stone-600 w-3", if is_open { "▾" } else { "▸" } }
                                        "📁 {name}"
                                    }
                                }
                            } else { rsx! {} }
                        }
                    }
                }
            }
        }
    }
}

// ── Componente principal ─────────────────────────────────────────────────────

#[component]
pub fn Lore() -> Element {
    let state = consume_context::<SharedState>().0;

    // Signal con los datos del vault: (root, entradas).
    // Se rellena via use_future que espera activamente a que vault.open() termine.
    let mut vault_data: Signal<Option<(PathBuf, Vec<VaultEntry>)>> = use_signal(|| None);

    use_future(move || {
        let state = state.clone();
        async move {
            // Polling hasta que el vault esté configurado (cubre la condición de
            // carrera entre vault.open() en load_campain y la navegación a /lore)
            loop {
                if state.vault.is_configured().await {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
            let root = state.vault.root().await.unwrap_or_default();
            let scanned = state.vault.scan_tree().await.unwrap_or_default();
            vault_data.set(Some((root, scanned)));
        }
    });

    // «entries» es un wrapper de lectura sobre vault_data para no cambiar el resto del componente
    let entries = vault_data;

    let mut expanded: Signal<HashSet<PathBuf>> = use_signal(HashSet::new);

    // Expandir todos los directorios cuando los datos lleguen
    use_effect(move || {
        if let Some((_, list)) = entries.read().as_ref() {
            let dirs: HashSet<PathBuf> = list
                .iter()
                .filter(|e| e.path.is_dir())
                .map(|e| e.path.clone())
                .collect();
            if !dirs.is_empty() {
                expanded.set(dirs);
            }
        }
    });

    let mut filter: Signal<String> = use_signal(String::new);
    let mut selected: Signal<Option<PathBuf>> = use_signal(|| None);

    let html_content =
        use_memo(move || selected.read().as_ref().map(md_to_html).unwrap_or_default());

    let mut link_target: Signal<Option<String>> = use_signal(|| None);

    let mut evalr = document::eval(
        r#"
        document.addEventListener('click', (e) => {
            const a = e.target.closest('a[href^="obsidian://"]');
            if (!a) return;
            e.preventDefault();
            const target = decodeURIComponent(a.href.replace('obsidian://', ''));
            dioxus.send(target);
        });
        "#,
    );

    use_future(move || async move {
        loop {
            if let Ok(val) = evalr.recv::<serde_json::Value>().await {
                if let Some(name) = val.as_str() {
                    link_target.set(Some(name.to_string()));
                }
            }
        }
    });

    // Resolver wikilinks al path real en el vault escaneado
    use_effect(move || {
        let name = link_target.read().clone();
        if let Some(ref name) = name {
            if let Some((_, list)) = entries.read().as_ref() {
                let name_lower = name.to_lowercase().replace("%20", " ");
                if let Some(entry) = list
                    .iter()
                    .find(|e| e.path.is_file() && e.slug.to_lowercase() == name_lower)
                {
                    selected.set(Some(entry.path.clone()));
                    link_target.set(None);
                }
            }
        }
    });
    // Nota: entries es ahora Signal<Option<...>> — mismo API que antes (read().as_ref())

    rsx! {
        div {
            class: "flex h-screen w-screen overflow-hidden bg-stone-950 text-stone-200 font-serif",

            // ── Panel izquierdo ───────────────────────────────────────────
            aside {
                class: "w-72 min-w-64 flex flex-col border-r border-stone-800 overflow-hidden",

                div {
                    class: "px-4 py-3 text-xs uppercase tracking-widest text-stone-500 border-b border-stone-800 shrink-0",
                    "Lore del mundo"
                }

                div {
                    class: "px-3 py-2 border-b border-stone-800 shrink-0",
                    div {
                        class: "flex items-center gap-2 bg-stone-900 rounded px-2 py-1",
                        span { class: "text-stone-500 text-xs select-none", "🔍" }
                        input {
                            class: "flex-1 bg-transparent text-sm text-stone-200 placeholder-stone-600 outline-none",
                            r#type: "text",
                            placeholder: "Buscar notas…",
                            value: "{filter.read()}",
                            oninput: move |e| filter.set(e.value()),
                        }
                        if !filter.read().is_empty() {
                            button {
                                class: "text-stone-500 hover:text-stone-300 text-xs leading-none",
                                onclick: move |_| filter.set(String::new()),
                                "✕"
                            }
                        }
                    }
                }

                div {
                    class: "flex-1 overflow-y-auto",
                    match entries.read().as_ref() {
                        None => rsx! {
                            div {
                                class: "p-4 text-stone-500 text-sm animate-pulse",
                                "Cargando vault…"
                            }
                        },
                        Some((root, list)) => rsx! {
                            VaultTree {
                                entries: list.clone(),
                                root: root.clone(),
                                filter: filter.read().clone(),
                                selected,
                                expanded,
                            }
                        },
                    }
                }
            }

            // ── Panel derecho ─────────────────────────────────────────────
            main {
                class: "flex-1 overflow-y-auto px-12 py-8",
                if selected.read().is_none() {
                    div {
                        class: "flex items-center justify-center h-full text-stone-600 text-lg",
                        "Selecciona una nota del vault"
                    }
                } else {
                    article {
                        class: "prose prose-invert prose-amber max-w-3xl mx-auto",
                        dangerous_inner_html: "{html_content.read()}"
                    }
                }
            }
        }
    }
}
