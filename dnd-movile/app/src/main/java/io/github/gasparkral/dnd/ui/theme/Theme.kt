package io.github.gasparkral.dnd.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable

/**
 * Paleta fija inspirada en el manual antiguo DnD:
 * fondo oscuro como la cripta, textos en pergamino, acentos dorados y rojo sangre.
 * Se desactiva el dynamic color para mantener la identidad visual.
 */
private val DndColorScheme = darkColorScheme(
    // Primarios — dorado / aurum
    primary          = Aurum,
    onPrimary        = Void,
    primaryContainer = Gold,
    onPrimaryContainer = Vellum,

    // Secundarios — rojo sangre
    secondary          = Blood,
    onSecondary        = Vellum,
    secondaryContainer = Crimson,
    onSecondaryContainer = Parchment,

    // Terciarios — arcano
    tertiary          = Ether,
    onTertiary        = Vellum,
    tertiaryContainer = Mystic,
    onTertiaryContainer = Glow,

    // Fondos
    background = Void,
    onBackground = Parchment,
    surface    = Abyss,
    onSurface  = Parchment,
    surfaceVariant    = Crypt,
    onSurfaceVariant  = Bone,

    // Contorno
    outline       = Iron,
    outlineVariant= Dungeon,

    // Error
    error    = Ember,
    onError  = Void,
    errorContainer    = Crimson,
    onErrorContainer  = Parchment,
)

@Composable
fun DndTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = DndColorScheme,
        typography  = DndTypography,
        content     = content
    )
}
