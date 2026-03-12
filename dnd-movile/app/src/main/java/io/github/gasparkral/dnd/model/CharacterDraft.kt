package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement

// ---------------------------------------------------------------------------
// Atributos
// ---------------------------------------------------------------------------

@Serializable
data class AttributesDto(
    val strength: Int,
    val dexterity: Int,
    val constitution: Int,
    val intelligence: Int,
    val wisdom: Int,
    val charisma: Int,
) {
    companion object {
        val DEFAULT = AttributesDto(8, 8, 8, 8, 8, 8)

        private val costTable = mapOf(
            8 to 0, 9 to 1, 10 to 2, 11 to 3,
            12 to 4, 13 to 5, 14 to 7, 15 to 9
        )

        fun costFor(score: Int): Int = costTable[score] ?: Int.MAX_VALUE
    }

    fun pointBuyCost(): Int = listOf(strength, dexterity, constitution, intelligence, wisdom, charisma)
        .sumOf { costFor(it) }

    fun withField(field: String, value: Int) = when (field) {
        "str" -> copy(strength = value)
        "dex" -> copy(dexterity = value)
        "con" -> copy(constitution = value)
        "int" -> copy(intelligence = value)
        "wis" -> copy(wisdom = value)
        "cha" -> copy(charisma = value)
        else  -> this
    }

    fun getField(field: String) = when (field) {
        "str" -> strength
        "dex" -> dexterity
        "con" -> constitution
        "int" -> intelligence
        "wis" -> wisdom
        "cha" -> charisma
        else  -> 8
    }
}

// ---------------------------------------------------------------------------
// Paso del wizard — espeja CreationStep de Rust
// ---------------------------------------------------------------------------

@Serializable
enum class CreationStep {
    @SerialName("name")       Name,
    @SerialName("race")       Race,
    @SerialName("class")      Class,
    @SerialName("attributes") Attributes,
    @SerialName("background") Background,
    @SerialName("feats")      Feats,
    @SerialName("review")     Review,
    @SerialName("complete")   Complete;

    fun isComplete() = this == Complete
}

// ---------------------------------------------------------------------------
// CharacterDraft — estado del personaje en construcción
// ---------------------------------------------------------------------------

@Serializable
data class CharacterDraft(
    @SerialName("draft_id")      val draftId: String? = null,
    val step: CreationStep = CreationStep.Name,
    val name: String? = null,
    @SerialName("race_id")       val raceId: String? = null,
    @SerialName("class_id")      val classId: String? = null,
    @SerialName("background_id") val backgroundId: String? = null,
    val attributes: AttributesDto? = null,
    @SerialName("feat_ids")      val featIds: List<String> = emptyList(),
    val choices: Map<String, JsonElement> = emptyMap(),
)

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

@Serializable
data class CreateDraftRequest(
    @SerialName("player_name") val playerName: String,
)

@Serializable
data class UpdateDraftRequest(
    @SerialName("draft_id")      val draftId: String,
    val step: CreationStep,
    @SerialName("player_name")   val playerName: String? = null,
    val name: String? = null,
    @SerialName("race_id")       val raceId: String? = null,
    @SerialName("class_id")      val classId: String? = null,
    @SerialName("background_id") val backgroundId: String? = null,
    val attributes: AttributesDto? = null,
    @SerialName("feat_ids")      val featIds: List<String> = emptyList(),
    val choices: Map<String, JsonElement> = emptyMap(),
)

@Serializable
data class SetDraftStepRequest(
    val step: CreationStep,
)

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

@Serializable
data class DraftResponse(
    val draft: CharacterDraft,
    val errors: List<String> = emptyList(),
    val finalized: Boolean = false,
)

@Serializable
data class DraftStatusResponse(
    val draft: CharacterDraft,
    @SerialName("is_complete") val isComplete: Boolean,
)
