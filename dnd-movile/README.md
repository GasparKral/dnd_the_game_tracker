# dnd-mobile — App Android para Jugadores

Aplicación Android para los jugadores de una partida de **D&D 5.5e (2024)**. Se conecta al servidor embebido en la aplicación desktop del Dungeon Master y actúa como hoja de personaje digital, visor de lore y panel de combate en tiempo real.

---

## Contexto del sistema

El proyecto se compone de dos piezas:

| Pieza | Tecnología | Rol |
|---|---|---|
| **dnd-desktop** | Rust · Dioxus 0.7 | Panel de control del DM. Corre el servidor y gestiona la campaña. |
| **dnd-mobile** | Kotlin · Jetpack Compose | App de los jugadores. Consume la API del servidor del DM. |

La comunicación es REST + WebSocket. El DM expone el servidor local y opcionalmente lo tunelea a través de Cloudflare para que los jugadores se conecten sin estar en la misma red.

```
Móvil del jugador  ──HTTP/WS──▶  dnd-desktop (servidor Axum)
                                        │
                                  campaign.json (disco)
                                  vault de Obsidian
```

---

## Flujo de uso

1. El jugador abre la app por primera vez e introduce su nombre.
2. Escanea el QR o introduce la URL del servidor del DM.
3. Si ya tiene un personaje creado en esa campaña, lo selecciona directamente.
4. Si no, completa el **wizard de creación** paso a paso — los catálogos (razas, clases, trasfondos) los provee el servidor en tiempo real.
5. Una vez en el dashboard, el jugador accede a su hoja de personaje, el inventario, el lore de la partida y el estado de combate.

---

## Funcionalidades implementadas

### Onboarding
- Registro de nombre de jugador (persiste en `SharedPreferences`).
- Pantalla de conexión al servidor: introduce URL o escanea QR, verifica conectividad antes de continuar.

### Wizard de creación de personaje
El wizard es **completamente dinámico**: el servidor envía los `ChoiceSchema` y la app los renderiza sin conocer el dominio D&D. Añadir razas, clases o trasfondos en el servidor actualiza la app sin tocarla.

Pasos del wizard:
1. **Nombre** del personaje.
2. **Raza** — con sub-elección de linaje donde aplica (ej. alto elfo, drow…).
3. **Clase** — con sub-elección de arquetipo/senda/dominio… según la clase.
4. **Atributos** — sistema de *point-buy* con tabla de costes oficial de 5.5e (presupuesto de 27 puntos).
5. **Trasfondo**.
6. **Dones** *(pendiente de implementación completa)*.
7. **Revisión** — resumen antes de finalizar.

Al finalizar el wizard el servidor crea el personaje y lo guarda en la campaña activa. El personaje persiste entre sesiones aunque el servidor se reinicie.

### Selección de personaje
Lista los personajes del jugador en la campaña activa, con nombre, raza · clase, PG y nivel. Al seleccionar uno navega al dashboard.

### Dashboard
Vista principal del personaje con:
- Cabecera: nombre, raza · clase.
- Chips de estado: PG actual/máx, nivel, XP.
- Accesos rápidos a Inventario, Lore y Combate.

### Lore
Visor del vault de Obsidian del DM:
- Árbol navegable de carpetas y entradas.
- Búsqueda por título.
- Carga perezosa del contenido al expandir cada entrada.
- Renderizado de Markdown con soporte de wikilinks `[[...]]` y etiquetas HTML.

### Inventario y Combate
*(Pendientes — estructura y navegación creadas, contenido por implementar.)*

---

## Arquitectura

```
app/
├── di/                     Koin — inyección de dependencias
├── infra/
│   ├── HttpClient.kt       Ktor client + configuración JSON
│   ├── HttpManager.kt      Singleton con baseUrl y helpers get/post tipados
│   └── repository/
│       └── DraftRepository.kt   Todos los endpoints de la API
├── model/                  DTOs Kotlin (espejo de los tipos Rust compartidos)
│   ├── Catalog.kt          CatalogEntry, ChoiceSchema (sealed), SelectOption
│   ├── CharacterDraft.kt   Wizard: pasos, request/response, AttributesDto
│   └── Campaign.kt         SavedCharacter, CampaignSummary, CharactersResponse
└── ui/
    ├── screen/             Una pantalla por destino de navegación
    ├── viewmodel/          CharacterCreationViewModel — estado del wizard
    ├── component/          Componentes reutilizables
    └── theme/              Tema oscuro con paleta temática D&D
```

**Gestión de estado**: `ViewModel` + `StateFlow` para el wizard; `remember`/`mutableStateOf` para el resto de pantallas.  
**Red**: Ktor Client con `ContentNegotiation` + `kotlinx.serialization`. Sin capa Retrofit ni Room — toda la persistencia vive en el servidor.  
**Inyección**: Koin. `DraftRepository` es singleton; `CharacterCreationViewModel` se instancia con el nombre del jugador como parámetro.

---

## Pendiente de desarrollar

| Área | Descripción |
|---|---|
| **Inventario** | Listar, añadir y usar objetos del personaje. |
| **Combate** | Mostrar orden de iniciativa, turno activo, HP de los participantes en tiempo real vía WebSocket. |
| **Dones** | Paso del wizard con selección de feats del catálogo. |
| **MultiSelect en choices** | Renderizado de `ChoiceSchema.MultiSelect` en el wizard (actualmente stub). |
| **Offline / caché** | DataStore o caché en memoria para consultar datos básicos sin conexión. |
| **Notificaciones** | Aviso al jugador cuando empieza su turno en combate. |
| **QR de conexión** | Escanear el QR que muestra el DM en lugar de escribir la URL. |

---

## Stack técnico

| Librería | Uso |
|---|---|
| Jetpack Compose + Material 3 | UI declarativa |
| Navigation Compose | Navegación type-safe con `@Serializable` |
| Ktor Client (CIO + Android) | HTTP y WebSocket |
| kotlinx.serialization | Deserialización JSON |
| Koin | Inyección de dependencias |
| DataStore Preferences | Persistencia local ligera (nombre de jugador, URL) |
| Coil | Carga de imágenes (lore) |
| intellij-markdown | Render de Markdown del vault |
| minSdk 26 (Android 8) | Target mínimo |
