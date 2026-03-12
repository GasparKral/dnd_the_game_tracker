@file:OptIn(ExperimentalUuidApi::class, ExperimentalSerializationApi::class, ExperimentalSerializationApi::class)

package io.github.gasparkral.dnd.infra

import io.github.gasparkral.dnd.model.RollResult
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonClassDiscriminator
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

/* ---------- UUID SERIALIZER ---------- */

object UuidSerializer : KSerializer<Uuid> {
    override val descriptor: SerialDescriptor =
        PrimitiveSerialDescriptor("Uuid", PrimitiveKind.STRING)

    override fun serialize(encoder: Encoder, value: Uuid) {
        encoder.encodeString(value.toString())
    }

    override fun deserialize(decoder: Decoder): Uuid {
        return Uuid.parse(decoder.decodeString())
    }
}



/* ---------- CLIENT MESSAGES ---------- */

@Serializable
@JsonClassDiscriminator("type")
sealed class ClientMessage {

    @Serializable
    @SerialName("join")
    data class Join(
        @SerialName("player_name")
        val playerName: String,
        @SerialName("character_name")
        val characterMame: String
    ) : ClientMessage()

    @Serializable
    @SerialName("roll_dice")
    data class RollDice(
        @SerialName("roll_result")
        val rollResult: RollResult
    ) : ClientMessage()

    @Serializable
    @SerialName("request_sync")
    object RequestSync : ClientMessage()

    @Serializable
    @SerialName("inventory_updated")
    data class InventoryUpdated(
        @Serializable(with = UuidSerializer::class)
        @SerialName("character_id")
        val characterId: Uuid
    ) : ClientMessage()
}

/* ---------- SERVER MESSAGES ---------- */

@Serializable
@JsonClassDiscriminator("type")
sealed class ServerMessage {

    @Serializable
    @SerialName("welcome")
    data class Welcome(
        @Serializable(with = UuidSerializer::class)
        val playerId: Uuid,
        @SerialName("player_name")
        val playerName: String
    ) : ServerMessage()

    @Serializable
    @SerialName("combat_started")
    object CombatStarted : ServerMessage()

    @Serializable
    @SerialName("combat_ended")
    object CombatEnded : ServerMessage()

    @Serializable
    @SerialName("initiative_update")
    data class InitiativeUpdate(
        val order: List<InitiativeEntry>
    ) : ServerMessage()

    @Serializable
    @SerialName("hp_update")
    data class HpUpdate(
        @Serializable(with = UuidSerializer::class)
        @SerialName("character_id")
        val characterId: Uuid,
        val current: Int,
        val max: Int
    ) : ServerMessage()

    @Serializable
    @SerialName("condition_update")
    data class ConditionUpdate(
        @Serializable(with = UuidSerializer::class)
        @SerialName("character_id")
        val characterId: Uuid,
        val conditions: List<String>
    ) : ServerMessage()

    @Serializable
    @SerialName("dice_roll")
    data class DiceRoll(
        @Serializable(with = UuidSerializer::class)
        @SerialName("player_id")
        val playerId: Uuid,
        @SerialName("roll_result")
        val rollResult: RollResult
    ) : ServerMessage()

    @Serializable
    @SerialName("dm_dice_roll")
    data class DmDiceRoll(
        @SerialName("roll_result")
        val rollResult: RollResult
    ) : ServerMessage()

    @Serializable
    @SerialName("private_message")
    data class PrivateMessage(
        @Serializable(with = UuidSerializer::class)
        @SerialName("target_id")
        val targetId: Uuid,
        val text: String
    ) : ServerMessage()

    @Serializable
    @SerialName("announcement")
    data class Announcement(
        val text: String
    ) : ServerMessage()

    @Serializable
    @SerialName("inventory_changed")
    data class InventoryChanged(
        @Serializable(with = UuidSerializer::class)
        @SerialName("character_id")
        val characterId: Uuid
    ) : ServerMessage()
}

/* ---------- INITIATIVE ENTRY ---------- */

@Serializable
data class InitiativeEntry(
    @Serializable(with = UuidSerializer::class)
    val characterId: Uuid,
    val name: String,
    val initiative: Int,
    @SerialName("is_player")
    val isPlayer: Boolean
)