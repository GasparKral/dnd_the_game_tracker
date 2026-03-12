package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonClassDiscriminator
import kotlinx.serialization.json.JsonElement

// ---------------------------------------------------------------------------
// Opciones de selección
// ---------------------------------------------------------------------------

@Serializable
data class SelectOption(
    val id: String,
    val label: String,
    val description: String? = null,
)

// ---------------------------------------------------------------------------
// ChoiceSchema — espeja el enum de Rust con serde tag = "kind"
// ---------------------------------------------------------------------------

@Serializable
@JsonClassDiscriminator("kind")
sealed class ChoiceSchema {
    abstract val id: String
    abstract val label: String

    @Serializable
    @SerialName("single_select")
    data class SingleSelect(
        override val id: String,
        override val label: String,
        val options: List<SelectOption>,
    ) : ChoiceSchema()

    @Serializable
    @SerialName("multi_select")
    data class MultiSelect(
        override val id: String,
        override val label: String,
        val min: Int,
        val max: Int,
        val options: List<SelectOption>,
    ) : ChoiceSchema()

    @Serializable
    @SerialName("point_buy")
    data class PointBuy(
        override val id: String,
        override val label: String,
        val points: Int,
        val fields: List<PointBuyField>,
    ) : ChoiceSchema()

    @Serializable
    @SerialName("text_input")
    data class TextInput(
        override val id: String,
        override val label: String,
        @SerialName("max_length") val maxLength: Int,
        val placeholder: String? = null,
    ) : ChoiceSchema()

    @Serializable
    @SerialName("number_input")
    data class NumberInput(
        override val id: String,
        override val label: String,
        val min: Int,
        val max: Int,
        val default: Int? = null,
    ) : ChoiceSchema()
}

@Serializable
data class PointBuyField(
    val id: String,
    val label: String,
    val min: Int,
    val max: Int,
)

// ---------------------------------------------------------------------------
// TraitDetail
// ---------------------------------------------------------------------------

@Serializable
data class TraitDetail(
    val name: String,
    val description: String,
)

// ---------------------------------------------------------------------------
// CatalogEntry
// ---------------------------------------------------------------------------

@Serializable
data class CatalogEntry(
    val id: String,
    val name: String,
    val source: String,
    val description: String? = null,
    val lore: String? = null,
    @SerialName("image_url") val imageUrl: String? = null,
    val choices: List<ChoiceSchema> = emptyList(),
    @SerialName("required_choices") val requiredChoices: List<String> = emptyList(),
    @SerialName("traits_preview") val traitsPreview: List<String> = emptyList(),
    @SerialName("traits_detail") val traitsDetail: List<TraitDetail> = emptyList(),
    @SerialName("speed_m") val speedM: Int? = null,
    val size: String? = null,
)

@Serializable
data class CatalogResponse(
    val entries: List<CatalogEntry>,
)
