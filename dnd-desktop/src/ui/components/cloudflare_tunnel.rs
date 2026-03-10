use dioxus::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Handle global del proceso — vive fuera del VirtualDom
// ---------------------------------------------------------------------------

// Un único proceso cloudflared por instancia de la app.
// Al usar un static con OnceLock + Mutex evitamos cualquier conflicto con
// el sistema de borrows de Dioxus durante el desmontaje del componente.
fn process_cell() -> &'static Mutex<Option<Child>> {
    static CELL: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
    CELL.get_or_init(|| Mutex::new(None))
}

fn kill_global_process() {
    if let Ok(mut guard) = process_cell().lock() {
        if let Some(child) = guard.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
        *guard = None;
    }
}

fn store_global_process(child: Child) {
    if let Ok(mut guard) = process_cell().lock() {
        *guard = Some(child);
    }
}

// ---------------------------------------------------------------------------
// Estado reactivo del túnel (solo UI, sin proceso)
// ---------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
enum TunnelState {
    Idle,
    Starting,
    Running { url: String },
    Error { message: String },
}

// ---------------------------------------------------------------------------
// Componente público
// ---------------------------------------------------------------------------

/// Botón en la barra de navegación que lanza `cloudflared tunnel --url localhost:3000`,
/// espera la URL pública y la muestra en un popup con opción de copiar al portapapeles.
#[component]
pub fn CloudflareTunnelButton() -> Element {
    let mut tunnel_state = use_signal(|| TunnelState::Idle);
    let mut show_popup = use_signal(|| false);
    let mut copied = use_signal(|| false);

    let label = match &*tunnel_state.read() {
        TunnelState::Idle => "☁ Tunnel",
        TunnelState::Starting => "⏳ Iniciando…",
        TunnelState::Running { .. } => "☁ Tunnel ●",
        TunnelState::Error { .. } => "☁ Tunnel ✕",
    };

    let btn_class = match &*tunnel_state.read() {
        TunnelState::Running { .. } =>
            "text-sm px-3 py-1 rounded border border-green-600 text-green-400 hover:bg-green-900/30 cursor-pointer",
        TunnelState::Error { .. } =>
            "text-sm px-3 py-1 rounded border border-red-700 text-red-400 hover:bg-red-900/30 cursor-pointer",
        TunnelState::Starting =>
            "text-sm px-3 py-1 rounded border border-stone-600 text-stone-400 cursor-wait",
        TunnelState::Idle =>
            "text-sm px-3 py-1 rounded border border-stone-600 text-stone-300 hover:bg-stone-700 cursor-pointer",
    };

    rsx!(
        // ── Botón de la barra ──────────────────────────────────────────────
        button {
            class: "{btn_class}",
            onclick: move |_| {
                // Leemos el estado *antes* de entrar al bloque, sin borrow activo
                let current = (*tunnel_state.read()).clone();
                match current {
                    TunnelState::Idle | TunnelState::Error { .. } => {
                        start_tunnel(tunnel_state, show_popup);
                    }
                    TunnelState::Running { .. } => {
                        let visible = *show_popup.read();
                        show_popup.set(!visible);
                    }
                    TunnelState::Starting => {}
                }
            },
            "{label}"
        }

        // ── Popup ──────────────────────────────────────────────────────────
        if *show_popup.read() {
            if let TunnelState::Running { url } = (*tunnel_state.read()).clone() {
                // Overlay para cerrar al hacer clic fuera
                div {
                    class: "fixed inset-0 z-40",
                    onclick: move |_| show_popup.set(false),
                }
                // Panel
                div {
                    class: "fixed top-14 right-4 z-50 bg-stone-900 border border-stone-700 rounded-xl shadow-2xl p-5 w-96",
                    div {
                        class: "flex items-center justify-between mb-3",
                        span { class: "text-sm font-semibold text-stone-200", "☁ Cloudflare Tunnel activo" }
                        button {
                            class: "text-stone-400 hover:text-stone-100 text-lg leading-none",
                            onclick: move |_| show_popup.set(false),
                            "×"
                        }
                    }
                    p { class: "text-xs text-stone-400 mb-1", "URL pública:" }
                    div {
                        class: "flex items-center gap-2 bg-stone-800 rounded-lg px-3 py-2 mb-4",
                        span {
                            class: "text-sm text-green-400 font-mono flex-1 break-all select-all",
                            "{url}"
                        }
                    }
                    div {
                        class: "flex gap-2",
                        {
                            let url_copy = url.clone();
                            rsx!(
                                // Copiar al portapapeles
                                button {
                                    class: "flex-1 text-sm px-3 py-2 rounded-lg bg-stone-700 hover:bg-stone-600 text-stone-200 transition-colors",
                                    onclick: move |_| {
                                        copy_to_clipboard(&url_copy);
                                        copied.set(true);
                                        spawn(async move {
                                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                            copied.set(false);
                                        });
                                    },
                                    if *copied.read() { "✓ Copiado" } else { "📋 Copiar URL" }
                                }
                                // Detener túnel
                                button {
                                    class: "flex-1 text-sm px-3 py-2 rounded-lg bg-red-900/50 hover:bg-red-800/60 text-red-300 transition-colors",
                                    onclick: move |_| {
                                        kill_global_process();
                                        tunnel_state.set(TunnelState::Idle);
                                        show_popup.set(false);
                                    },
                                    "⏹ Detener"
                                }
                            )
                        }
                    }
                }
            }
        }

        // ── Toast de error ─────────────────────────────────────────────────
        if let TunnelState::Error { message } = (*tunnel_state.read()).clone() {
            div {
                class: "fixed top-14 right-4 z-50 bg-red-950 border border-red-700 rounded-xl shadow-xl p-4 w-96",
                div {
                    class: "flex items-start justify-between gap-3",
                    div {
                        p { class: "text-sm font-semibold text-red-300 mb-1", "Error al iniciar el túnel" }
                        p { class: "text-xs text-red-400 font-mono break-all", "{message}" }
                    }
                    button {
                        class: "text-red-400 hover:text-red-100 text-lg leading-none flex-shrink-0",
                        onclick: move |_| tunnel_state.set(TunnelState::Idle),
                        "×"
                    }
                }
            }
        }
    )
}

// ---------------------------------------------------------------------------
// Lógica del proceso
// ---------------------------------------------------------------------------

fn start_tunnel(mut tunnel_state: Signal<TunnelState>, mut show_popup: Signal<bool>) {
    tunnel_state.set(TunnelState::Starting);

    spawn(async move {
        let child = Command::new("cloudflared")
            .args(["--url", "localhost:3000"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(c) => c,
            Err(e) => {
                tunnel_state.set(TunnelState::Error {
                    message: format!("No se pudo lanzar cloudflared: {e}"),
                });
                return;
            }
        };

        // cloudflared escribe la URL en stderr
        let stderr = match child.stderr.take() {
            Some(s) => s,
            None => {
                tunnel_state.set(TunnelState::Error {
                    message: "No se pudo capturar stderr de cloudflared".into(),
                });
                return;
            }
        };

        // Guardamos el Child en el static global
        store_global_process(child);

        // Leemos stderr en un hilo bloqueante hasta encontrar la URL
        let url = tokio::task::spawn_blocking(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                info!(line);
                if let Some(url) = extract_trycloudflare_url(&line) {
                    return Some(url);
                }
            }
            None
        })
        .await
        .ok()
        .flatten();

        match url {
            Some(url) => {
                show_popup.set(true);
                tunnel_state.set(TunnelState::Running { url });
            }
            None => {
                tunnel_state.set(TunnelState::Error {
                    message: "cloudflared terminó sin emitir una URL. ¿Está instalado?".into(),
                });
            }
        }
    });
}

/// Extrae la primera URL `https://*.trycloudflare.com` de una línea de texto.
fn extract_trycloudflare_url(line: &str) -> Option<String> {
    let marker = "https://";
    let mut start = 0;
    while let Some(pos) = line[start..].find(marker) {
        let abs = start + pos;
        let candidate = &line[abs..];
        let end = candidate
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'')
            .unwrap_or(candidate.len());
        let url = &candidate[..end];
        if url.contains(".trycloudflare.com") {
            return Some(url.to_string());
        }
        start = abs + marker.len();
    }
    None
}

/// Copia texto al portapapeles usando las herramientas del sistema disponibles.
fn copy_to_clipboard(text: &str) {
    #[cfg(target_os = "linux")]
    {
        use std::io::Write;
        let tools: &[(&str, &[&str])] = &[
            ("xclip", &["-selection", "clipboard"]),
            ("xsel", &["--clipboard", "--input"]),
            ("wl-copy", &[]),
        ];
        for (tool, args) in tools {
            if let Ok(mut c) = Command::new(tool).args(*args).stdin(Stdio::piped()).spawn() {
                let _ = c.stdin.as_mut().map(|s| s.write_all(text.as_bytes()));
                let _ = c.wait();
                return;
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        use std::io::Write;
        if let Ok(mut c) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() {
            let _ = c.stdin.as_mut().map(|s| s.write_all(text.as_bytes()));
            let _ = c.wait();
        }
    }
    #[cfg(target_os = "windows")]
    {
        use std::io::Write;
        if let Ok(mut c) = Command::new("clip").stdin(Stdio::piped()).spawn() {
            let _ = c.stdin.as_mut().map(|s| s.write_all(text.as_bytes()));
            let _ = c.wait();
        }
    }
}
