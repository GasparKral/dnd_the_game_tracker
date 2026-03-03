package io.github.gasparkral.dnd.ui.component

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.infra.dbstruct.Character
import io.github.gasparkral.dnd.ui.theme.Ash
import io.github.gasparkral.dnd.ui.theme.Aurum
import io.github.gasparkral.dnd.ui.theme.Gold

@Composable
fun CharacterSelectionItem(
    modifier: Modifier = Modifier,
    character: Character,
    onClick: (Character) -> Unit
) {
    DndCard(
        modifier = modifier
            .fillMaxWidth()
            .clickable { onClick(character) }
    ) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
        ) {
            // Símbolo ornamental a modo de viñeta
            Text(
                text = "◆",
                color = Gold,
                style = MaterialTheme.typography.labelMedium,
                modifier = Modifier.padding(end = 12.dp)
            )

            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = character.characterName,
                    style = MaterialTheme.typography.titleMedium,
                )
                // TODO: cuando el modelo tenga raza/clase mostrarlas aquí
                Text(
                    text = "Personaje · Nivel 1",
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash
                )
            }

            Icon(
                imageVector = Icons.Filled.ChevronRight,
                contentDescription = null,
                tint = Aurum
            )
        }
    }
}
