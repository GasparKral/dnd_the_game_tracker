package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.infra.dbstruct.Character

/**
 * Hub central del jugador. Desde aquí se navega a todas las secciones.
 *
 * [isInCombat] determina si se muestra la tarjeta de combate activo.
 * Las lambdas de navegación se conectarán en MainActivity.
 */
@Composable
fun DashboardScreen(
    modifier: Modifier = Modifier,
    character: Character,
    isInCombat: Boolean = false,          // TODO: vendrá del estado del servidor (WebSocket)
    onNavigateToCharacterInfo: () -> Unit,
    onNavigateToInventory: () -> Unit,
    onNavigateToLore: () -> Unit,
    onNavigateToCombat: () -> Unit,
) {
    Column(modifier.padding(16.dp)) {

        // Cabecera
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 20.dp)
        ) {
            Icon(
                imageVector = Icons.Filled.AccountCircle,
                contentDescription = null,
                modifier = Modifier.size(48.dp)
            )
            Spacer(Modifier.width(12.dp))
            Column {
                Text(character.characterName, style = MaterialTheme.typography.titleLarge)
                // TODO: raza / clase cuando existan en el modelo
                Text("Aventurero", style = MaterialTheme.typography.bodySmall)
            }
        }

        // Alerta de combate activo
        if (isInCombat) {
            CombatBanner(onClick = onNavigateToCombat)
            Spacer(Modifier.height(12.dp))
        }

        // Grid de accesos rápidos
        LazyVerticalGrid(
            columns = GridCells.Fixed(2),
            verticalArrangement = Arrangement.spacedBy(12.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            item {
                DashboardCard(
                    title = "Personaje",
                    subtitle = "Estadísticas y habilidades",
                    icon = Icons.Filled.Person,
                    onClick = onNavigateToCharacterInfo
                )
            }
            item {
                DashboardCard(
                    title = "Inventario",
                    subtitle = "Objetos y equipamiento",
                    icon = Icons.Filled.ShoppingBag,
                    onClick = onNavigateToInventory
                )
            }
            item {
                DashboardCard(
                    title = "Lore",
                    subtitle = "Mundo y conocimiento",
                    icon = Icons.Filled.MenuBook,
                    onClick = onNavigateToLore
                )
            }
        }
    }
}

// ─── Componentes internos ────────────────────────────────────────────────────

@Composable
private fun CombatBanner(onClick: () -> Unit) {
    Card(
        onClick = onClick,
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.errorContainer
        ),
        modifier = Modifier.fillMaxWidth()
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(Icons.Filled.Warning, contentDescription = null)
            Spacer(Modifier.width(12.dp))
            Column {
                Text("¡Combate activo!", style = MaterialTheme.typography.titleSmall)
                Text("Pulsa para unirte", style = MaterialTheme.typography.bodySmall)
            }
        }
    }
}

@Composable
private fun DashboardCard(
    title: String,
    subtitle: String,
    icon: ImageVector,
    onClick: () -> Unit
) {
    Card(
        onClick = onClick,
        modifier = Modifier
            .fillMaxWidth()
            .aspectRatio(1f)
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(16.dp),
            verticalArrangement = Arrangement.Center,
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Icon(icon, contentDescription = null, modifier = Modifier.size(36.dp))
            Spacer(Modifier.height(8.dp))
            Text(title, style = MaterialTheme.typography.titleMedium)
            Text(subtitle, style = MaterialTheme.typography.bodySmall)
        }
    }
}
