package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.model.SavedCharacter
import io.github.gasparkral.dnd.ui.theme.*
import org.koin.compose.koinInject

@Composable
fun DashboardScreen(
    modifier: Modifier = Modifier,
    draftId: String,
    onNavigateToInventory: () -> Unit = {},
    onNavigateToLore: () -> Unit = {},
    onNavigateToCombat: () -> Unit = {},
) {
    val repo: DraftRepository = koinInject()
    var character by remember { mutableStateOf<SavedCharacter?>(null) }
    var isLoading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(draftId) {
        // Carga desde personajes guardados — funciona tras reinicio del servidor
        repo.getCharacter(draftId).fold(
            onOk = { character = it; isLoading = false },
            onErr = { error = "No se pudo cargar el personaje"; isLoading = false }
        )
    }

    when {
        isLoading -> Box(modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
            CircularProgressIndicator(color = Gold)
        }
        error != null -> Box(modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
            Text(error!!, color = Ember, style = MaterialTheme.typography.bodyMedium)
        }
        else -> DashboardContent(
            modifier = modifier,
            character = character!!,
            onNavigateToInventory = onNavigateToInventory,
            onNavigateToLore = onNavigateToLore,
            onNavigateToCombat = onNavigateToCombat,
        )
    }
}

@Composable
private fun DashboardContent(
    modifier: Modifier,
    character: SavedCharacter,
    onNavigateToInventory: () -> Unit,
    onNavigateToLore: () -> Unit,
    onNavigateToCombat: () -> Unit,
) {
    Column(modifier.padding(16.dp)) {

        // ── Cabecera ──────────────────────────────────────────────────────
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 8.dp),
        ) {
            Icon(
                imageVector = Icons.Filled.AccountCircle,
                contentDescription = null,
                modifier = Modifier.size(48.dp),
                tint = Aurum,
            )
            Spacer(Modifier.width(12.dp))
            Column {
                Text(character.name, style = MaterialTheme.typography.titleLarge)
                Text(
                    "${character.raceId} · ${character.classId}",
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash,
                )
            }
        }

        // ── Barra de estado ───────────────────────────────────────────────
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 16.dp),
            horizontalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            StatChip(label = "PG", value = "${character.currentHp}/${character.maxHp}", modifier = Modifier.weight(1f))
            StatChip(label = "Niv", value = "${character.level}", modifier = Modifier.weight(1f))
            StatChip(label = "XP", value = "${character.xp}", modifier = Modifier.weight(1f))
        }

        // ── Grid de accesos ───────────────────────────────────────────────
        LazyVerticalGrid(
            columns = GridCells.Fixed(2),
            verticalArrangement = Arrangement.spacedBy(12.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            item {
                DashboardCard(
                    title = "Inventario",
                    subtitle = "Objetos y equipo",
                    icon = Icons.Filled.ShoppingBag,
                    onClick = onNavigateToInventory,
                )
            }
            item {
                DashboardCard(
                    title = "Lore",
                    subtitle = "Mundo y conocimiento",
                    icon = Icons.Filled.MenuBook,
                    onClick = onNavigateToLore,
                )
            }
            item {
                DashboardCard(
                    title = "Combate",
                    subtitle = "Turno e iniciativa",
                    icon = Icons.Filled.GpsFixed,
                    onClick = onNavigateToCombat,
                )
            }
        }
    }
}

@Composable
private fun StatChip(label: String, value: String, modifier: Modifier = Modifier) {
    Card(
        colors = CardDefaults.cardColors(containerColor = Crypt),
        modifier = modifier,
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 8.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(label, style = MaterialTheme.typography.labelSmall, color = Ash)
            Text(value, style = MaterialTheme.typography.titleSmall, color = Aurum)
        }
    }
}

@Composable
private fun DashboardCard(
    title: String,
    subtitle: String,
    icon: ImageVector,
    onClick: () -> Unit,
) {
    Card(
        onClick = onClick,
        modifier = Modifier
            .fillMaxWidth()
            .aspectRatio(1f),
        colors = CardDefaults.cardColors(containerColor = Crypt),
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp),
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Icon(icon, contentDescription = null, modifier = Modifier.size(36.dp), tint = Gold)
            Spacer(Modifier.height(8.dp))
            Text(title, style = MaterialTheme.typography.titleMedium)
            Text(subtitle, style = MaterialTheme.typography.bodySmall, color = Ash)
        }
    }
}
