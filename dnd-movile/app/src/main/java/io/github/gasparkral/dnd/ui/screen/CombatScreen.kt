package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Favorite
import androidx.compose.material.icons.filled.Shield
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Template para la pantalla de combate activo.
 *
 * El estado (turno, iniciativa, HP) llegará por WebSocket desde el DM.
 * Los controles de acción se enviarán de vuelta al servidor.
 */
@Composable
fun CombatScreen(
    modifier: Modifier = Modifier,
    draftId: String,
    onBack: () -> Unit = {},
) {
    Column(modifier.padding(16.dp)) {

        // Barra superior
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.Filled.ArrowBack, contentDescription = "Volver")
            }
            Text("Combate", style = MaterialTheme.typography.headlineSmall)
        }

        Spacer(Modifier.height(8.dp))

        // Estado vital del personaje
        CombatStatsBar(
            currentHp = 28,     // TODO: desde ViewModel / WS
            maxHp = 35,
            armorClass = 15
        )

        Spacer(Modifier.height(16.dp))

        // Turno actual
        TurnIndicator(isMyTurn = false) // TODO: desde estado WS

        Spacer(Modifier.height(16.dp))

        // Lista de participantes / orden de iniciativa
        Text("Orden de iniciativa", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(8.dp))

        // TODO: lista real desde servidor
        val participants = listOf(
            CombatantUi("Tú", 5, isPlayer = true, isActive = false),
            CombatantUi("Goblin 1", 3, isPlayer = false, isActive = true),
            CombatantUi("Goblin 2", 1, isPlayer = false, isActive = false),
        )
        LazyColumn(verticalArrangement = Arrangement.spacedBy(6.dp)) {
            items(participants.size) { idx ->
                CombatantRow(participants[idx])
            }
        }

        Spacer(Modifier.weight(1f))

        // Acciones rápidas (solo habilitadas en tu turno)
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Button(onClick = { /* TODO: enviar acción al servidor */ }, modifier = Modifier.weight(1f)) {
                Text("Atacar")
            }
            OutlinedButton(onClick = { /* TODO */ }, modifier = Modifier.weight(1f)) {
                Text("Dash")
            }
            OutlinedButton(onClick = { /* TODO */ }, modifier = Modifier.weight(1f)) {
                Text("Pasar")
            }
        }
    }
}

// ─── Componentes internos ────────────────────────────────────────────────────

@Composable
private fun CombatStatsBar(currentHp: Int, maxHp: Int, armorClass: Int) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceEvenly
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Icon(Icons.Filled.Favorite, contentDescription = null)
            Text("$currentHp / $maxHp", style = MaterialTheme.typography.titleMedium)
            Text("PG", style = MaterialTheme.typography.bodySmall)
        }
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Icon(Icons.Filled.Shield, contentDescription = null)
            Text("$armorClass", style = MaterialTheme.typography.titleMedium)
            Text("CA", style = MaterialTheme.typography.bodySmall)
        }
    }
}

@Composable
private fun TurnIndicator(isMyTurn: Boolean) {
    val color = if (isMyTurn)
        MaterialTheme.colorScheme.primaryContainer
    else
        MaterialTheme.colorScheme.surfaceVariant

    Card(
        colors = CardDefaults.cardColors(containerColor = color),
        modifier = Modifier.fillMaxWidth()
    ) {
        Text(
            text = if (isMyTurn) "🎲 ¡Es tu turno!" else "Esperando turno…",
            modifier = Modifier.padding(16.dp),
            style = MaterialTheme.typography.titleSmall
        )
    }
}

@Composable
private fun CombatantRow(combatant: CombatantUi) {
    val containerColor = when {
        combatant.isActive -> MaterialTheme.colorScheme.secondaryContainer
        combatant.isPlayer -> MaterialTheme.colorScheme.surface
        else -> MaterialTheme.colorScheme.surface
    }
    Card(
        colors = CardDefaults.cardColors(containerColor = containerColor),
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier.padding(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(combatant.name, modifier = Modifier.weight(1f))
            Text("Init: ${combatant.initiative}", style = MaterialTheme.typography.bodySmall)
        }
    }
}

data class CombatantUi(
    val name: String,
    val initiative: Int,
    val isPlayer: Boolean,
    val isActive: Boolean
)
