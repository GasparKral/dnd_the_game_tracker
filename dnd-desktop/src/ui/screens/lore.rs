use crate::states::SharedState;
use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::OnceLock;
use walkdir::WalkDir;

// ── Tipos locales ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
struct VaultEntry {
    path: PathBuf,
    label: String,
    depth: usize,
    is_dir: bool,
    parent: Option<PathBuf>,
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn resolve_obsidian_links(input: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap());
    re.replace_all(input, |caps: &regex::Captures| {
        let target = &caps[1];
        let label = caps.get(2).map_or(target, |m| m.as_str());
        let encoded = target.replace(' ', "%20");
        format!("<a href='obsidian://{encoded}' class='obsidian-link text-amber-400 underline hover:text-amber-200 cursor-pointer'>{label}</a>")
    })
    .to_string()
}

fn scan_vault(root: &PathBuf) -> Vec<VaultEntry> {
    WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            !p.components()
                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
                && (e.file_type().is_dir() || p.extension().map_or(false, |x| x == "md"))
        })
        .filter(|e| {
            !(e.path().is_dir() && e.file_name().to_string_lossy().to_uppercase() == "IMAGENES")
        })
        .map(|e| {
            let depth = e.depth().saturating_sub(1);
            let is_dir = e.file_type().is_dir();
            let label = if is_dir {
                e.file_name().to_string_lossy().to_string()
            } else {
                e.path()
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            };
            let parent = if is_dir {
                None
            } else {
                e.path().parent().map(|p| p.to_path_buf())
            };
            VaultEntry {
                path: e.path().to_path_buf(),
                label,
                depth,
                is_dir,
                parent,
            }
        })
        .collect()
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

fn md_to_html(path: &PathBuf) -> String {
    let Ok(source) = std::fs::read_to_string(path) else {
        return "<p class='text-red-400'>No se pudo leer el archivo.</p>".to_string();
    };
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

fn compute_filtered_expanded(entries: &[VaultEntry], filter: &str) -> HashSet<PathBuf> {
    let filter_lower = filter.to_lowercase();
    let mut visible_dirs: HashSet<PathBuf> = HashSet::new();
    for entry in entries.iter().filter(|e| !e.is_dir) {
        if entry.label.to_lowercase().contains(&filter_lower) {
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
    filter: String,
    selected: Signal<Option<PathBuf>>,
    expanded: Signal<HashSet<PathBuf>>,
) -> Element {
    if entries.is_empty() {
        return rsx! {
            div { class: "p-4 text-stone-500 text-sm", "El vault está vacío o no se ha configurado." }
        };
    }

    let vault_root = entries.first().map(|e| e.path.clone()).unwrap_or_default();
    let is_filtering = !filter.is_empty();

    let effective_expanded: HashSet<PathBuf> = if is_filtering {
        compute_filtered_expanded(&entries, &filter)
    } else {
        expanded.read().clone()
    };

    let has_matches = if is_filtering {
        entries
            .iter()
            .any(|e| !e.is_dir && e.label.to_lowercase().contains(&filter.to_lowercase()))
    } else {
        true
    };

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
                        let eff_exp = effective_expanded.clone();
                        let root = vault_root.clone();

                        if entry.is_dir && entry.depth > 0 {
                            let dir_path = entry.path.clone();
                            let is_open = eff_exp.contains(&dir_path);
                            let visible = if is_filtering {
                                eff_exp.contains(&dir_path)
                                    || eff_exp.iter().any(|p| p.starts_with(&dir_path))
                            } else {
                                all_ancestors_expanded(&entry, &eff_exp, &root)
                            };

                            if visible {
                                rsx! {
                                    button {
                                        key: "{dir_path:?}",
                                        class: "w-full text-left py-1 text-xs uppercase tracking-wider text-stone-500 mt-1 hover:text-stone-300 flex items-center gap-1",
                                        style: "padding-left: {entry.depth * 12}px",
                                        onclick: move |_| {
                                            if !is_filtering {
                                                let mut set = expanded.write();
                                                if set.contains(&dir_path) {
                                                    set.remove(&dir_path);
                                                } else {
                                                    set.insert(dir_path.clone());
                                                }
                                            }
                                        },
                                        span { class: "text-stone-600 w-3", if is_open { "▾" } else { "▸" } }
                                        "📁 {entry.label}"
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        } else if !entry.is_dir {
                            let path = entry.path.clone();
                            let is_active = selected.read().as_ref() == Some(&path);
                            let matches_filter = entry.label.to_lowercase().contains(&filter.to_lowercase());
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
                                        style: "padding-left: {entry.depth * 12 + 4}px",
                                        onclick: move |_| selected.set(Some(path.clone())),
                                        span { class: "text-stone-600 w-3", "·" }
                                        "{entry.label}"
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        } else {
                            rsx! {}
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

    let entries = use_resource(move || {
        let state = state.clone();
        async move {
            let vault = state.get_vault().await;
            if vault.as_os_str().is_empty() || !vault.is_dir() {
                return vec![];
            }
            scan_vault(&vault)
        }
    });

    let mut expanded: Signal<HashSet<PathBuf>> = use_signal(HashSet::new);

    use_effect(move || {
        if let Some(list) = entries.read().as_ref() {
            let dirs: HashSet<PathBuf> = list
                .iter()
                .filter(|e| e.is_dir)
                .map(|e| e.path.clone())
                .collect();
            expanded.set(dirs);
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

    use_effect(move || {
        let name = link_target.read().clone();
        if let Some(ref name) = name {
            if let Some(list) = entries.read().as_ref() {
                let name_lower = name.to_lowercase().replace("%20", " ");
                if let Some(entry) = list
                    .iter()
                    .find(|e| !e.is_dir && e.label.to_lowercase() == name_lower)
                {
                    selected.set(Some(entry.path.clone()));
                    link_target.set(None);
                }
            }
        }
    });

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

                // Barra de búsqueda
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

                // Árbol
                div {
                    class: "flex-1 overflow-y-auto",
                    match entries.read().as_ref() {
                        None => rsx! {
                            div {
                                class: "p-4 text-stone-500 text-sm animate-pulse",
                                "Cargando vault…"
                            }
                        },
                        Some(list) => rsx! {
                            VaultTree {
                                entries: list.clone(),
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
