package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Book
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Template para las entradas de lore expuestas por el servidor del DM.
 *
 * Los datos vendrán de un ViewModel que consulte el endpoint REST / WebSocket.
 * Por ahora muestra datos de placeholder.
 */
@Composable
fun LoreScreen(
    modifier: Modifier = Modifier,
    onBack: () -> Unit = {}
) {
    // TODO: reemplazar por ViewModel state
    val entries = remember {
        listOf(
            LoreEntryUi("La ciudad de Eldenmoor", "Una ciudad portuaria al norte del reino..."),
            LoreEntryUi("Los Guardianes del Velo", "Facción secreta que custodia los portales..."),
            LoreEntryUi("La Maldición de Kael'thas", "Se dice que el archimago selló su alma en..."),
        )
    }

    Column(modifier.padding(16.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.Filled.Book, contentDescription = "Volver")
            }
            Text("Lore del mundo", style = MaterialTheme.typography.headlineSmall)
        }

        Spacer(Modifier.height(8.dp))

        if (entries.isEmpty()) {
            Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                Text("El maestro aún no ha revelado nada…")
            }
        } else {
            LazyColumn(verticalArrangement = Arrangement.spacedBy(8.dp)) {
                items(entries) { entry ->
                    LoreEntryCard(entry)
                }
            }
        }
    }
}

@Composable
private fun LoreEntryCard(entry: LoreEntryUi) {
    var expanded by remember { mutableStateOf(false) }
    Card(
        onClick = { expanded = !expanded },
        modifier = Modifier.fillMaxWidth()
    ) {
        Column(Modifier.padding(16.dp)) {
            Text(entry.title, style = MaterialTheme.typography.titleMedium)
            if (expanded) {
                Spacer(Modifier.height(4.dp))
                Text(entry.body, style = MaterialTheme.typography.bodyMedium)
            }
        }
    }
}

/** UI model mínimo — sin persistencia ni serialización propia aún. */
data class LoreEntryUi(val title: String, val body: String)
