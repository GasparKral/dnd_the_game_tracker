package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.github.gasparkral.dnd.model.AttributesDto
import io.github.gasparkral.dnd.model.CatalogEntry
import io.github.gasparkral.dnd.model.ChoiceSchema
import io.github.gasparkral.dnd.model.CreationStep
import io.github.gasparkral.dnd.ui.theme.*
import io.github.gasparkral.dnd.ui.viewmodel.CharacterCreationViewModel
import kotlinx.serialization.json.JsonPrimitive
import org.koin.androidx.compose.koinViewModel
import org.koin.core.parameter.parametersOf

@Composable
fun CharacterCreationScreen(
    modifier: Modifier = Modifier,
    playerName: String,
    onBack: () -> Unit = {},
    onCharacterCreated: () -> Unit = {},
) {
    val viewModel: CharacterCreationViewModel = koinViewModel(
        parameters = { parametersOf(playerName) }
    )
    val state by viewModel.uiState.collectAsStateWithLifecycle()

    // Navegar cuando el servidor confirma que el personaje está completo
    LaunchedEffect(state.isComplete) {
        if (state.isComplete) onCharacterCreated()
    }

    when {
        state.isLoadingCatalogs -> FullScreenLoading("Preparando el mundo…")
        state.catalogError != null -> FullScreenError(state.catalogError!!) {
            onBack()
        }
        else -> WizardContent(
            modifier = modifier,
            state = state,
            viewModel = viewModel,
            onBack = onBack,
        )
    }
}

// ---------------------------------------------------------------------------
// Cuerpo principal del wizard
// ---------------------------------------------------------------------------

@Composable
private fun WizardContent(
    modifier: Modifier,
    state: io.github.gasparkral.dnd.ui.viewmodel.CreationUiState,
    viewModel: CharacterCreationViewModel,
    onBack: () -> Unit,
) {
    Column(modifier.padding(16.dp)) {

        // ── Cabecera ──────────────────────────────────────────────────────
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(onClick = {
                if (!viewModel.back()) onBack()
            }) {
                Icon(
                    Icons.AutoMirrored.Filled.ArrowBack,
                    contentDescription = "Atrás"
                )
            }
            Spacer(Modifier.width(8.dp))
            Text(
                text = stepTitle(state.currentStep),
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.weight(1f),
            )
            Text(
                text = "${state.stepIndex + 1} / ${state.totalSteps}",
                style = MaterialTheme.typography.bodySmall,
                color = Ash,
            )
        }

        LinearProgressIndicator(
            progress = { (state.stepIndex + 1).toFloat() / state.totalSteps },
            color = Gold,
            trackColor = Iron,
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 8.dp),
        )

        Spacer(Modifier.height(8.dp))

        // ── Contenido del paso ────────────────────────────────────────────
        Box(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .verticalScroll(rememberScrollState())
        ) {
            when (state.currentStep) {
                CreationStep.Name -> StepName(
                    name = state.localName,
                    onChange = viewModel::onNameChange,
                )
                CreationStep.Race -> StepCatalogPicker(
                    title = "Elige tu especie",
                    entries = state.races,
                    selectedId = state.selectedRaceId,
                    choices = state.choices,
                    onEntrySelected = viewModel::onRaceSelected,
                    onChoiceAnswered = { id, v -> viewModel.onChoiceAnswered(id, v) },
                )
                CreationStep.Class -> StepCatalogPicker(
                    title = "Elige tu clase",
                    entries = state.classes,
                    selectedId = state.selectedClassId,
                    choices = state.choices,
                    onEntrySelected = viewModel::onClassSelected,
                    onChoiceAnswered = { id, v -> viewModel.onChoiceAnswered(id, v) },
                )
                CreationStep.Background -> StepCatalogPicker(
                    title = "Elige tu trasfondo",
                    entries = state.backgrounds,
                    selectedId = state.selectedBackgroundId,
                    choices = state.choices,
                    onEntrySelected = viewModel::onBackgroundSelected,
                    onChoiceAnswered = { id, v -> viewModel.onChoiceAnswered(id, v) },
                )
                CreationStep.Attributes -> StepAttributes(
                    attributes = state.attributes,
                    onChanged = viewModel::onAttributeChanged,
                )
                CreationStep.Feats -> StepFeats()
                CreationStep.Review -> StepReview(state)
                CreationStep.Complete -> Unit
            }
        }

        // ── Errores del servidor ──────────────────────────────────────────
        if (state.stepErrors.isNotEmpty()) {
            Spacer(Modifier.height(6.dp))
            state.stepErrors.forEach { err ->
                Text(
                    text = "✦ $err",
                    color = Ember,
                    style = MaterialTheme.typography.bodySmall,
                )
            }
        }

        Spacer(Modifier.height(8.dp))

        // ── Botón avanzar ─────────────────────────────────────────────────
        Button(
            onClick = { viewModel.advance() },
            enabled = state.canAdvance && !state.isSaving,
            colors = ButtonDefaults.buttonColors(
                containerColor = Gold,
                contentColor = Void,
                disabledContainerColor = Iron,
                disabledContentColor = Ash,
            ),
            modifier = Modifier.fillMaxWidth(),
        ) {
            if (state.isSaving) {
                CircularProgressIndicator(
                    color = Void,
                    modifier = Modifier.size(18.dp),
                    strokeWidth = 2.dp,
                )
            } else if (state.currentStep == CreationStep.Review) {
                Text("Crear personaje", style = MaterialTheme.typography.labelLarge)
            } else {
                Text("Siguiente", style = MaterialTheme.typography.labelLarge)
                Spacer(Modifier.width(4.dp))
                Icon(Icons.AutoMirrored.Filled.ArrowForward, contentDescription = null)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Paso: nombre
// ---------------------------------------------------------------------------

@Composable
private fun StepName(name: String, onChange: (String) -> Unit) {
    Column {
        Text("¿Cómo se llama tu personaje?", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(16.dp))
        OutlinedTextField(
            value = name,
            onValueChange = onChange,
            label = { Text("Nombre del personaje") },
            singleLine = true,
            colors = OutlinedTextFieldDefaults.colors(
                focusedBorderColor = Gold,
                unfocusedBorderColor = Iron,
                focusedTextColor = Parchment,
                unfocusedTextColor = Parchment,
                cursorColor = Aurum,
            ),
            modifier = Modifier.fillMaxWidth(),
        )
    }
}

// ---------------------------------------------------------------------------
// Paso: picker de catálogo (raza / clase / trasfondo) — genérico
// ---------------------------------------------------------------------------

@Composable
private fun StepCatalogPicker(
    title: String,
    entries: List<CatalogEntry>,
    selectedId: String,
    choices: Map<String, kotlinx.serialization.json.JsonElement>,
    onEntrySelected: (String) -> Unit,
    onChoiceAnswered: (String, JsonPrimitive) -> Unit,
) {
    Column {
        Text(title, style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(8.dp))

        if (entries.isEmpty()) {
            Text("No hay opciones disponibles.", color = Ash, style = MaterialTheme.typography.bodySmall)
            return@Column
        }

        entries.forEach { entry ->
            val isSelected = entry.id == selectedId
            Card(
                onClick = { onEntrySelected(entry.id) },
                colors = CardDefaults.cardColors(
                    containerColor = if (isSelected) Crypt else MaterialTheme.colorScheme.surface,
                ),
                border = if (isSelected) CardDefaults.outlinedCardBorder() else null,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 3.dp),
            ) {
                Column(Modifier.padding(12.dp)) {
                    Row(verticalAlignment = Alignment.CenterVertically) {
                        RadioButton(
                            selected = isSelected,
                            onClick = { onEntrySelected(entry.id) },
                            colors = RadioButtonDefaults.colors(
                                selectedColor = Gold,
                                unselectedColor = Ash,
                            ),
                        )
                        Spacer(Modifier.width(4.dp))
                        Column {
                            Text(entry.name, style = MaterialTheme.typography.bodyMedium)
                            if (!entry.source.isNullOrBlank()) {
                                Text(
                                    entry.source,
                                    style = MaterialTheme.typography.labelSmall,
                                    color = Ash,
                                )
                            }
                        }
                    }

                    // Descripción y traits al expandirse la selección
                    if (isSelected) {
                        entry.description?.let { desc ->
                            Spacer(Modifier.height(6.dp))
                            Text(desc, style = MaterialTheme.typography.bodySmall, color = Parchment)
                        }
                        if (entry.traitsPreview.isNotEmpty()) {
                            Spacer(Modifier.height(4.dp))
                            Text(
                                "Rasgos: ${entry.traitsPreview.joinToString(", ")}",
                                style = MaterialTheme.typography.labelSmall,
                                color = Aurum,
                            )
                        }
                        // Renderizar choices dinámicos de la entrada
                        entry.choices.forEach { schema ->
                            Spacer(Modifier.height(10.dp))
                            ChoiceRenderer(
                                schema = schema,
                                currentValue = choices[schema.id],
                                onAnswer = { onChoiceAnswered(schema.id, it) },
                            )
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Renderizador genérico de ChoiceSchema
// ---------------------------------------------------------------------------

@Composable
private fun ChoiceRenderer(
    schema: ChoiceSchema,
    currentValue: kotlinx.serialization.json.JsonElement?,
    onAnswer: (JsonPrimitive) -> Unit,
) {
    when (schema) {
        is ChoiceSchema.SingleSelect -> {
            Text(schema.label, style = MaterialTheme.typography.labelMedium, color = Gold)
            Spacer(Modifier.height(4.dp))
            schema.options.forEach { opt ->
                val selected = currentValue?.let {
                    (it as? JsonPrimitive)?.content == opt.id
                } == true
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    RadioButton(
                        selected = selected,
                        onClick = { onAnswer(JsonPrimitive(opt.id)) },
                        colors = RadioButtonDefaults.colors(selectedColor = Aurum, unselectedColor = Ash),
                    )
                    Column {
                        Text(opt.label, style = MaterialTheme.typography.bodySmall)
                        opt.description?.let {
                            Text(it, style = MaterialTheme.typography.labelSmall, color = Ash)
                        }
                    }
                }
            }
        }

        is ChoiceSchema.TextInput -> {
            Text(schema.label, style = MaterialTheme.typography.labelMedium, color = Gold)
            Spacer(Modifier.height(4.dp))
            val text = (currentValue as? JsonPrimitive)?.content ?: ""
            OutlinedTextField(
                value = text,
                onValueChange = { onAnswer(JsonPrimitive(it)) },
                placeholder = schema.placeholder?.let { { Text(it) } },
                singleLine = true,
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = Gold,
                    unfocusedBorderColor = Iron,
                    focusedTextColor = Parchment,
                    unfocusedTextColor = Parchment,
                ),
                modifier = Modifier.fillMaxWidth(),
            )
        }

        // MultiSelect y NumberInput — versión básica, extensible
        is ChoiceSchema.MultiSelect -> {
            Text(schema.label, style = MaterialTheme.typography.labelMedium, color = Gold)
            Text("Elige entre ${schema.min} y ${schema.max}", style = MaterialTheme.typography.labelSmall, color = Ash)
            // TODO: implementar multi-check cuando la API exponga opciones con este tipo
        }

        is ChoiceSchema.PointBuy, is ChoiceSchema.NumberInput -> {
            // Manejados por StepAttributes o ignorados si llegan en un catálogo
        }
    }
}

// ---------------------------------------------------------------------------
// Paso: atributos (point-buy)
// ---------------------------------------------------------------------------

private val STAT_FIELDS = listOf(
    "str" to "Fuerza",
    "dex" to "Destreza",
    "con" to "Constitución",
    "int" to "Inteligencia",
    "wis" to "Sabiduría",
    "cha" to "Carisma",
)

@Composable
private fun StepAttributes(
    attributes: AttributesDto,
    onChanged: (String, Int) -> Unit,
) {
    val budget = 27
    val spent = attributes.pointBuyCost()
    val remaining = budget - spent

    Column {
        Text("Atributos (Point Buy)", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(4.dp))
        Row(verticalAlignment = Alignment.CenterVertically) {
            Text("Puntos restantes: ", style = MaterialTheme.typography.bodyMedium)
            Text(
                "$remaining",
                style = MaterialTheme.typography.bodyMedium,
                color = if (remaining < 0) Ember else Aurum,
            )
        }
        Spacer(Modifier.height(8.dp))

        STAT_FIELDS.forEach { (field, label) ->
            val value = attributes.getField(field)
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(vertical = 4.dp),
            ) {
                Text(label, modifier = Modifier.width(100.dp), style = MaterialTheme.typography.labelLarge)

                val nextCost = AttributesDto.costFor(value + 1) - AttributesDto.costFor(value)

                IconButton(
                    onClick = { if (value > 8) onChanged(field, value - 1) },
                    enabled = value > 8,
                ) { Text("−", color = Aurum) }

                Text(
                    "$value",
                    modifier = Modifier.width(28.dp),
                    style = MaterialTheme.typography.titleSmall,
                )

                IconButton(
                    onClick = { if (value < 15 && remaining >= nextCost) onChanged(field, value + 1) },
                    enabled = value < 15 && remaining >= nextCost,
                ) { Text("+", color = Aurum) }

                Text(
                    statModifier(value),
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash,
                    modifier = Modifier.padding(start = 4.dp),
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Paso: dones (stub)
// ---------------------------------------------------------------------------

@Composable
private fun StepFeats() {
    Column {
        Text("Dones", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(8.dp))
        Text(
            "En DnD 2024 algunos trasfondos otorgan un don inicial. Próximamente podrás elegirlos aquí.",
            style = MaterialTheme.typography.bodyMedium,
            color = Ash,
        )
    }
}

// ---------------------------------------------------------------------------
// Paso: resumen
// ---------------------------------------------------------------------------

@Composable
private fun StepReview(state: io.github.gasparkral.dnd.ui.viewmodel.CreationUiState) {
    Column {
        Text("Resumen del personaje", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(12.dp))
        SummaryRow("Nombre", state.localName.ifBlank { state.draft.name ?: "—" })
        SummaryRow("Especie", state.selectedRace?.name ?: "—")
        SummaryRow("Clase", state.selectedClass?.name ?: "—")
        SummaryRow("Trasfondo", state.selectedBackground?.name ?: "—")
        Spacer(Modifier.height(8.dp))
        Text("Atributos", style = MaterialTheme.typography.titleSmall)
        Spacer(Modifier.height(4.dp))
        LazyRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            items(STAT_FIELDS) { (field, label) ->
                AttributeChip(label.take(3).uppercase(), state.attributes.getField(field))
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Componentes auxiliares
// ---------------------------------------------------------------------------

@Composable
private fun SummaryRow(label: String, value: String) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 2.dp)
    ) {
        Text("$label: ", style = MaterialTheme.typography.bodyMedium, color = Ash)
        Text(value, style = MaterialTheme.typography.bodyMedium)
    }
}

@Composable
private fun AttributeChip(stat: String, value: Int) {
    Card(colors = CardDefaults.cardColors(containerColor = Crypt)) {
        Column(
            modifier = Modifier.padding(horizontal = 12.dp, vertical = 8.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(stat, style = MaterialTheme.typography.labelSmall)
            Text("$value", style = MaterialTheme.typography.titleSmall)
            Text(statModifier(value), style = MaterialTheme.typography.bodySmall, color = Aurum)
        }
    }
}

@Composable
private fun FullScreenLoading(message: String) {
    Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            CircularProgressIndicator(color = Gold)
            Spacer(Modifier.height(12.dp))
            Text(message, color = Ash, style = MaterialTheme.typography.bodyMedium)
        }
    }
}

@Composable
private fun FullScreenError(message: String, onBack: () -> Unit) {
    Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
        Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = Modifier.padding(24.dp)) {
            Text("✦ $message", color = Ember, style = MaterialTheme.typography.bodyMedium)
            Spacer(Modifier.height(16.dp))
            Button(onClick = onBack) { Text("Volver") }
        }
    }
}

// ---------------------------------------------------------------------------
// Utilidades
// ---------------------------------------------------------------------------

private fun stepTitle(step: CreationStep) = when (step) {
    CreationStep.Name       -> "Nombre"
    CreationStep.Race       -> "Especie"
    CreationStep.Class      -> "Clase"
    CreationStep.Attributes -> "Atributos"
    CreationStep.Background -> "Trasfondo"
    CreationStep.Feats      -> "Dones"
    CreationStep.Review     -> "Resumen"
    CreationStep.Complete   -> "Completado"
}

private fun statModifier(score: Int): String {
    val mod = (score - 10) / 2
    return if (mod >= 0) "+$mod" else "$mod"
}
