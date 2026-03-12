package io.github.gasparkral.dnd.ui.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.github.gasparkral.dnd.infra.ClientMessage
import io.github.gasparkral.dnd.infra.SocketManager
import io.github.gasparkral.dnd.model.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

// ─── Estado de la UI ──────────────────────────────────────────────────────────

data class DiceRollerUiState(
    /** Cantidad de cada dado seleccionado: clave = DiceType */
    val counts: Map<DiceType, Int> = DiceType.entries.associateWith { 0 },
    val modifier: Int = 0,
    val mode: RollMode = RollMode.Normal,
    val label: String = "",
    val rolling: Boolean = false,
    val errorMsg: String? = null,
    /** Historial local de las últimas 10 tiradas */
    val history: List<RollResult> = emptyList(),
) {
    val isEmpty: Boolean get() = counts.values.all { it == 0 }
    val hasD20: Boolean get() = (counts[DiceType.D20] ?: 0) > 0

    fun toRequest(): RollRequest = RollRequest(
        rolls = counts
            .filter { it.value > 0 }
            .map { (dice, count) -> DiceRollDto(count = count, dice = dice) },
        modifier = modifier,
        mode = if (hasD20) mode else RollMode.Normal,
        label = label.trim().ifEmpty { null },
    )
}

// ─── ViewModel ────────────────────────────────────────────────────────────────

class DiceRollerViewModel(val socketManager: SocketManager) : ViewModel() {

    private val _uiState = MutableStateFlow(DiceRollerUiState())
    val uiState: StateFlow<DiceRollerUiState> = _uiState.asStateFlow()

    // ── Acciones del usuario ─────────────────────────────────────────────────

    fun incrementDice(dice: DiceType) {
        _uiState.update { s ->
            val cur = s.counts[dice] ?: 0
            if (cur >= 10) return
            s.copy(counts = s.counts + (dice to cur + 1))
        }
    }

    fun decrementDice(dice: DiceType) {
        _uiState.update { s ->
            val cur = s.counts[dice] ?: 0
            if (cur <= 0) return
            s.copy(counts = s.counts + (dice to cur - 1))
        }
    }

    fun setModifier(value: Int) {
        _uiState.update { it.copy(modifier = value) }
    }

    fun setMode(mode: RollMode) {
        _uiState.update { it.copy(mode = mode) }
    }

    fun setLabel(label: String) {
        _uiState.update { it.copy(label = label) }
    }

    // ── Tirar ─────────────────────────────────────────────────────────────────

    fun roll() {
        val state = _uiState.value
        if (state.isEmpty) {
            _uiState.update { it.copy(errorMsg = "Selecciona al menos un dado") }
            return
        }
        _uiState.update { it.copy(rolling = true, errorMsg = null) }

        viewModelScope.launch {
            val request = state.toRequest()
            val result = executeRollLocally(request)

            // Enviar al servidor vía WebSocket para que el DM y el resto lo vean
            socketManager.send(ClientMessage.RollDice(rollResult = result))

            _uiState.update { s ->
                val newHistory = (listOf(result) + s.history).take(10)
                s.copy(rolling = false, history = newHistory)
            }
        }
    }

    // ── Lógica de tirada local ───────────────────────────────────────────────
    // Espeja la lógica de RollRequest::execute() en Rust.

    private fun executeRollLocally(request: RollRequest): RollResult {
        val rng = java.util.Random()

        fun rollDie(faces: Int): Int = rng.nextInt(faces) + 1

        val individualRolls: MutableList<List<Int>> = mutableListOf()
        var discardedD20: Int? = null

        for (dr in request.rolls) {
            val values = (1..dr.count).map { rollDie(dr.dice.faces) }.toMutableList()

            // Ventaja/desventaja sobre el primer d20
            if (dr.dice == DiceType.D20 && request.mode != RollMode.Normal && discardedD20 == null) {
                val extra = rollDie(20)
                val current = values[0]
                val (keep, discard) = when (request.mode) {
                    RollMode.Advantage -> if (extra >= current) extra to current else current to extra
                    RollMode.Disadvantage -> if (extra <= current) extra to current else current to extra
                    RollMode.Normal -> current to extra
                }
                values[0] = keep
                discardedD20 = discard
            }
            individualRolls.add(values)
        }

        val sum = individualRolls.flatten().sum()
        val total = sum + request.modifier
        val ts = System.currentTimeMillis() / 1000L

        return RollResult(
            request = request,
            individualRolls = individualRolls,
            discardedD20 = discardedD20,
            total = total,
            timestamp = ts,
        )
    }
}
