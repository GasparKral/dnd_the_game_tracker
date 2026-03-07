package io.github.gasparkral.dnd.ui.screen

import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.github.gasparkral.dnd.model.*
import io.github.gasparkral.dnd.ui.theme.Ash
import io.github.gasparkral.dnd.ui.theme.Ember
import io.github.gasparkral.dnd.ui.viewmodel.SpellViewModel
import org.koin.androidx.compose.koinViewModel
import org.koin.core.parameter.parametersOf

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

private enum class SpellTab { Slots, Prepared, Known }

// ---------------------------------------------------------------------------
// Pantalla principal
// ---------------------------------------------------------------------------

@Composable
fun SpellScreen(
    modifier: Modifier = Modifier,
    draftId: String,
    onBack: () -> Unit = {},
) {
    val vm: SpellViewModel = koinViewModel(parameters = { parametersOf(draftId) })
    val state by vm.state.collectAsState()
    var activeTab by remember { mutableStateOf(SpellTab.Known) }

    Box(
        modifier
            .fillMaxSize()
            .background(Brush.verticalGradient(listOf(Color(0xFF0C0A09), Color(0xFF100C1A))))
    ) {
        Column(Modifier.fillMaxSize()) {

            // ── Cabecera ──────────────────────────────────────────────────
            SpellHeader(onBack = onBack)

            // ── Tabs ───────────────────────────────────────────────────────
            SpellTabRow(active = activeTab, onSelect = { activeTab = it })

            when {
                state.isLoading -> Box(
                    Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center,
                ) { CircularProgressIndicator(color = Color(0xFFA78BFA)) }

                state.error != null -> Box(
                    Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center,
                ) { Text("⚠ ${state.error}", color = Ember) }

                else -> when (activeTab) {
                    SpellTab.Slots -> SlotsTab(slots = state.slots, vm = vm)
                    SpellTab.Prepared -> PreparedTab(spells = state.preparedSpells)
                    SpellTab.Known -> KnownTab(
                        spells = state.knownSpells,
                        preparedIds = state.preparedSpells.map { it.id }.toSet(),
                        vm = vm,
                    )
                }
            }
        }

        // FAB solo en "Conocidos"
        if (!state.isLoading && state.error == null && activeTab == SpellTab.Known) {
            FloatingActionButton(
                onClick = vm::openAddDialog,
                modifier = Modifier
                    .align(Alignment.BottomEnd)
                    .padding(20.dp),
                containerColor = Color(0xFF4C1D95),
                contentColor = Color(0xFFEDE9FE),
                shape = RoundedCornerShape(16.dp),
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.padding(horizontal = 16.dp),
                ) {
                    Icon(Icons.Filled.Add, null, modifier = Modifier.size(20.dp))
                    Spacer(Modifier.width(6.dp))
                    Text("Hechizo", style = MaterialTheme.typography.labelLarge)
                }
            }
        }
    }

    if (state.showAddDialog) {
        AddSpellDialog(
            isSaving = state.isSaving,
            onDismiss = vm::closeAddDialog,
            onConfirm = { req -> vm.addSpell(req) },
        )
    }
}

// ---------------------------------------------------------------------------
// Cabecera
// ---------------------------------------------------------------------------

@Composable
private fun SpellHeader(onBack: () -> Unit) {
    Box(
        Modifier
            .fillMaxWidth()
            .background(Brush.verticalGradient(listOf(Color(0xFF1E1030), Color(0xFF0C0A09))))
    ) {
        // Línea violeta superior
        Box(
            Modifier
                .fillMaxWidth()
                .height(2.dp)
                .background(
                    Brush.horizontalGradient(
                        listOf(Color(0xFF4C1D95), Color(0xFFA78BFA), Color(0xFF4C1D95))
                    )
                )
        )
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 4.dp, vertical = 12.dp)
                .padding(top = 2.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Filled.ArrowBack, "Volver", tint = Color(0xFFA78BFA))
            }
            Box(
                modifier = Modifier
                    .size(40.dp)
                    .clip(RoundedCornerShape(10.dp))
                    .background(Color(0xFF1E1030))
                    .border(1.dp, Color(0xFF4C1D95), RoundedCornerShape(10.dp)),
                contentAlignment = Alignment.Center,
            ) { Text("✨", fontSize = 20.sp) }
            Spacer(Modifier.width(12.dp))
            Column(Modifier.weight(1f)) {
                Text(
                    "Hechizos",
                    style = MaterialTheme.typography.titleLarge,
                    color = Color(0xFFEDE9FE),
                    fontWeight = FontWeight.Bold,
                )
                Text("Grimorio del personaje", style = MaterialTheme.typography.bodySmall, color = Ash)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tab row
// ---------------------------------------------------------------------------

@Composable
private fun SpellTabRow(active: SpellTab, onSelect: (SpellTab) -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(Color(0xFF0C0A09))
            .padding(horizontal = 16.dp, vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        SpellTab.entries.forEach { tab ->
            val selected = active == tab
            val label = when (tab) {
                SpellTab.Slots -> "Espacios"
                SpellTab.Prepared -> "Preparados"
                SpellTab.Known -> "Conocidos"
            }
            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(20.dp))
                    .background(if (selected) Color(0xFF4C1D95) else Color(0xFF1C1917))
                    .border(
                        1.dp,
                        if (selected) Color(0xFF7C3AED) else Color(0xFF292524),
                        RoundedCornerShape(20.dp),
                    )
                    .clickable { onSelect(tab) }
                    .padding(horizontal = 14.dp, vertical = 7.dp),
            ) {
                Text(
                    label,
                    style = MaterialTheme.typography.labelMedium,
                    color = if (selected) Color(0xFFEDE9FE) else Ash,
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tab: Espacios de hechizo (editable)
// ---------------------------------------------------------------------------

@Composable
private fun SlotsTab(slots: List<SpellSlotLevel>, vm: SpellViewModel) {
    var hasChanges by remember { mutableStateOf(false) }

    LazyColumn(
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        item {
            Text(
                "Espacios disponibles por nivel",
                style = MaterialTheme.typography.titleSmall,
                color = Color(0xFFA78BFA),
                modifier = Modifier.padding(bottom = 4.dp),
            )
        }

        items((1..9).toList()) { lvl ->
            val slot = slots.find { it.level == lvl } ?: SpellSlotLevel(lvl, 0, 0)
            SlotRow(
                level = lvl,
                slot = slot,
                onTotalChange = { v -> vm.updateSlot(lvl, total = v); hasChanges = true },
                onRemainingChange = { v -> vm.updateSlot(lvl, remaining = v); hasChanges = true },
            )
        }

        if (hasChanges) {
            item {
                Spacer(Modifier.height(8.dp))
                Button(
                    onClick = { vm.saveSlots(); hasChanges = false },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = Color(0xFF4C1D95),
                        contentColor = Color(0xFFEDE9FE),
                    ),
                    shape = RoundedCornerShape(12.dp),
                ) { Text("💾 Guardar espacios") }
            }
        }
    }
}

@Composable
private fun SlotRow(
    level: Int,
    slot: SpellSlotLevel,
    onTotalChange: (Int) -> Unit,
    onRemainingChange: (Int) -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(Color(0xFF1C1917))
            .border(1.dp, Color(0xFF292524), RoundedCornerShape(12.dp))
            .padding(horizontal = 12.dp, vertical = 10.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        // Badge nivel
        Box(
            modifier = Modifier
                .size(34.dp)
                .clip(RoundedCornerShape(8.dp))
                .background(Color(0xFF1E1030))
                .border(1.dp, Color(0xFF4C1D95), RoundedCornerShape(8.dp)),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                "$level", style = MaterialTheme.typography.titleSmall,
                color = Color(0xFFA78BFA), fontWeight = FontWeight.Bold
            )
        }

        Spacer(Modifier.width(10.dp))
        Text(
            "Nivel $level",
            style = MaterialTheme.typography.bodyMedium,
            color = Color(0xFFD6D3D1),
            modifier = Modifier.weight(1f),
        )

        // Pips visuales (máx. 9)
        Row(horizontalArrangement = Arrangement.spacedBy(3.dp)) {
            repeat(slot.total.coerceIn(0, 9)) { i ->
                Box(
                    Modifier
                        .size(9.dp)
                        .clip(RoundedCornerShape(50))
                        .background(if (i < slot.remaining) Color(0xFF7C3AED) else Color(0xFF292524))
                )
            }
        }

        Spacer(Modifier.width(10.dp))

        // Inputs total / restantes
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            modifier = Modifier.width(80.dp),
        ) {
            SlotInput("Total", slot.total, onTotalChange, 9, Color(0xFFA78BFA))
            Spacer(Modifier.height(4.dp))
            SlotInput("Restantes", slot.remaining, onRemainingChange, slot.total, Color(0xFF34D399))
        }
    }
}

@Composable
private fun SlotInput(
    label: String,
    value: Int,
    onValueChange: (Int) -> Unit,
    max: Int,
    accentColor: Color,
) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        Text(label, style = MaterialTheme.typography.labelSmall, color = Ash, fontSize = 9.sp)
        BasicTextField(
            value = value.toString(),
            onValueChange = { s ->
                s.toIntOrNull()?.let { v -> onValueChange(v.coerceIn(0, max.coerceAtLeast(0))) }
            },
            singleLine = true,
            textStyle = LocalTextStyle.current.copy(
                textAlign = TextAlign.Center,
                fontSize = 14.sp,
                fontWeight = FontWeight.Bold,
                color = accentColor,
            ),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier
                .width(54.dp)
                .height(44.dp)
                .border(1.dp, accentColor.copy(alpha = 0.5f), RoundedCornerShape(8.dp))
                .padding(horizontal = 4.dp, vertical = 10.dp),
        )
    }
}

// ---------------------------------------------------------------------------
// Tab: Preparados (solo lectura)
// ---------------------------------------------------------------------------

@Composable
private fun PreparedTab(spells: List<Spell>) {
    if (spells.isEmpty()) {
        EmptySpellsPlaceholder(
            title = "Ningún hechizo preparado",
            subtitle = "Prepara hechizos desde la pestaña Conocidos",
        )
        return
    }
    LazyColumn(
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        item {
            Text(
                "${spells.size} hechizo(s) preparado(s)",
                style = MaterialTheme.typography.labelMedium,
                color = Color(0xFF34D399),
                modifier = Modifier.padding(bottom = 4.dp),
            )
        }
        items(spells, key = { it.id }) { spell ->
            SpellCard(spell = spell, isPrepared = true)
        }
    }
}

// ---------------------------------------------------------------------------
// Tab: Conocidos (toggle preparado + eliminar)
// ---------------------------------------------------------------------------

@Composable
private fun KnownTab(
    spells: List<Spell>,
    preparedIds: Set<String>,
    vm: SpellViewModel,
) {
    if (spells.isEmpty()) {
        EmptySpellsPlaceholder(
            title = "Sin hechizos conocidos",
            subtitle = "Pulsa ✨ para añadir tu primer hechizo",
        )
        return
    }

    val grouped = remember(spells) {
        spells.groupBy { it.level }.toSortedMap()
    }

    LazyColumn(
        contentPadding = PaddingValues(start = 16.dp, end = 16.dp, top = 8.dp, bottom = 100.dp),
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        grouped.forEach { (level, group) ->
            // Cabecera de nivel
            item(key = "hdr_$level") {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 6.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    val levelLabel = if (level == 0) "Trucos" else "Nivel $level"
                    Box(
                        modifier = Modifier
                            .clip(RoundedCornerShape(6.dp))
                            .background(Color(0xFF1E1030))
                            .border(1.dp, Color(0xFF4C1D95), RoundedCornerShape(6.dp))
                            .padding(horizontal = 10.dp, vertical = 3.dp),
                    ) {
                        Text(
                            levelLabel, style = MaterialTheme.typography.labelSmall,
                            color = Color(0xFFA78BFA)
                        )
                    }
                    Spacer(Modifier.width(8.dp))
                    HorizontalDivider(color = Color(0xFF1C1917))
                }
            }

            items(group, key = { it.id }) { spell ->
                SpellCard(
                    spell = spell,
                    isPrepared = spell.id in preparedIds,
                    onToggle = { vm.togglePrepared(spell) },
                    onRemove = { vm.removeSpell(spell) },
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tarjeta de hechizo expandible
// ---------------------------------------------------------------------------

@Composable
private fun SpellCard(
    spell: Spell,
    isPrepared: Boolean,
    onToggle: (() -> Unit)? = null,
    onRemove: (() -> Unit)? = null,
) {
    var expanded by remember { mutableStateOf(false) }
    val arrowRot by animateFloatAsState(
        targetValue = if (expanded) 180f else 0f,
        animationSpec = tween(180),
        label = "arrow",
    )

    val schoolColor = Color(spell.school.color())
    val prepBg = if (isPrepared) Color(0xFF1A2E0A) else Color(0xFF141210)
    val prepBorder = if (isPrepared) Color(0xFF166534) else Color(0xFF252220)

    // Badge de nivel
    val levelBadge = if (spell.level == 0) "Truco" else "Nv${spell.level}"

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(prepBg)
            .border(1.dp, prepBorder, RoundedCornerShape(12.dp)),
    ) {
        // ── Fila principal ────────────────────────────────────────────────
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable { expanded = !expanded }
                .padding(horizontal = 12.dp, vertical = 10.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            // Badge nivel/escuela
            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(6.dp))
                    .background(schoolColor.copy(alpha = 0.15f))
                    .border(1.dp, schoolColor.copy(alpha = 0.5f), RoundedCornerShape(6.dp))
                    .padding(horizontal = 7.dp, vertical = 3.dp),
                contentAlignment = Alignment.Center,
            ) {
                Text(
                    levelBadge,
                    fontSize = 10.sp,
                    fontWeight = FontWeight.Bold,
                    color = schoolColor,
                )
            }

            Spacer(Modifier.width(10.dp))

            Column(Modifier.weight(1f)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        spell.name,
                        style = MaterialTheme.typography.titleSmall,
                        color = Color(0xFFE7E5E4),
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f, fill = false),
                    )
                    if (spell.concentration) {
                        Spacer(Modifier.width(4.dp))
                        SpellBadge("C", Color(0xFFFBBF24))
                    }
                    if (spell.ritual) {
                        Spacer(Modifier.width(4.dp))
                        SpellBadge("R", Color(0xFF34D399))
                    }
                }
                Text(
                    spell.school.label(),
                    style = MaterialTheme.typography.labelSmall,
                    color = schoolColor.copy(alpha = 0.7f),
                    fontSize = 10.sp,
                )
            }

            // Toggle preparado (solo si se pasa callback)
            if (onToggle != null) {
                Spacer(Modifier.width(6.dp))
                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(6.dp))
                        .background(if (isPrepared) Color(0xFF166534) else Color(0xFF1C1917))
                        .border(
                            1.dp,
                            if (isPrepared) Color(0xFF34D399) else Color(0xFF292524),
                            RoundedCornerShape(6.dp),
                        )
                        .clickable { onToggle() }
                        .padding(horizontal = 8.dp, vertical = 4.dp),
                ) {
                    Text(
                        if (isPrepared) "✔" else "Prep.",
                        fontSize = 10.sp,
                        color = if (isPrepared) Color(0xFF34D399) else Ash,
                    )
                }
            }

            // Flecha expandir
            Spacer(Modifier.width(4.dp))
            Icon(
                Icons.Filled.KeyboardArrowDown,
                contentDescription = null,
                tint = Color(0xFF57534E),
                modifier = Modifier
                    .size(18.dp)
                    .rotate(arrowRot),
            )
        }

        // ── Detalle expandido ─────────────────────────────────────────────
        AnimatedVisibility(
            visible = expanded,
            enter = expandVertically() + fadeIn(),
            exit = shrinkVertically() + fadeOut(),
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Color(0xFF0C0A09))
                    .padding(horizontal = 14.dp, vertical = 10.dp),
                verticalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                // Meta rápida
                Row(horizontalArrangement = Arrangement.spacedBy(16.dp)) {
                    if (spell.castingTime.isNotBlank())
                        SpellMetaChip("⏱", spell.castingTime)
                    if (spell.range.isNotBlank())
                        SpellMetaChip("🎯", spell.range)
                    if (spell.duration.isNotBlank())
                        SpellMetaChip("⌛", spell.duration)
                }

                if (!spell.damage.isNullOrBlank())
                    SpellMetaChip("⚔️", spell.damage)
                if (!spell.savingThrow.isNullOrBlank())
                    SpellMetaChip("🛡", spell.savingThrow)

                if (spell.description.isNotBlank()) {
                    Text(
                        spell.description,
                        style = MaterialTheme.typography.bodySmall,
                        color = Color(0xFFA8A29E),
                        lineHeight = 18.sp,
                    )
                }
                if (spell.notes.isNotBlank()) {
                    Text(
                        "📜 ${spell.notes}",
                        style = MaterialTheme.typography.labelSmall,
                        color = Color(0xFF78716C),
                        fontSize = 11.sp,
                    )
                }

                // Botón eliminar (solo si se pasa callback)
                if (onRemove != null) {
                    OutlinedButton(
                        onClick = onRemove,
                        colors = ButtonDefaults.outlinedButtonColors(contentColor = Ember),
                        border = BorderStroke(1.dp, Color(0xFF7F1D1D)),
                        contentPadding = PaddingValues(horizontal = 12.dp, vertical = 4.dp),
                        modifier = Modifier.height(32.dp),
                    ) {
                        Icon(Icons.Filled.Delete, null, modifier = Modifier.size(12.dp))
                        Spacer(Modifier.width(4.dp))
                        Text("Olvidar hechizo", style = MaterialTheme.typography.labelSmall)
                    }
                }
            }
        }
    }
}

@Composable
private fun SpellBadge(text: String, color: Color) {
    Box(
        modifier = Modifier
            .clip(RoundedCornerShape(4.dp))
            .background(color.copy(alpha = 0.15f))
            .padding(horizontal = 4.dp, vertical = 1.dp),
    ) {
        Text(text, fontSize = 9.sp, fontWeight = FontWeight.Bold, color = color)
    }
}

@Composable
private fun SpellMetaChip(icon: String, value: String) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Text(icon, fontSize = 11.sp)
        Spacer(Modifier.width(3.dp))
        Text(value, style = MaterialTheme.typography.labelSmall, color = Color(0xFFD6D3D1))
    }
}

// ---------------------------------------------------------------------------
// Placeholder vacío
// ---------------------------------------------------------------------------

@Composable
private fun EmptySpellsPlaceholder(title: String, subtitle: String) {
    Box(
        Modifier.fillMaxSize(),
        contentAlignment = Alignment.Center,
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Text("✨", fontSize = 48.sp)
            Spacer(Modifier.height(8.dp))
            Text(title, style = MaterialTheme.typography.titleSmall, color = Color(0xFFA78BFA))
            Spacer(Modifier.height(4.dp))
            Text(
                subtitle, style = MaterialTheme.typography.bodySmall, color = Ash,
                textAlign = TextAlign.Center, modifier = Modifier.padding(horizontal = 32.dp)
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Diálogo añadir hechizo
// ---------------------------------------------------------------------------

@Composable
private fun AddSpellDialog(
    isSaving: Boolean,
    onDismiss: () -> Unit,
    onConfirm: (AddSpellRequest) -> Unit,
) {
    var name by remember { mutableStateOf("") }
    var levelText by remember { mutableStateOf("0") }
    var school by remember { mutableStateOf(SpellSchool.Unknown) }
    var castingTime by remember { mutableStateOf("") }
    var range by remember { mutableStateOf("") }
    var duration by remember { mutableStateOf("") }
    var description by remember { mutableStateOf("") }
    var damage by remember { mutableStateOf("") }
    var savingThrow by remember { mutableStateOf("") }
    var notes by remember { mutableStateOf("") }
    var concentration by remember { mutableStateOf(false) }
    var ritual by remember { mutableStateOf(false) }
    var prepared by remember { mutableStateOf(false) }
    var nameError by remember { mutableStateOf(false) }

    AlertDialog(
        onDismissRequest = onDismiss,
        containerColor = Color(0xFF1C1917),
        titleContentColor = Color(0xFFEDE9FE),
        title = { Text("✨ Añadir hechizo") },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {

                // Nombre + Nivel
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedTextField(
                        value = name,
                        onValueChange = { name = it; nameError = false },
                        label = { Text("Nombre *") },
                        isError = nameError,
                        singleLine = true,
                        modifier = Modifier.weight(1f),
                    )
                    OutlinedTextField(
                        value = levelText,
                        onValueChange = { levelText = it.filter(Char::isDigit).take(1) },
                        label = { Text("Nv") },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        singleLine = true,
                        modifier = Modifier.width(64.dp),
                    )
                }

                // Escuela — chips
                Text("Escuela", style = MaterialTheme.typography.labelSmall, color = Ash)
                LazyRow(
                    horizontalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    items(SpellSchool.entries) { s ->
                        val sel = school == s
                        FilterChip(
                            selected = sel,
                            onClick = { school = s },
                            label = { Text("${s.emoji()} ${s.label()}", fontSize = 11.sp) },
                        )
                    }
                }

                // Tiempo / Alcance / Duración
                Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                    OutlinedTextField(
                        value = castingTime,
                        onValueChange = { castingTime = it },
                        label = { Text("Tiempo") },
                        singleLine = true,
                        modifier = Modifier.weight(1f),
                    )
                    OutlinedTextField(
                        value = range,
                        onValueChange = { range = it },
                        label = { Text("Alcance") },
                        singleLine = true,
                        modifier = Modifier.weight(1f),
                    )
                }
                OutlinedTextField(
                    value = duration,
                    onValueChange = { duration = it },
                    label = { Text("Duración") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )

                // Daño / Salvación
                Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                    OutlinedTextField(
                        value = damage,
                        onValueChange = { damage = it },
                        label = { Text("Daño") },
                        singleLine = true,
                        modifier = Modifier.weight(1f),
                        placeholder = { Text("8d6 fuego", fontSize = 11.sp) },
                    )
                    OutlinedTextField(
                        value = savingThrow,
                        onValueChange = { savingThrow = it },
                        label = { Text("Salvación") },
                        singleLine = true,
                        modifier = Modifier.weight(1f),
                    )
                }

                // Descripción
                OutlinedTextField(
                    value = description,
                    onValueChange = { description = it },
                    label = { Text("Descripción") },
                    modifier = Modifier.fillMaxWidth(),
                    maxLines = 3,
                )

                // Notas
                OutlinedTextField(
                    value = notes,
                    onValueChange = { notes = it },
                    label = { Text("Notas") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )

                // Flags
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    SpellFlag("Concentración", concentration) { concentration = it }
                    SpellFlag("Ritual", ritual) { ritual = it }
                    SpellFlag("Preparado", prepared) { prepared = it }
                }
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    if (name.isBlank()) {
                        nameError = true; return@TextButton
                    }
                    onConfirm(
                        AddSpellRequest(
                            name = name.trim(),
                            level = levelText.toIntOrNull()?.coerceIn(0, 9) ?: 0,
                            school = school,
                            castingTime = castingTime.trim(),
                            range = range.trim(),
                            duration = duration.trim(),
                            description = description.trim(),
                            damage = damage.trim().takeIf { it.isNotBlank() },
                            savingThrow = savingThrow.trim().takeIf { it.isNotBlank() },
                            notes = notes.trim(),
                            concentration = concentration,
                            ritual = ritual,
                            prepared = prepared,
                        )
                    )
                },
                enabled = !isSaving,
            ) {
                if (isSaving)
                    CircularProgressIndicator(Modifier.size(16.dp), strokeWidth = 2.dp, color = Color(0xFFA78BFA))
                else
                    Text("Añadir", color = Color(0xFFA78BFA))
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss, enabled = !isSaving) {
                Text("Cancelar", color = Ash)
            }
        },
    )
}

@Composable
private fun SpellFlag(label: String, checked: Boolean, onChecked: (Boolean) -> Unit) {
    Row(verticalAlignment = Alignment.CenterVertically) {
        Checkbox(
            checked = checked,
            onCheckedChange = onChecked,
            modifier = Modifier.size(20.dp),
            colors = CheckboxDefaults.colors(
                checkedColor = Color(0xFF7C3AED),
                uncheckedColor = Ash,
            ),
        )
        Spacer(Modifier.width(4.dp))
        Text(label, style = MaterialTheme.typography.labelSmall, color = Ash)
    }
}
