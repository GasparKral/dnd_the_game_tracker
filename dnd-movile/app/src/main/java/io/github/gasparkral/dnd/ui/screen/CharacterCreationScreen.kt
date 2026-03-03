package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.ArrowForward
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/**
 * Wizard de creación de personaje al estilo DnD 5.5e (2024).
 *
 * Pasos:
 *  0 - Nombre
 *  1 - Especie (raza)
 *  2 - Clase
 *  3 - Trasfondo (Background)
 *  4 - Atributos (point buy simplificado)
 *  5 - Resumen y confirmación
 *
 *  TODO: conectar con CharacterDAO para persistir al confirmar.
 */
@Composable
fun CharacterCreationScreen(
    modifier: Modifier = Modifier,
    onBack: () -> Unit = {},
    onCharacterCreated: () -> Unit = {}
) {
    var step by remember { mutableIntStateOf(0) }
    val totalSteps = 6

    // Estado del wizard — sin modelo de datos definitivo aún
    var charName by remember { mutableStateOf("") }
    var selectedSpecies by remember { mutableStateOf("") }
    var selectedClass by remember { mutableStateOf("") }
    var selectedBackground by remember { mutableStateOf("") }
    val attributes = remember {
        mutableStateMapOf(
            "FUE" to 10, "DES" to 10, "CON" to 10,
            "INT" to 10, "SAB" to 10, "CAR" to 10
        )
    }

    Column(modifier.padding(16.dp)) {

        // Cabecera con paso actual
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            if (step > 0) {
                IconButton(onClick = { step-- }) {
                    Icon(Icons.Filled.ArrowBack, contentDescription = "Paso anterior")
                }
            } else {
                IconButton(onClick = onBack) {
                    Icon(Icons.Filled.ArrowBack, contentDescription = "Cancelar")
                }
            }
            Spacer(Modifier.width(8.dp))
            Text(
                "Crear personaje  (${step + 1}/$totalSteps)",
                style = MaterialTheme.typography.headlineSmall,
                modifier = Modifier.weight(1f)
            )
        }

        LinearProgressIndicator(
            progress = { (step + 1).toFloat() / totalSteps },
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 8.dp)
        )

        Spacer(Modifier.height(8.dp))

        // Contenido según el paso
        Box(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .verticalScroll(rememberScrollState())
        ) {
            when (step) {
                0 -> StepName(charName) { charName = it }
                1 -> StepSpecies(selectedSpecies) { selectedSpecies = it }
                2 -> StepClass(selectedClass) { selectedClass = it }
                3 -> StepBackground(selectedBackground) { selectedBackground = it }
                4 -> StepAttributes(attributes)
                5 -> StepSummary(charName, selectedSpecies, selectedClass, selectedBackground, attributes)
            }
        }

        // Botón avanzar / confirmar
        val canAdvance = when (step) {
            0 -> charName.isNotBlank()
            1 -> selectedSpecies.isNotBlank()
            2 -> selectedClass.isNotBlank()
            3 -> selectedBackground.isNotBlank()
            else -> true
        }

        Button(
            onClick = {
                if (step < totalSteps - 1) step++
                else {
                    // TODO: llamar al DAO para persistir el personaje
                    onCharacterCreated()
                }
            },
            enabled = canAdvance,
            modifier = Modifier
                .fillMaxWidth()
                .padding(top = 8.dp)
        ) {
            if (step < totalSteps - 1) {
                Text("Siguiente")
                Icon(Icons.Filled.ArrowForward, contentDescription = null)
            } else {
                Text("Crear personaje")
            }
        }
    }
}

// ─── Pasos ───────────────────────────────────────────────────────────────────

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
            modifier = Modifier.fillMaxWidth()
        )
    }
}

@Composable
private fun StepSpecies(selected: String, onSelect: (String) -> Unit) {
    // Especies del manual 2024
    val species = listOf(
        "Humano", "Elfo", "Enano", "Mediano", "Gnomo",
        "Semiorco", "Semielfo", "Tiefling", "Dracónido", "Aasimar"
    )
    Column {
        Text("Elige tu especie", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(8.dp))
        species.forEach { s ->
            OptionRow(label = s, selected = selected == s, onClick = { onSelect(s) })
        }
    }
}

@Composable
private fun StepClass(selected: String, onSelect: (String) -> Unit) {
    val classes = listOf(
        "Bárbaro", "Bardo", "Clérigo", "Druida", "Guerrero",
        "Hechicero", "Mago", "Monje", "Paladín", "Pícaro",
        "Cazador", "Brujo", "Artificiero"
    )
    Column {
        Text("Elige tu clase", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(8.dp))
        classes.forEach { c ->
            OptionRow(label = c, selected = selected == c, onClick = { onSelect(c) })
        }
    }
}

@Composable
private fun StepBackground(selected: String, onSelect: (String) -> Unit) {
    val backgrounds = listOf(
        "Acólito", "Artesano", "Charlatán", "Criminal", "Eremita",
        "Forastero", "Marinero", "Noble", "Sabio", "Soldado"
    )
    Column {
        Text("Elige tu trasfondo", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(4.dp))
        Text(
            "El trasfondo concede competencias y rasgos de personalidad (DnD 2024).",
            style = MaterialTheme.typography.bodySmall
        )
        Spacer(Modifier.height(8.dp))
        backgrounds.forEach { b ->
            OptionRow(label = b, selected = selected == b, onClick = { onSelect(b) })
        }
    }
}

@Composable
private fun StepAttributes(attributes: MutableMap<String, Int>) {
    // Point buy simplificado: cada stat entre 8-15, presupuesto de 27 puntos
    // Coste: 8=0, 9=1, 10=2, 11=3, 12=4, 13=5, 14=7, 15=9
    val costTable = mapOf(8 to 0, 9 to 1, 10 to 2, 11 to 3, 12 to 4, 13 to 5, 14 to 7, 15 to 9)
    val budget = 27
    val spent = attributes.values.sumOf { costTable[it] ?: 0 }
    val remaining = budget - spent

    Column {
        Text("Atributos (Point Buy)", style = MaterialTheme.typography.titleMedium)
        Row {
            Text("Puntos restantes: ", style = MaterialTheme.typography.bodyMedium)
            Text(
                "$remaining",
                style = MaterialTheme.typography.bodyMedium,
                color = if (remaining < 0) MaterialTheme.colorScheme.error
                else MaterialTheme.colorScheme.primary
            )
        }
        Spacer(Modifier.height(8.dp))

        attributes.keys.toList().forEach { stat ->
            val value = attributes[stat] ?: 10
            Row(
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp)
            ) {
                Text(stat, modifier = Modifier.width(48.dp))
                IconButton(
                    onClick = { if (value > 8) attributes[stat] = value - 1 },
                    enabled = value > 8
                ) { Text("-") }
                Text(
                    "$value",
                    modifier = Modifier.width(32.dp),
                    style = MaterialTheme.typography.titleSmall
                )
                IconButton(
                    onClick = {
                        val nextCost = (costTable[value + 1] ?: 99) - (costTable[value] ?: 0)
                        if (value < 15 && remaining >= nextCost) attributes[stat] = value + 1
                    },
                    enabled = value < 15
                ) { Text("+") }
                Text(
                    "Mod: ${modifier(value)}",
                    style = MaterialTheme.typography.bodySmall
                )
            }
        }
    }
}

@Composable
private fun StepSummary(
    name: String,
    species: String,
    charClass: String,
    background: String,
    attributes: Map<String, Int>
) {
    Column {
        Text("Resumen del personaje", style = MaterialTheme.typography.titleMedium)
        Spacer(Modifier.height(12.dp))
        SummaryRow("Nombre", name)
        SummaryRow("Especie", species)
        SummaryRow("Clase", charClass)
        SummaryRow("Trasfondo", background)
        Spacer(Modifier.height(8.dp))
        Text("Atributos", style = MaterialTheme.typography.titleSmall)
        LazyRow(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            items(attributes.entries.toList()) { (stat, value) ->
                AttributeChip(stat, value)
            }
        }
    }
}

// ─── Utilidades de UI ────────────────────────────────────────────────────────

@Composable
private fun OptionRow(label: String, selected: Boolean, onClick: () -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 2.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        RadioButton(selected = selected, onClick = onClick)
        Text(label)
    }
}

@Composable
private fun SummaryRow(label: String, value: String) {
    Row(modifier = Modifier.fillMaxWidth().padding(vertical = 2.dp)) {
        Text("$label: ", style = MaterialTheme.typography.bodyMedium)
        Text(value, style = MaterialTheme.typography.bodyMedium)
    }
}

@Composable
private fun AttributeChip(stat: String, value: Int) {
    Card {
        Column(
            modifier = Modifier.padding(8.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Text(stat, style = MaterialTheme.typography.labelSmall)
            Text("$value", style = MaterialTheme.typography.titleSmall)
            Text(modifier(value), style = MaterialTheme.typography.bodySmall)
        }
    }
}

private fun modifier(score: Int): String {
    val mod = (score - 10) / 2
    return if (mod >= 0) "+$mod" else "$mod"
}
