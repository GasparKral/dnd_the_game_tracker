package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

// ---------------------------------------------------------------------------
// Personaje guardado — snapshot finalizado que devuelve el servidor
// ---------------------------------------------------------------------------

@Serializable
data class SavedCharacter(
    val id: String,
    @SerialName("player_name")   val playerName: String,
    val name: String,
    @SerialName("race_id")       val raceId: String,
    @SerialName("class_id")      val classId: String,
    @SerialName("background_id") val backgroundId: String,
    val attributes: AttributesDto,
    @SerialName("feat_ids")      val featIds: List<String> = emptyList(),
    val level: Int,
    @SerialName("current_hp")    val currentHp: Int,
    @SerialName("max_hp")        val maxHp: Int,
    val xp: Long,
    val notes: String = "",
    @SerialName("updated_at")    val updatedAt: String,
)

@Serializable
data class CharactersResponse(
    val characters: List<SavedCharacter>,
)

// ---------------------------------------------------------------------------
// Campaña
// ---------------------------------------------------------------------------

@Serializable
data class CampaignSummary(
    val name: String,
    val description: String,
    @SerialName("vault_path")       val vaultPath: String? = null,
    @SerialName("character_count")  val characterCount: Int,
    @SerialName("updated_at")       val updatedAt: String,
)

@Serializable
data class CreateCampaignRequest(
    val name: String,
    val description: String = "",
)
