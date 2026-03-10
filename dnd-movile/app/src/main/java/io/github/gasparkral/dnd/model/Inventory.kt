package io.github.gasparkral.dnd.model

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

// ---------------------------------------------------------------------------
// Sistema de bonificadores de estadística
// ---------------------------------------------------------------------------

@Serializable
enum class BonusStat {
    @SerialName("strength")          Strength,
    @SerialName("dexterity")         Dexterity,
    @SerialName("constitution")      Constitution,
    @SerialName("intelligence")      Intelligence,
    @SerialName("wisdom")            Wisdom,
    @SerialName("charisma")          Charisma,
    @SerialName("armor_class")       ArmorClass,
    @SerialName("max_hp")            MaxHp,
    @SerialName("speed")             Speed,
    @SerialName("attack_bonus")      AttackBonus,
    @SerialName("damage_bonus")      DamageBonus,
    @SerialName("saving_throw_str")  SavingThrowStr,
    @SerialName("saving_throw_dex")  SavingThrowDex,
    @SerialName("saving_throw_con")  SavingThrowCon,
    @SerialName("saving_throw_int")  SavingThrowInt,
    @SerialName("saving_throw_wis")  SavingThrowWis,
    @SerialName("saving_throw_cha")  SavingThrowCha;

    fun label() = when (this) {
        Strength        -> "FUE"
        Dexterity       -> "DES"
        Constitution    -> "CON"
        Intelligence    -> "INT"
        Wisdom          -> "SAB"
        Charisma        -> "CAR"
        ArmorClass      -> "CA"
        MaxHp           -> "PG máx"
        Speed           -> "Veloc."
        AttackBonus     -> "Ataque"
        DamageBonus     -> "Daño"
        SavingThrowStr  -> "Sal. FUE"
        SavingThrowDex  -> "Sal. DES"
        SavingThrowCon  -> "Sal. CON"
        SavingThrowInt  -> "Sal. INT"
        SavingThrowWis  -> "Sal. SAB"
        SavingThrowCha  -> "Sal. CAR"
    }

    /** Emoji decorativo para la UI */
    fun emoji() = when (this) {
        Strength        -> "💪"
        Dexterity       -> "🎯"
        Constitution    -> "❤️"
        Intelligence    -> "📚"
        Wisdom          -> "👁️"
        Charisma        -> "✨"
        ArmorClass      -> "🛡️"
        MaxHp           -> "❤️"
        Speed           -> "💨"
        AttackBonus     -> "⚔️"
        DamageBonus     -> "🔥"
        SavingThrowStr,
        SavingThrowDex,
        SavingThrowCon,
        SavingThrowInt,
        SavingThrowWis,
        SavingThrowCha  -> "🏙️"
    }

    /** True si este stat es un atributo base (FUE/DES/CON/INT/SAB/CAR) */
    fun isBaseAttribute() = this in setOf(
        Strength, Dexterity, Constitution, Intelligence, Wisdom, Charisma
    )
}

@Serializable
enum class BonusType {
    @SerialName("item")          Item,
    @SerialName("circumstance")  Circumstance,
    @SerialName("status")        Status,
    @SerialName("untyped")       Untyped;

    /**
     * Indica si este tipo se apila con otros del mismo tipo.
     * En D&D 5.5e, Status NO se apila — solo cuenta el mayor.
     * El resto (Item, Circumstance, Untyped) sí se apilan.
     */
    fun stacks() = this != Status
}

@Serializable
data class StatBonus(
    val stat: BonusStat,
    val value: Int,
    @SerialName("bonus_type") val bonusType: BonusType = BonusType.Untyped,
    val source: String = "",
)

// ---------------------------------------------------------------------------
// Stats efectivos calculados en cliente
// ---------------------------------------------------------------------------

/**
 * Resultado de aplicar todos los bonificadores de ítems equipados
 * sobre los atributos base del personaje.
 *
 * Es la fuente de verdad para mostrar stats en Dashboard, Inventario y Combate.
 */
data class EffectiveStats(
    /** Atributos base del personaje (sin modificadores de equipo) */
    val base: AttributesDto,
    /** Suma neta de bonificadores por atributo base, respetando reglas de apilamiento */
    val attributeBonuses: Map<BonusStat, Int>,
    /** Bonus neto a CA (suma de todos los ítems equipados) */
    val armorClassBonus: Int,
    /** Bonus neto a PG máximos */
    val maxHpBonus: Int,
    /** Bonus neto a velocidad en pies */
    val speedBonus: Int,
    /** Bonus neto a ataque */
    val attackBonus: Int,
    /** Bonus neto a daño */
    val damageBonus: Int,
    /** Bonus a tiradas de salvación por atributo */
    val savingThrowBonuses: Map<BonusStat, Int>,
    /** Lista plana de todos los bonus activos con su fuente — para mostrar en tooltips */
    val activeBonuses: List<ActiveBonus>,
) {
    /** Atributo efectivo final: base + bonus de equipo */
    fun effectiveStrength()     = base.strength     + (attributeBonuses[BonusStat.Strength]     ?: 0)
    fun effectiveDexterity()    = base.dexterity    + (attributeBonuses[BonusStat.Dexterity]    ?: 0)
    fun effectiveConstitution() = base.constitution + (attributeBonuses[BonusStat.Constitution] ?: 0)
    fun effectiveIntelligence() = base.intelligence + (attributeBonuses[BonusStat.Intelligence] ?: 0)
    fun effectiveWisdom()       = base.wisdom       + (attributeBonuses[BonusStat.Wisdom]       ?: 0)
    fun effectiveCharisma()     = base.charisma     + (attributeBonuses[BonusStat.Charisma]     ?: 0)

    fun hasAnyBonus() = activeBonuses.isNotEmpty()
}

/** Bonus activo individual — para mostrar en tooltips y panel de equipo */
data class ActiveBonus(
    val stat: BonusStat,
    val value: Int,
    val bonusType: BonusType,
    val source: String,  // nombre del ítem que lo provee
)

// ---------------------------------------------------------------------------
// Cálculo de stats efectivos
// ---------------------------------------------------------------------------

/**
 * Calcula los [EffectiveStats] de un personaje aplicando todos los
 * [StatBonus] de los ítems que están equipados actualmente.
 *
 * Reglas de apilamiento (D&D 5.5e):
 * - BonusType.Item, Circumstance, Untyped: se suman todos
 * - BonusType.Status: solo cuenta el mayor valor para cada stat
 *
 * @param base      Atributos base del personaje
 * @param inventory Lista completa del inventario (se filtran los equipados)
 */
fun calculateEffectiveStats(base: AttributesDto, inventory: List<InventoryItem>): EffectiveStats {
    val equipped = inventory.filter { it.equipped && it.statBonuses.isNotEmpty() }

    // Construir lista plana de todos los bonuses activos
    val allBonuses: List<ActiveBonus> = equipped.flatMap { item ->
        item.statBonuses.map { bonus ->
            ActiveBonus(
                stat      = bonus.stat,
                value     = bonus.value,
                bonusType = bonus.bonusType,
                source    = item.name,
            )
        }
    }

    /**
     * Aplica las reglas de apilamiento para un [BonusStat] concreto.
     * Devuelve la suma neta de todos los bonuses que aplican a ese stat.
     */
    fun netBonus(stat: BonusStat): Int {
        val forStat = allBonuses.filter { it.stat == stat }
        if (forStat.isEmpty()) return 0

        // Separar por tipo y aplicar reglas
        val statusBonuses      = forStat.filter { it.bonusType == BonusType.Status }
        val stackingBonuses    = forStat.filter { it.bonusType != BonusType.Status }

        // Status: solo el mayor (positivo) o el menor (negativo, debuffs)
        val statusNet = when {
            statusBonuses.isEmpty() -> 0
            statusBonuses.any { it.value > 0 } ->
                statusBonuses.filter { it.value > 0 }.maxOf { it.value }
            else ->
                statusBonuses.minOf { it.value }  // debuff negativo
        }

        // Item / Circumstance / Untyped: se suman todos
        val stackingNet = stackingBonuses.sumOf { it.value }

        return statusNet + stackingNet
    }

    // Calcular bonuses por categoría
    val attrStats = listOf(
        BonusStat.Strength, BonusStat.Dexterity, BonusStat.Constitution,
        BonusStat.Intelligence, BonusStat.Wisdom, BonusStat.Charisma,
    )
    val savingStats = listOf(
        BonusStat.SavingThrowStr, BonusStat.SavingThrowDex, BonusStat.SavingThrowCon,
        BonusStat.SavingThrowInt, BonusStat.SavingThrowWis, BonusStat.SavingThrowCha,
    )

    val attributeBonuses  = attrStats.associateWith { netBonus(it) }.filterValues { it != 0 }
    val savingBonuses     = savingStats.associateWith { netBonus(it) }.filterValues { it != 0 }

    return EffectiveStats(
        base               = base,
        attributeBonuses   = attributeBonuses,
        armorClassBonus    = netBonus(BonusStat.ArmorClass),
        maxHpBonus         = netBonus(BonusStat.MaxHp),
        speedBonus         = netBonus(BonusStat.Speed),
        attackBonus        = netBonus(BonusStat.AttackBonus),
        damageBonus        = netBonus(BonusStat.DamageBonus),
        savingThrowBonuses = savingBonuses,
        activeBonuses      = allBonuses,
    )
}

// ---------------------------------------------------------------------------
// Extensiones de utilidad
// ---------------------------------------------------------------------------

/** Modificador D&D (score - 10) / 2, redondeando hacia abajo */
fun attributeModifier(score: Int): Int = Math.floorDiv(score - 10, 2)

/** Formatea un modificador como "+N" o "−N" */
fun formatModifier(mod: Int): String = if (mod >= 0) "+$mod" else "−${-mod}"

// ---------------------------------------------------------------------------

@Serializable
enum class ItemCategory {
    @SerialName("weapon")     Weapon,
    @SerialName("armour")     Armour,
    @SerialName("consumable") Consumable,
    @SerialName("tool")       Tool,
    @SerialName("treasure")   Treasure,
    @SerialName("accessory")  Accessory,
    @SerialName("misc")       Misc;

    fun label() = when (this) {
        Weapon    -> "Arma"
        Armour    -> "Armadura"
        Consumable -> "Consumible"
        Tool      -> "Herramienta"
        Treasure  -> "Tesoro"
        Accessory -> "Accesorio"
        Misc      -> "Misc"
    }

    fun emoji() = when (this) {
        Weapon    -> "⚔️"
        Armour    -> "🛡️"
        Consumable -> "🧪"
        Tool      -> "🔧"
        Treasure  -> "💎"
        Accessory -> "💍"
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
    @SerialName("accessory_type") val accessoryType: String? = null,
    @SerialName("stat_bonuses") val statBonuses: List<StatBonus> = emptyList(),
    val notes: String = "",
) {
    /** True si este ítem aporta bonificadores cuando está equipado */
    val hasStatBonuses: Boolean get() = statBonuses.isNotEmpty()

    /** True si este ítem es equipable por su categoría */
    val isEquippable: Boolean get() = category in setOf(
        ItemCategory.Weapon, ItemCategory.Armour, ItemCategory.Accessory
    )
}

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
    @SerialName("accessory_type") val accessoryType: String? = null,
    @SerialName("stat_bonuses") val statBonuses: List<StatBonus> = emptyList(),
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
