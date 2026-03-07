package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

// ---------------------------------------------------------------------------
// Escuela de magia
// ---------------------------------------------------------------------------

@Serializable
enum class SpellSchool {
    @SerialName("abjuration")    Abjuration,
    @SerialName("conjuration")   Conjuration,
    @SerialName("divination")    Divination,
    @SerialName("enchantment")   Enchantment,
    @SerialName("evocation")     Evocation,
    @SerialName("illusion")      Illusion,
    @SerialName("necromancy")    Necromancy,
    @SerialName("transmutation") Transmutation,
    @SerialName("unknown")       Unknown;

    fun label() = when (this) {
        Abjuration    -> "Abjuración"
        Conjuration   -> "Conjuración"
        Divination    -> "Adivinación"
        Enchantment   -> "Encantamiento"
        Evocation     -> "Evocación"
        Illusion      -> "Ilusión"
        Necromancy    -> "Nigromancia"
        Transmutation -> "Transmutación"
        Unknown       -> "Desconocida"
    }

    fun emoji() = when (this) {
        Abjuration    -> "🛡"
        Conjuration   -> "✨"
        Divination    -> "🔮"
        Enchantment   -> "💫"
        Evocation     -> "🔥"
        Illusion      -> "🌀"
        Necromancy    -> "💀"
        Transmutation -> "⚗️"
        Unknown       -> "❓"
    }

    fun color() = when (this) {
        Abjuration    -> 0xFF3B82F6L // azul
        Conjuration   -> 0xFF8B5CF6L // violeta
        Divination    -> 0xFF06B6D4L // cyan
        Enchantment   -> 0xFFEC4899L // rosa
        Evocation     -> 0xFFEF4444L // rojo
        Illusion      -> 0xFF6366F1L // índigo
        Necromancy    -> 0xFF6B7280L // gris
        Transmutation -> 0xFF10B981L // verde
        Unknown       -> 0xFF78716CL // stone
    }
}

// ---------------------------------------------------------------------------
// Componentes de hechizo
// ---------------------------------------------------------------------------

@Serializable
data class SpellComponents(
    val verbal: Boolean = false,
    val somatic: Boolean = false,
    val material: Boolean = false,
    @SerialName("material_component") val materialComponent: String = "",
)

// ---------------------------------------------------------------------------
// Hechizo
// ---------------------------------------------------------------------------

@Serializable
data class Spell(
    val id: String,
    val name: String,
    val level: Int,            // 0 = truco
    val school: SpellSchool,
    @SerialName("casting_time") val castingTime: String = "",
    val range: String = "",
    val duration: String = "",
    val components: SpellComponents = SpellComponents(),
    val description: String = "",
    val damage: String? = null,
    @SerialName("saving_throw") val savingThrow: String? = null,
    val notes: String = "",
    val concentration: Boolean = false,
    val ritual: Boolean = false,
    val prepared: Boolean = false,
)

// ---------------------------------------------------------------------------
// Espacio de hechizo por nivel
// ---------------------------------------------------------------------------

@Serializable
data class SpellSlotLevel(
    val level: Int,     // 1-9
    val total: Int,
    val remaining: Int,
)

// ---------------------------------------------------------------------------
// Respuesta del servidor
// ---------------------------------------------------------------------------

@Serializable
data class SpellsResponse(
    @SerialName("spell_slots")    val spellSlots: List<SpellSlotLevel>,
    @SerialName("known_spells")   val knownSpells: List<Spell>,
    @SerialName("prepared_spells") val preparedSpells: List<Spell>,
)

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

@Serializable
data class AddSpellRequest(
    val name: String,
    val level: Int,
    val school: SpellSchool,
    @SerialName("casting_time") val castingTime: String = "",
    val range: String = "",
    val duration: String = "",
    val components: SpellComponents = SpellComponents(),
    val description: String = "",
    val damage: String? = null,
    @SerialName("saving_throw") val savingThrow: String? = null,
    val notes: String = "",
    val concentration: Boolean = false,
    val ritual: Boolean = false,
    val prepared: Boolean = false,
)

@Serializable
data class UpdateSpellSlotsRequest(
    val slots: List<SpellSlotLevel>,
)

// Wrapper para deserializar { "prepared": true } del endpoint toggle_prepared
@Serializable
data class TogglePreparedResponse(
    val prepared: Boolean,
)
