package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

// ─── Espejo de shared::models::dice ──────────────────────────────────────────
// Estos tipos reflejan exactamente las structs Rust del crate shared para que
// la serialización JSON sea compatible con el servidor Axum.

@Serializable
enum class DiceType {
    @SerialName("D2")   D2,
    @SerialName("D4")   D4,
    @SerialName("D6")   D6,
    @SerialName("D8")   D8,
    @SerialName("D10")  D10,
    @SerialName("D12")  D12,
    @SerialName("D20")  D20,
    @SerialName("D100") D100;

    val faces: Int get() = when (this) {
        D2   -> 2
        D4   -> 4
        D6   -> 6
        D8   -> 8
        D10  -> 10
        D12  -> 12
        D20  -> 20
        D100 -> 100
    }

    val label: String get() = when (this) {
        D100 -> "d%"
        else -> "d${faces}"
    }
}

@Serializable
data class DiceRollDto(
    val count: Int,
    val dice: DiceType,
) {
    override fun toString(): String = "${count}d${dice.faces}"
}

@Serializable
enum class RollMode {
    @SerialName("normal")       Normal,
    @SerialName("advantage")    Advantage,
    @SerialName("disadvantage") Disadvantage,
}

@Serializable
data class RollRequest(
    val rolls: List<DiceRollDto>,
    val modifier: Int,
    val mode: RollMode,
    val label: String? = null,
)

@Serializable
data class RollResult(
    val request: RollRequest,
    @SerialName("individual_rolls")
    val individualRolls: List<List<Int>>,
    @SerialName("discarded_d20")
    val discardedD20: Int? = null,
    val total: Int,
    val timestamp: Long,
) {
    /** Texto corto para el feed: "Ataque +3 → 18" */
    fun toFeedLine(): String {
        val lbl = request.label ?: request.rolls.joinToString("+") { it.toString() }
        val mod = when {
            request.modifier > 0 -> " +${request.modifier}"
            request.modifier < 0 -> " ${request.modifier}"
            else -> ""
        }
        val modeBadge = when (request.mode) {
            RollMode.Advantage    -> " [V]"
            RollMode.Disadvantage -> " [D]"
            RollMode.Normal       -> ""
        }
        return "$lbl$mod$modeBadge → $total"
    }
}
