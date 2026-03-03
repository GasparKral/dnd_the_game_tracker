package io.github.gasparkral.dnd.ui.component

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.ui.theme.*

val DndShape = RoundedCornerShape(3.dp)

/**
 * Tarjeta con estética de manual: fondo crypt/abyss, borde iron,
 * ornamentos de esquina dorados (igual que .card del CSS del desktop).
 */
@Composable
fun DndCard(
    modifier: Modifier = Modifier,
    cornerSize: Dp = 14.dp,
    content: @Composable ColumnScope.() -> Unit
) {
    val cornerColor = Gold
    Box(
        modifier = modifier
            .background(
                brush = Brush.linearGradientBrush(
                    colors = listOf(Crypt, Abyss),
                    start = Offset(0f, 0f),
                    end = Offset(Float.MAX_VALUE, Float.MAX_VALUE)
                ),
                shape = DndShape
            )
            .border(width = 1.dp, color = Iron, shape = DndShape)
            .drawBehind {
                val c = cornerSize.toPx()
                val stroke = 1.dp.toPx()
                val col = cornerColor.copy(alpha = 0.55f)
                // Esquina superior izquierda
                drawLine(col, Offset(8.dp.toPx(), 8.dp.toPx()), Offset(8.dp.toPx() + c, 8.dp.toPx()), stroke)
                drawLine(col, Offset(8.dp.toPx(), 8.dp.toPx()), Offset(8.dp.toPx(), 8.dp.toPx() + c), stroke)
                // Esquina inferior derecha
                drawLine(col, Offset(size.width - 8.dp.toPx(), size.height - 8.dp.toPx()), Offset(size.width - 8.dp.toPx() - c, size.height - 8.dp.toPx()), stroke)
                drawLine(col, Offset(size.width - 8.dp.toPx(), size.height - 8.dp.toPx()), Offset(size.width - 8.dp.toPx(), size.height - 8.dp.toPx() - c), stroke)
            }
    ) {
        Column(modifier = Modifier.padding(16.dp), content = content)
    }
}

/**
 * Divisor ornamental: línea con símbolo ✦ central, igual que <hr> del CSS.
 */
@Composable
fun DndDivider(modifier: Modifier = Modifier, symbol: String = "✦") {
    Row(
        modifier = modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically
    ) {
        HorizontalDivider(
            modifier = Modifier.weight(1f),
            color = Iron
        )
        Text(
            text = "  $symbol  ",
            style = MaterialTheme.typography.labelSmall,
            color = Gold
        )
        HorizontalDivider(
            modifier = Modifier.weight(1f),
            color = Iron
        )
    }
}

/**
 * Etiqueta uppercase estilo .form-label del CSS.
 */
@Composable
fun DndLabel(text: String, modifier: Modifier = Modifier) {
    Text(
        text = text.uppercase(),
        style = MaterialTheme.typography.labelMedium,
        color = Ash,
        modifier = modifier
    )
}

// Extensión cómoda para Brush.linearGradient con Offset
private fun Brush.Companion.linearGradientBrush(
    colors: List<Color>,
    start: Offset,
    end: Offset
) = linearGradient(colors = colors, start = start, end = end)
