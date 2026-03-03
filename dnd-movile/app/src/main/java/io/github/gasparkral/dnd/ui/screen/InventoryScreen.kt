package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Template para el inventario del personaje.
 *
 * La lista de items vendrá del servidor / BD local.
 * Estructura pensada para soportar categorías (armas, armadura, consumibles, misc).
 */
@Composable
fun InventoryScreen(
    modifier: Modifier = Modifier,
    onBack: () -> Unit = {}
) {
    // TODO: reemplazar con ViewModel state
    val items = remember {
        listOf(
            InventoryItemUi("Espada larga", "Arma", "1d8 cortante"),
            InventoryItemUi("Escudo de madera", "Armadura", "+2 CA"),
            InventoryItemUi("Poción de curación", "Consumible", "Recupera 2d4+2 PG"),
            InventoryItemUi("Cuerda (15m)", "Misc", ""),
        )
    }

    Column(modifier.padding(16.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.Filled.ArrowBack, contentDescription = "Volver")
            }
            Text("Inventario", style = MaterialTheme.typography.headlineSmall)
        }

        Spacer(Modifier.height(8.dp))

        // TODO: peso total / slots cuando el modelo lo soporte
        Text(
            "${items.size} objetos",
            style = MaterialTheme.typography.bodySmall
        )

        Spacer(Modifier.height(8.dp))

        LazyColumn(verticalArrangement = Arrangement.spacedBy(6.dp)) {
            items(items) { item ->
                InventoryItemRow(item)
            }
        }
    }
}

@Composable
private fun InventoryItemRow(item: InventoryItemUi) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.padding(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column(Modifier.weight(1f)) {
                Text(item.name, style = MaterialTheme.typography.titleSmall)
                if (item.description.isNotBlank()) {
                    Text(item.description, style = MaterialTheme.typography.bodySmall)
                }
            }
            AssistChip(
                onClick = {},
                label = { Text(item.category) }
            )
        }
    }
}

/** UI model mínimo para el inventario. */
data class InventoryItemUi(val name: String, val category: String, val description: String)
