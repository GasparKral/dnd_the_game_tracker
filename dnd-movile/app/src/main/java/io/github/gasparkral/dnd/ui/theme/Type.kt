package io.github.gasparkral.dnd.ui.theme

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp
import com.dndmanager.R


/**
 * Tipografía DnD:
 *  - Cinzel        → títulos y cabeceras  (equivale al font-heading/display del desktop)
 *  - CrimsonPro    → cuerpo de texto      (equivale al font-body del desktop)
 *
 * Los archivos .ttf deben estar en res/font/:
 *   cinzel_regular.ttf, cinzel_bold.ttf
 *   crimsonpro_regular.ttf, crimsonpro_italic.ttf, crimsonpro_bold.ttf
 *
 * Si los ficheros aún no están en el proyecto se usará Serif como fallback
 * hasta que se añadan (la app compilará igualmente).
 */

private val CinzelFamily = try {
    FontFamily(
        Font(R.font.cinzel_regular, FontWeight.Normal),
        Font(R.font.cinzel_bold, FontWeight.Bold),
    )
} catch (e: Exception) {
    FontFamily.Serif
}

private val CrimsonProFamily = try {
    FontFamily(
        Font(R.font.crimsonpro_regular, FontWeight.Normal),
        Font(R.font.crimsonpro_italic, FontWeight.Normal, FontStyle.Italic),
        Font(R.font.crimsonpro_bold, FontWeight.Bold),
    )
} catch (e: Exception) {
    FontFamily.Serif
}

val DndTypography = Typography(
    // ── Títulos de pantalla ────────────────────────────────────────────────
    displayLarge = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Bold,
        fontSize = 36.sp,
        lineHeight = 44.sp,
        letterSpacing = 0.08.sp,
        color = Aureate,
    ),
    displayMedium = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Bold,
        fontSize = 28.sp,
        lineHeight = 36.sp,
        letterSpacing = 0.06.sp,
        color = Aureate,
    ),
    displaySmall = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Bold,
        fontSize = 22.sp,
        lineHeight = 30.sp,
        letterSpacing = 0.05.sp,
        color = Aureate,
    ),

    // ── Cabeceras de sección ───────────────────────────────────────────────
    headlineLarge = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Bold,
        fontSize = 24.sp,
        lineHeight = 32.sp,
        letterSpacing = 0.05.sp,
        color = Bone,
    ),
    headlineMedium = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 20.sp,
        lineHeight = 28.sp,
        letterSpacing = 0.04.sp,
        color = Bone,
    ),
    headlineSmall = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 18.sp,
        lineHeight = 26.sp,
        letterSpacing = 0.03.sp,
        color = Bone,
    ),

    // ── Títulos de tarjeta / elemento ─────────────────────────────────────
    titleLarge = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Bold,
        fontSize = 18.sp,
        lineHeight = 24.sp,
        letterSpacing = 0.05.sp,
        color = Parchment,
    ),
    titleMedium = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 15.sp,
        lineHeight = 22.sp,
        letterSpacing = 0.04.sp,
        color = Parchment,
    ),
    titleSmall = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 13.sp,
        lineHeight = 20.sp,
        letterSpacing = 0.04.sp,
        color = Bone,
    ),

    // ── Cuerpo de texto ───────────────────────────────────────────────────
    bodyLarge = TextStyle(
        fontFamily = CrimsonProFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 17.sp,
        lineHeight = 26.sp,
        letterSpacing = 0.2.sp,
        color = Parchment,
    ),
    bodyMedium = TextStyle(
        fontFamily = CrimsonProFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 15.sp,
        lineHeight = 23.sp,
        letterSpacing = 0.2.sp,
        color = Parchment,
    ),
    bodySmall = TextStyle(
        fontFamily = CrimsonProFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 13.sp,
        lineHeight = 20.sp,
        letterSpacing = 0.2.sp,
        color = Ash,
    ),

    // ── Etiquetas / chips ─────────────────────────────────────────────────
    labelLarge = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Bold,
        fontSize = 13.sp,
        lineHeight = 18.sp,
        letterSpacing = 0.1.sp,
        color = Aureate,
    ),
    labelMedium = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 11.sp,
        lineHeight = 16.sp,
        letterSpacing = 0.1.sp,
        color = Ash,
    ),
    labelSmall = TextStyle(
        fontFamily = CinzelFamily,
        fontWeight = FontWeight.Normal,
        fontSize = 10.sp,
        lineHeight = 14.sp,
        letterSpacing = 0.12.sp,
        color = Ash,
    ),
)

// Alias legacy para que Theme.kt compile sin cambios
val Typography = DndTypography
