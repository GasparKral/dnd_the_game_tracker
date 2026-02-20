# 🎲 DnD The Game Tracker

Monorepo para una suite de herramientas de gestión de partidas de Dungeons & Dragons. El sistema está compuesto por una app de escritorio para el Dungeon Master y una app móvil para los jugadores, comunicadas en tiempo real mediante WebSockets.

---

## Estructura del proyecto

```
dnd_the_game_tracker/
│
├── Cargo.toml          ← Workspace raíz de Rust
│
├── dnd-desktop/        ← App de escritorio para el DM (Rust + Dioxus)
│   ├── src/
│   │   ├── main.rs
│   │   ├── backend/    ← Servidor Axum embebido + WebSocket
│   │   ├── states/     ← Estado global de la app
│   │   └── ui/
│   │       ├── components/
│   │       ├── layouts/
│   │       └── screens/    ← main_menu, new_campain, load_campain, lore, options
│   ├── assets/         ← Tailwind CSS
│   └── Dioxus.toml
│
├── dnd-movile/         ← App Android para jugadores (Kotlin + Jetpack Compose)
│   └── app/src/main/
│       ├── AndroidManifest.xml
│       └── java/io/github/gasparkral/dnd_movile/
│           ├── MainActivity.kt
│           └── ui/theme/
│
└── shared/             ← Crate Rust con tipos compartidos (DTOs, modelos)
    └── src/
        ├── models/     ← Character, Attributes, Dice, Damage, Inventory, Items...
        └── traits/
```

---

## Stack tecnológico

| Capa | Tecnología |
|---|---|
| App desktop | Rust + [Dioxus](https://dioxuslabs.com/) (modo desktop) |
| Servidor embebido | [Axum](https://github.com/tokio-rs/axum) (corre dentro del proceso desktop) |
| WebSockets | Axum WS + `futures-util` |
| App móvil | Kotlin + Jetpack Compose (Android) |
| Tipos compartidos | Crate `shared` (Rust) con `serde` |
| Estilos | Tailwind CSS |
| Lore / vault | Integración con Obsidian (Markdown + frontmatter via `gray_matter` + `pulldown-cmark`) |

---

## Arquitectura

El DM ejecuta la app de escritorio, que levanta internamente un servidor Axum con WebSockets. Los jugadores se conectan desde sus dispositivos Android a ese servidor a través de la red local (o un túnel Cloudflare para juego remoto). El crate `shared` define los modelos y DTOs que ambos extremos usan, garantizando consistencia de tipos.

```
┌─────────────────────────────────┐
│        dnd-desktop (DM)         │
│  ┌─────────────┐  ┌──────────┐  │
│  │  Dioxus UI  │  │  Axum +  │  │
│  │  (pantallas)│  │  WS API  │  │
│  └─────────────┘  └────┬─────┘  │
└───────────────────────┼─────────┘
                         │ WebSocket / HTTP
              ┌──────────┴──────────┐
              │                     │
      ┌───────┴──────┐     ┌────────┴─────┐
      │ dnd-movile   │     │ dnd-movile   │
      │  (jugador 1) │     │  (jugador 2) │
      └──────────────┘     └──────────────┘
```

---

## Primeros pasos

### Requisitos

- [Rust](https://rustup.rs/) (stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.5/getting_started): `cargo install dioxus-cli`
- [Bun](https://bun.sh/) (para Tailwind)
- Android Studio (para la app móvil)

### App de escritorio (DM)

```bash
cd dnd-desktop

# Instalar dependencias de Tailwind
bun install

# Modo desarrollo
dx serve --platform desktop

# Build de producción
dx build --platform desktop --release
```

### App móvil (jugadores)

Abrir la carpeta `dnd-movile/` con Android Studio y ejecutar en un dispositivo o emulador.

### Túnel para juego remoto (opcional)

Si los jugadores no están en la misma red local:

```bash
cloudflared tunnel --url http://localhost:<puerto>
```

---

## Módulo `shared`

Contiene todos los tipos de dominio usados tanto por el backend como por la UI:

- `models/character.rs` — struct `Character` y campos de personaje
- `models/attributes.rs` — atributos D&D (STR, DEX, CON, INT, WIS, CHA)
- `models/dice.rs` — tipos de dados y tiradas
- `models/damage.rs` — tipos y cálculo de daño
- `models/inventory.rs` — inventario del personaje
- `models/items/` — definición de objetos
- `models/builders/` — builders para construcción de entidades
- `models/defaults/` — valores por defecto de las entidades

---

## Licencia

MIT — Gaspar Gómez Kral
