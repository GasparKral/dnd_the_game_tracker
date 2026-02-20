# 📱 dnd-movile

App Android para los **jugadores** de una partida de D&D. Se conecta en tiempo real a la app de escritorio del Dungeon Master ([dnd-desktop](../dnd-desktop)) mediante HTTP y WebSockets.

> ⚠️ Proyecto en desarrollo inicial. La estructura y dependencias están definidas, la UI está por implementar.

---

## Stack

| | Tecnología |
|---|---|
| Lenguaje | Kotlin |
| UI | Jetpack Compose + Material 3 |
| Navegación | Navigation Compose |
| Red | [Ktor Client](https://ktor.io/docs/client-create-multiplatform-application.html) (HTTP + WebSockets) |
| Serialización | `kotlinx.serialization` |
| ViewModel | AndroidX Lifecycle ViewModel |
| DI | [Koin](https://insert-koin.io/) |
| Persistencia local | DataStore Preferences (token JWT, URL del servidor) |
| Imágenes | Coil |
| Min SDK | 26 (Android 8.0) |
| Target SDK | 35 |

---

## Conexión con el DM

La app se conecta al servidor Axum embebido en `dnd-desktop`. La URL base se configura por build variant:

- **Debug** → `http://10.0.2.2:3000` (localhost desde emulador, o IP local para dispositivo físico)
- **Release** → URL del túnel Cloudflare (para juego remoto)

```
DM (dnd-desktop)
  └── Axum server :3000
        ├── HTTP REST  ←── consultas de personaje, inventario, lore
        └── WebSocket  ←── eventos de combate en tiempo real
```

---

## Estructura del proyecto

```
dnd-movile/
└── app/src/main/
    ├── AndroidManifest.xml
    └── java/io/github/gasparkral/dnd_movile/
        ├── MainActivity.kt
        └── ui/
            └── theme/          ← Color, Typography, Theme (Material 3)
```

> La estructura de pantallas, ViewModels y networking está planificada pero pendiente de implementar.

---

## Funcionalidades planificadas

- Visualización y edición de la ficha del personaje
- Inventario interactivo
- Lore del mundo (notas Markdown desde el vault de Obsidian del DM)
- Eventos de combate en tiempo real (iniciativa, daño, estados)
- Tiradas de dados

---

## Desarrollo

### Requisitos

- Android Studio Hedgehog o superior
- JDK 21
- Dispositivo o emulador con Android 8.0+

### Configurar la URL del servidor (dispositivo físico)

En modo debug, reemplaza la IP en `app/build.gradle.kts`:

```kotlin
buildConfigField("String", "BASE_URL", "\"http://<IP-del-DM>:3000\"")
```

### Build

Abrir la carpeta `dnd-movile/` en Android Studio y ejecutar en un dispositivo o emulador, o bien desde terminal:

```bash
./gradlew assembleDebug
```

---

## Licencia

MIT — Gaspar Gómez Kral
