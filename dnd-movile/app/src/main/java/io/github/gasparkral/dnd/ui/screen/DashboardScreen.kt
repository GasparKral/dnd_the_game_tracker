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
import io.github.gasparkral.dnd.model.*
import io.github.gasparkral.dnd.ui.theme.*
import org.koin.compose.koinInject

@Composable
fun DashboardScreen(
    modifier: Modifier = Modifier,
    draftId: String,
    onNavigateToInventory: () -> Unit = {},
    onNavigateToLore: () -> Unit = {},
    onNavigateToCombat: () -> Unit = {},
    onNavigateToSpells: () -> Unit = {},
    onNavigateToDiceRoller: () -> Unit = {},
) {
    val repo: DraftRepository = koinInject()
    var character by remember { mutableStateOf<SavedCharacter?>(null) }
    var isLoading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(draftId) {
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

        else -> {
            val c = character!!
            val effectiveStats = remember(c.inventory) {
                calculateEffectiveStats(c.attributes, c.inventory)
            }
            DashboardContent(
                modifier = modifier,
                character = c,
                effectiveStats = effectiveStats,
                onNavigateToInventory = onNavigateToInventory,
                onNavigateToLore = onNavigateToLore,
                onNavigateToCombat = onNavigateToCombat,
                onNavigateToSpells = onNavigateToSpells,
                onNavigateToDiceRoller = onNavigateToDiceRoller,
            )
        }
    }
}

@Composable
private fun DashboardContent(
    modifier: Modifier,
    character: SavedCharacter,
    effectiveStats: EffectiveStats,
    onNavigateToInventory: () -> Unit,
    onNavigateToLore: () -> Unit,
    onNavigateToCombat: () -> Unit,
    onNavigateToSpells: () -> Unit,
    onNavigateToDiceRoller: () -> Unit = {},
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
        val maxHpEffective = character.maxHp + effectiveStats.maxHpBonus
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 12.dp),
            horizontalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            StatChip(
                label = "PG",
                value = "${character.currentHp}/$maxHpEffective",
                modifier = Modifier.weight(1f),
                bonus = effectiveStats.maxHpBonus.takeIf { it != 0 },
            )
            StatChip(label = "Niv", value = "${character.level}", modifier = Modifier.weight(1f))
            StatChip(label = "XP", value = "${character.xp}", modifier = Modifier.weight(1f))
        }

        // ── Atributos con bonuses de equipo ──────────────────────────────
        AttributesGrid(
            effectiveStats = effectiveStats,
            modifier = Modifier.padding(bottom = 12.dp),
        )

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
                    title = "Hechizos",
                    subtitle = "Grimorio y espacios",
                    icon = Icons.Filled.AutoAwesome,
                    onClick = onNavigateToSpells,
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
                    title = "Dados",
                    subtitle = "Tirar y modificadores",
                    icon = Icons.Filled.Casino,
                    onClick = onNavigateToDiceRoller,
                )
            }
        }
    }
}

@Composable
private fun StatChip(
    label: String,
    value: String,
    modifier: Modifier = Modifier,
    bonus: Int? = null,
) {
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
            if (bonus != null) {
                val bonusColor = if (bonus > 0) androidx.compose.ui.graphics.Color(0xFF86EFAC)
                else androidx.compose.ui.graphics.Color(0xFFFCA5A5)
                Text(
                    formatModifier(bonus),
                    style = MaterialTheme.typography.labelSmall,
                    color = bonusColor,
                    fontSize = androidx.compose.ui.unit.TextUnit(9f, androidx.compose.ui.unit.TextUnitType.Sp),
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Grid de atributos con bonuses de equipo
// ---------------------------------------------------------------------------

private val attrRows = listOf(
    Triple("FUE", BonusStat.Strength, { s: EffectiveStats -> s.effectiveStrength() }),
    Triple("DES", BonusStat.Dexterity, { s: EffectiveStats -> s.effectiveDexterity() }),
    Triple("CON", BonusStat.Constitution, { s: EffectiveStats -> s.effectiveConstitution() }),
    Triple("INT", BonusStat.Intelligence, { s: EffectiveStats -> s.effectiveIntelligence() }),
    Triple("SAB", BonusStat.Wisdom, { s: EffectiveStats -> s.effectiveWisdom() }),
    Triple("CAR", BonusStat.Charisma, { s: EffectiveStats -> s.effectiveCharisma() }),
)

@Composable
private fun AttributesGrid(
    effectiveStats: EffectiveStats,
    modifier: Modifier = Modifier,
) {
    val hasAnyBonus = effectiveStats.hasAnyBonus()

    Column(modifier) {
        if (hasAnyBonus) {
            Text(
                "✨ Atributos (con bonificadores de equipo)",
                style = MaterialTheme.typography.labelSmall,
                color = androidx.compose.ui.graphics.Color(0xFF86EFAC),
                modifier = Modifier.padding(bottom = 6.dp),
            )
        } else {
            Text(
                "Atributos",
                style = MaterialTheme.typography.labelSmall,
                color = Ash,
                modifier = Modifier.padding(bottom = 6.dp),
            )
        }

        // 2 columnas x 3 filas
        for (row in attrRows.chunked(2)) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(bottom = 6.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                for ((label, stat, effective) in row) {
                    val score = effective(effectiveStats)
                    val mod = attributeModifier(score)
                    val bonus = effectiveStats.attributeBonuses[stat]

                    Card(
                        colors = CardDefaults.cardColors(
                            containerColor = if (bonus != null && bonus != 0)
                                androidx.compose.ui.graphics.Color(0xFF0D2010)
                            else
                                Crypt
                        ),
                        modifier = Modifier.weight(1f),
                    ) {
                        Column(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(vertical = 6.dp),
                            horizontalAlignment = Alignment.CenterHorizontally,
                        ) {
                            Text(
                                label,
                                style = MaterialTheme.typography.labelSmall,
                                color = Ash,
                            )
                            Text(
                                "$score",
                                style = MaterialTheme.typography.titleMedium,
                                color = Aurum,
                            )
                            Text(
                                formatModifier(mod),
                                style = MaterialTheme.typography.labelMedium,
                                color = androidx.compose.ui.graphics.Color(0xFFD6D3D1),
                            )
                            // Bonus del equipo
                            if (bonus != null && bonus != 0) {
                                val bonusColor = if (bonus > 0)
                                    androidx.compose.ui.graphics.Color(0xFF86EFAC)
                                else
                                    androidx.compose.ui.graphics.Color(0xFFFCA5A5)
                                Text(
                                    "equipo ${formatModifier(bonus)}",
                                    style = MaterialTheme.typography.labelSmall,
                                    color = bonusColor,
                                    fontSize = androidx.compose.ui.unit.TextUnit(
                                        9f,
                                        androidx.compose.ui.unit.TextUnitType.Sp
                                    ),
                                )
                            }
                        }
                    }
                }
                // Relleno si la fila tiene solo 1 elemento
                if (row.size == 1) Spacer(Modifier.weight(1f))
            }
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
