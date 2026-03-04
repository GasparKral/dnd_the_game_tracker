package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.model.SavedCharacter
import io.github.gasparkral.dnd.ui.component.DndDivider
import io.github.gasparkral.dnd.ui.theme.*
import org.koin.compose.koinInject

@Composable
fun CharacterSelectionScreen(
    modifier: Modifier = Modifier,
    playerName: String,
    navigateToCreateCharacter: () -> Unit,
    navigateToDashboard: (draftId: String) -> Unit,
) {
    val repo: DraftRepository = koinInject()
    var characters by remember { mutableStateOf<List<SavedCharacter>>(emptyList()) }
    var isLoading by remember { mutableStateOf(true) }
    var error by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(playerName) {
        isLoading = true
        repo.getCharacters(playerName).fold(
            onOk = { response ->
                characters = response.characters
                isLoading = false
            },
            onErr = {
                // Si no hay campaña activa (503) no es un error crítico — lista vacía
                characters = emptyList()
                isLoading = false
            }
        )
    }

    Box(
        modifier = modifier.background(
            Brush.radialGradient(colors = listOf(Dungeon, Void), radius = 1400f)
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(horizontal = 24.dp)
                .padding(top = 40.dp, bottom = 24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {

            // ── Cabecera ──────────────────────────────────────────────────
            Text(
                text = "Elige tu Héroe",
                style = MaterialTheme.typography.displaySmall,
                textAlign = TextAlign.Center,
            )
            Spacer(Modifier.height(4.dp))
            Text(
                text = "Jugador: $playerName",
                style = MaterialTheme.typography.bodySmall,
                color = Aurum,
                textAlign = TextAlign.Center,
            )
            Spacer(Modifier.height(4.dp))
            Text(
                text = "Selecciona un personaje para continuar la aventura",
                style = MaterialTheme.typography.bodySmall,
                color = Ash,
                textAlign = TextAlign.Center,
            )

            Spacer(Modifier.height(20.dp))
            DndDivider(symbol = "✦")
            Spacer(Modifier.height(20.dp))

            // ── Lista / estado ────────────────────────────────────────────
            when {
                isLoading -> {
                    Spacer(Modifier.weight(1f))
                    CircularProgressIndicator(color = Gold)
                    Spacer(Modifier.weight(1f))
                }

                error != null -> {
                    Spacer(Modifier.weight(1f))
                    Text(
                        text = "✦ $error",
                        color = Ember,
                        style = MaterialTheme.typography.bodyMedium,
                        textAlign = TextAlign.Center,
                    )
                    Spacer(Modifier.weight(1f))
                }

                characters.isEmpty() -> {
                    Spacer(Modifier.weight(1f))
                    Text(
                        text = "No hay personajes.\nCrea uno para comenzar.",
                        style = MaterialTheme.typography.bodyMedium,
                        color = Ash,
                        textAlign = TextAlign.Center,
                    )
                    Spacer(Modifier.weight(1f))
                }

                else -> {
                    LazyColumn(
                        modifier = Modifier.weight(1f),
                        verticalArrangement = Arrangement.spacedBy(10.dp),
                    ) {
                        items(characters, key = { it.id }) { character ->
                            CharacterItem(
                                character = character,
                                onClick = { navigateToDashboard(character.id) },
                            )
                        }
                    }
                }
            }

            Spacer(Modifier.height(16.dp))
            DndDivider()
            Spacer(Modifier.height(16.dp))

            // ── Botón crear ───────────────────────────────────────────────
            OutlinedButton(
                onClick = navigateToCreateCharacter,
                colors = ButtonDefaults.outlinedButtonColors(contentColor = Aurum),
                shape = RoundedCornerShape(3.dp),
                modifier = Modifier.fillMaxWidth(0.75f),
            ) {
                Icon(Icons.Filled.Add, contentDescription = null, tint = Gold)
                Spacer(Modifier.width(8.dp))
                Text("Crear nuevo personaje", style = MaterialTheme.typography.labelLarge)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Item de la lista
// ---------------------------------------------------------------------------

@Composable
private fun CharacterItem(
    character: SavedCharacter,
    onClick: () -> Unit,
) {
    Card(
        onClick = onClick,
        colors = CardDefaults.cardColors(containerColor = Crypt),
        shape = RoundedCornerShape(4.dp),
        modifier = Modifier.fillMaxWidth(),
    ) {
        Row(
            modifier = Modifier
                .padding(horizontal = 16.dp, vertical = 12.dp)
                .fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(Modifier.weight(1f)) {
                Text(
                    text = character.name,
                    style = MaterialTheme.typography.titleSmall,
                )
                Spacer(Modifier.height(2.dp))
                Text(
                    text = "${character.raceId} · ${character.classId}",
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash,
                )
                Spacer(Modifier.height(2.dp))
                Text(
                    text = "PG: ${character.currentHp}/${character.maxHp}  ·  Nivel ${character.level}",
                    style = MaterialTheme.typography.labelSmall,
                    color = Aurum,
                )
            }
        }
    }
}
