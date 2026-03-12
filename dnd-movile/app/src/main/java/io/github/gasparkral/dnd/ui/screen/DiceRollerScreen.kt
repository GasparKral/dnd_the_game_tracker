package io.github.gasparkral.dnd.ui.screen

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import org.koin.androidx.compose.koinViewModel
import io.github.gasparkral.dnd.model.*
import io.github.gasparkral.dnd.ui.theme.*
import io.github.gasparkral.dnd.ui.viewmodel.DiceRollerViewModel

// ─── Pantalla principal ───────────────────────────────────────────────────────

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DiceRollerScreen(
    modifier: Modifier = Modifier,
    onBack: () -> Unit = {},
    vm: DiceRollerViewModel = koinViewModel(),
) {
    val state by vm.uiState.collectAsStateWithLifecycle()

    Scaffold(
        modifier = modifier,
        containerColor = Abyss,
        topBar = {
            TopAppBar(
                title = {
                    Text(
                        "🎲 Tirar dados",
                        color = Aurum,
                        fontWeight = FontWeight.Bold,
                    )
                },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Icon(
                            Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = "Volver",
                            tint = Bone,
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = Crypt),
            )
        },
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
            contentPadding = PaddingValues(vertical = 16.dp),
        ) {

            // ── Selector de dados ──────────────────────────────────────
            item {
                SectionCard(title = "Dados") {
                    DiceGrid(
                        counts    = state.counts,
                        onInc     = vm::incrementDice,
                        onDec     = vm::decrementDice,
                    )
                }
            }

            // ── Modificador ────────────────────────────────────────────
            item {
                SectionCard(title = "Modificador") {
                    ModifierRow(
                        modifier  = state.modifier,
                        onChange  = vm::setModifier,
                    )
                }
            }

            // ── Ventaja / Desventaja (solo con d20) ────────────────────
            if (state.hasD20) {
                item {
                    SectionCard(title = "Modo (d20)") {
                        ModeRow(
                            mode     = state.mode,
                            onChange = vm::setMode,
                        )
                    }
                }
            }

            // ── Etiqueta ───────────────────────────────────────────────
            item {
                OutlinedTextField(
                    value         = state.label,
                    onValueChange = vm::setLabel,
                    label         = { Text("Etiqueta (ej: Ataque, Percepción…)", color = Ash) },
                    singleLine    = true,
                    modifier      = Modifier.fillMaxWidth(),
                    colors        = OutlinedTextFieldDefaults.colors(
                        focusedBorderColor   = Aurum,
                        unfocusedBorderColor = Iron,
                        focusedTextColor     = Bone,
                        unfocusedTextColor   = Bone,
                        cursorColor          = Aurum,
                    ),
                )
            }

            // ── Error ──────────────────────────────────────────────────
            state.errorMsg?.let { err ->
                item {
                    Text(err, color = Ember, fontSize = 13.sp)
                }
            }

            // ── Botón Tirar ────────────────────────────────────────────
            item {
                Button(
                    onClick  = vm::roll,
                    enabled  = !state.rolling,
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(52.dp),
                    colors   = ButtonDefaults.buttonColors(
                        containerColor         = Gold,
                        contentColor           = Abyss,
                        disabledContainerColor = Iron,
                        disabledContentColor   = Ash,
                    ),
                    shape = RoundedCornerShape(12.dp),
                ) {
                    if (state.rolling) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(20.dp),
                            color    = Aurum,
                            strokeWidth = 2.dp,
                        )
                    } else {
                        Text(
                            "⚡ Tirar",
                            fontWeight = FontWeight.ExtraBold,
                            fontSize   = 16.sp,
                        )
                    }
                }
            }

            // ── Historial ──────────────────────────────────────────────
            if (state.history.isNotEmpty()) {
                item {
                    Text(
                        "Historial reciente",
                        color     = Ash,
                        fontSize  = 11.sp,
                        modifier  = Modifier.padding(top = 4.dp),
                    )
                }
                items(state.history) { result ->
                    RollHistoryItem(result = result)
                }
            }
        }
    }
}

// ─── Componentes internos ─────────────────────────────────────────────────────

@Composable
private fun SectionCard(
    title: String,
    content: @Composable ColumnScope.() -> Unit,
) {
    Card(
        colors   = CardDefaults.cardColors(containerColor = Crypt),
        modifier = Modifier.fillMaxWidth(),
        shape    = RoundedCornerShape(12.dp),
    ) {
        Column(Modifier.padding(12.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
            Text(title, color = Ash, fontSize = 11.sp, fontWeight = FontWeight.SemiBold)
            content()
        }
    }
}

// ── DiceGrid ─────────────────────────────────────────────────────────────────

@Composable
private fun DiceGrid(
    counts: Map<DiceType, Int>,
    onInc: (DiceType) -> Unit,
    onDec: (DiceType) -> Unit,
) {
    val diceList = DiceType.entries

    // 4 columnas
    val rows = diceList.chunked(4)
    Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
        for (row in rows) {
            Row(
                modifier              = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                for (dice in row) {
                    DiceChip(
                        dice    = dice,
                        count   = counts[dice] ?: 0,
                        onInc   = { onInc(dice) },
                        onDec   = { onDec(dice) },
                        modifier = Modifier.weight(1f),
                    )
                }
                // Relleno si fila incompleta
                repeat(4 - row.size) {
                    Spacer(Modifier.weight(1f))
                }
            }
        }
    }
}

@Composable
private fun DiceChip(
    dice: DiceType,
    count: Int,
    onInc: () -> Unit,
    onDec: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val active = count > 0
    val bg     = if (active) Dungeon else Crypt
    val col    = if (active) Aureate else Ash

    Column(
        modifier            = modifier
            .background(bg, RoundedCornerShape(10.dp))
            .border(1.dp, if (active) Gold else Iron, RoundedCornerShape(10.dp))
            .padding(4.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(2.dp),
    ) {
        Text(
            dice.label,
            color      = col,
            fontWeight = FontWeight.Bold,
            fontSize   = 12.sp,
            textAlign  = TextAlign.Center,
        )
        Row(
            horizontalArrangement = Arrangement.spacedBy(4.dp),
            verticalAlignment     = Alignment.CenterVertically,
        ) {
            TextButton(
                onClick      = onDec,
                contentPadding = PaddingValues(0.dp),
                modifier     = Modifier.size(24.dp),
            ) {
                Text("−", color = col, fontSize = 14.sp)
            }
            Text(
                "$count",
                color      = col,
                fontWeight = FontWeight.ExtraBold,
                fontSize   = 13.sp,
                modifier   = Modifier.widthIn(min = 16.dp),
                textAlign  = TextAlign.Center,
            )
            TextButton(
                onClick      = onInc,
                contentPadding = PaddingValues(0.dp),
                modifier     = Modifier.size(24.dp),
            ) {
                Text("+", color = col, fontSize = 14.sp)
            }
        }
    }
}

// ── ModifierRow ───────────────────────────────────────────────────────────────

@Composable
private fun ModifierRow(
    modifier: Int,
    onChange: (Int) -> Unit,
) {
    Row(
        verticalAlignment     = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        TextButton(onClick = { onChange(modifier - 1) }) {
            Text("−", color = Bone, fontSize = 20.sp, fontWeight = FontWeight.Bold)
        }
        Text(
            if (modifier >= 0) "+$modifier" else "$modifier",
            color      = if (modifier >= 0) Ichor else Ember,
            fontWeight = FontWeight.ExtraBold,
            fontSize   = 22.sp,
            modifier   = Modifier.widthIn(min = 48.dp),
            textAlign  = TextAlign.Center,
        )
        TextButton(onClick = { onChange(modifier + 1) }) {
            Text("+", color = Bone, fontSize = 20.sp, fontWeight = FontWeight.Bold)
        }
        Spacer(Modifier.weight(1f))
        TextButton(onClick = { onChange(0) }) {
            Text("Reset", color = Ash, fontSize = 12.sp)
        }
    }
}

// ── ModeRow ───────────────────────────────────────────────────────────────────

@Composable
private fun ModeRow(
    mode: RollMode,
    onChange: (RollMode) -> Unit,
) {
    val options = listOf(
        RollMode.Disadvantage to ("Desventaja" to Ember),
        RollMode.Normal       to ("Normal"     to Ash),
        RollMode.Advantage    to ("Ventaja"    to Ichor),
    )

    Row(
        modifier              = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        for ((m, pair) in options) {
            val (lbl, col) = pair
            val active = mode == m
            OutlinedButton(
                onClick  = { onChange(m) },
                modifier = Modifier.weight(1f),
                colors   = ButtonDefaults.outlinedButtonColors(
                    containerColor = if (active) Dungeon else Color.Transparent,
                    contentColor   = col,
                ),
                border  = androidx.compose.foundation.BorderStroke(
                    1.dp,
                    if (active) col else Iron,
                ),
                shape = RoundedCornerShape(8.dp),
            ) {
                Text(lbl, fontSize = 11.sp, fontWeight = if (active) FontWeight.Bold else FontWeight.Normal)
            }
        }
    }
}

// ── RollHistoryItem ───────────────────────────────────────────────────────────

@Composable
private fun RollHistoryItem(result: RollResult) {
    val totalColor = when {
        result.total >= 20 -> Aureate
        result.total <= 2  -> Ember
        else               -> Bone
    }

    val allValues = result.individualRolls.flatten()
    val modeTag   = when (result.request.mode) {
        RollMode.Advantage    -> " [V]"
        RollMode.Disadvantage -> " [D]"
        RollMode.Normal       -> ""
    }

    Card(
        colors   = CardDefaults.cardColors(containerColor = Crypt),
        modifier = Modifier.fillMaxWidth(),
        shape    = RoundedCornerShape(10.dp),
    ) {
        Row(
            modifier              = Modifier
                .fillMaxWidth()
                .padding(horizontal = 14.dp, vertical = 10.dp),
            verticalAlignment     = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Column(Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(2.dp)) {
                // Etiqueta y modo
                val label = result.request.label
                    ?: result.request.rolls.joinToString("+") { it.toString() }
                Text(
                    "$label$modeTag",
                    color      = Bone,
                    fontSize   = 13.sp,
                    fontWeight = FontWeight.SemiBold,
                )
                // Modificador y dados
                val diceStr = result.request.rolls.joinToString("+") { it.toString() }
                val modStr  = when {
                    result.request.modifier > 0 -> " +${result.request.modifier}"
                    result.request.modifier < 0 -> " ${result.request.modifier}"
                    else                        -> ""
                }
                Text("$diceStr$modStr", color = Ash, fontSize = 11.sp)
                // Valores individuales
                Text("[${allValues.joinToString(", ")}]", color = Iron, fontSize = 10.sp)
                // D20 descartado
                result.discardedD20?.let {
                    Text("descartado: $it", color = Ash, fontSize = 10.sp)
                }
            }
            Text(
                "${result.total}",
                color      = totalColor,
                fontSize   = 28.sp,
                fontWeight = FontWeight.ExtraBold,
            )
        }
    }
}
