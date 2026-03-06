package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
enum class ItemCategory {
    @SerialName("weapon")     Weapon,
    @SerialName("armour")     Armour,
    @SerialName("consumable") Consumable,
    @SerialName("tool")       Tool,
    @SerialName("treasure")   Treasure,
    @SerialName("misc")       Misc;

    fun label() = when (this) {
        Weapon    -> "Arma"
        Armour    -> "Armadura"
        Consumable -> "Consumible"
        Tool      -> "Herramienta"
        Treasure  -> "Tesoro"
        Misc      -> "Misc"
    }

    fun emoji() = when (this) {
        Weapon    -> "⚔️"
        Armour    -> "🛡️"
        Consumable -> "🧪"
        Tool      -> "🔧"
        Treasure  -> "💎"
        Misc      -> "📦"
    }
}

@Serializable
data class InventoryItem(
    val id: String,
    val name: String,
    val category: ItemCategory,
    @SerialName("description") val description: String = "",
    val quantity: Int,
    val weight: Float? = null,
    val equipped: Boolean = false,
    val notes: String = "",
)

@Serializable
data class Currency(
    val copper: Int = 0,
    val silver: Int = 0,
    val electrum: Int = 0,
    val gold: Int = 0,
    val platinum: Int = 0,
)

@Serializable
data class InventoryResponse(
    val items: List<InventoryItem>,
    val currency: Currency,
    @SerialName("total_weight") val totalWeight: Float,
)

@Serializable
data class AddItemRequest(
    val name: String,
    val category: ItemCategory,
    val description: String = "",
    val quantity: Int,
    val weight: Float? = null,
    val notes: String = "",
)

@Serializable
data class UpdateItemRequest(
    val quantity: Int? = null,
    val equipped: Boolean? = null,
    val notes: String? = null,
)

@Serializable
data class UpdateCurrencyRequest(
    val copper: Int? = null,
    val silver: Int? = null,
    val electrum: Int? = null,
    val gold: Int? = null,
    val platinum: Int? = null,
)
