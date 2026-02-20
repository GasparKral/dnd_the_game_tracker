# dnd-manager — Monorepo

## Estructura del proyecto

```
dnd-manager/
│
├── Cargo.toml                  ← Workspace raíz (Rust)
│
├── crates/                     ← Código Rust
│   ├── backend/                ← Servidor Axum
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── config.rs       ← Configuración (puerto, ruta vault, JWT secret)
│   │   │   ├── db/
│   │   │   │   ├── mod.rs
│   │   │   │   └── migrations/ ← Migraciones SQLite (.sql)
│   │   │   ├── routes/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── characters.rs
│   │   │   │   ├── combat.rs
│   │   │   │   ├── inventory.rs
│   │   │   │   ├── lore.rs     ← Sirve el vault de Obsidian
│   │   │   │   └── ws.rs       ← WebSocket handler (eventos de combate)
│   │   │   ├── models/         ← Structs de base de datos (sqlx::FromRow)
│   │   │   ├── handlers/       ← Lógica de cada endpoint
│   │   │   └── vault/
│   │   │       └── watcher.rs  ← File watcher del vault de Obsidian
│   │   └── migrations/
│   │       ├── 001_init.sql
│   │       ├── 002_characters.sql
│   │       └── 003_combat.sql
│   │
│   └── shared/                 ← Tipos compartidos backend ↔ Tauri
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── models.rs       ← DTOs (Character, CombatState, InventoryItem…)
│           └── events.rs       ← Enum WsEvent para mensajes WebSocket
│
├── apps/
│   ├── desktop/                ← App Tauri (panel master)
│   │   ├── src-tauri/
│   │   │   ├── Cargo.toml      ← Puede importar el crate "shared"
│   │   │   └── src/
│   │   │       └── main.rs
│   │   ├── src/                ← Frontend (React/Svelte/Vue)
│   │   │   └── ...
│   │   ├── package.json
│   │   └── tauri.conf.json
│   │
│   └── mobile/                 ← App Kotlin + Compose (jugadores)
│       ├── app/
│       │   └── src/
│       │       └── main/
│       │           ├── kotlin/
│       │           │   └── com/dndmanager/
│       │           │       ├── MainActivity.kt
│       │           │       ├── network/
│       │           │       │   ├── ApiClient.kt    ← Retrofit / Ktor client
│       │           │       │   └── WsClient.kt     ← WebSocket client
│       │           │       ├── ui/
│       │           │       │   ├── character/
│       │           │       │   ├── inventory/
│       │           │       │   ├── combat/
│       │           │       │   └── lore/
│       │           │       └── viewmodel/
│       │           └── res/
│       ├── build.gradle.kts
│       └── settings.gradle.kts
│
├── .env.example                ← Variables de entorno de ejemplo
├── .gitignore
└── README.md
```

## Arrancar el proyecto

### Backend
```bash
# Instalar sqlx-cli para migraciones
cargo install sqlx-cli --no-default-features --features sqlite

# Crear la base de datos y correr migraciones
cd crates/backend
sqlx database create
sqlx migrate run

# Arrancar el servidor
cargo run -p backend
```

### Desktop (Tauri)
```bash
cd apps/desktop
npm install
npm run tauri dev
```

### Túnel (Cloudflare)
```bash
cloudflared tunnel --url http://localhost:3000
```
