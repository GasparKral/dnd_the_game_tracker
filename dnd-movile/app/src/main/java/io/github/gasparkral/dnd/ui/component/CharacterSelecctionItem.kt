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
import io.github.gasparkral.dnd.model.CharacterDraft
import io.github.gasparkral.dnd.ui.theme.Ash
import io.github.gasparkral.dnd.ui.theme.Aurum
import io.github.gasparkral.dnd.ui.theme.Gold

@Composable
fun CharacterSelectionItem(
    modifier: Modifier = Modifier,
    draft: CharacterDraft,
    onClick: (CharacterDraft) -> Unit,
) {
    DndCard(
        modifier = modifier
            .fillMaxWidth()
            .clickable { onClick(draft) }
    ) {
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text(
                text = "◆",
                color = Gold,
                style = MaterialTheme.typography.labelMedium,
                modifier = Modifier.padding(end = 12.dp),
            )
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = draft.name ?: "Sin nombre",
                    style = MaterialTheme.typography.titleMedium,
                )
                val subtitle = listOfNotNull(draft.raceId, draft.classId)
                    .joinToString(" · ")
                    .ifBlank { "Personaje en creación" }
                Text(
                    text = subtitle,
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash,
                )
                Text(
                    text = draft.step.name.lowercase().replaceFirstChar { it.uppercase() },
                    style = MaterialTheme.typography.labelSmall,
                    color = Aurum,
                )
            }
            Icon(
                imageVector = Icons.Filled.ChevronRight,
                contentDescription = null,
                tint = Aurum,
            )
        }
    }
}
