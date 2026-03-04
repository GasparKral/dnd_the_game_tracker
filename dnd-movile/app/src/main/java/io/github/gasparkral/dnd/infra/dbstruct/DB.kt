package io.github.gasparkral.dnd.infra.dbstruct

import androidx.room.*
import io.github.gasparkral.dnd.infra.dao.CharacterDAO


@Database(entities = [Character::class], version = 1, exportSchema = false)
abstract class AppDatabase : RoomDatabase() {
    abstract fun characterDao(): CharacterDAO
}

@Entity
data class Character(
    @PrimaryKey(autoGenerate = true) var id: Long = 0L,
    @ColumnInfo(name = "character_level") val level: Int,
    @ColumnInfo(name = "character_experience") val exp: Int,
    @ColumnInfo(name = "character_name") val characterName: String,
    @ColumnInfo(name = "background") val characterBackground: String,
    @ColumnInfo(name = "class") val characterClass: String,
    @ColumnInfo(name = "raze") val characterRaze: String,
    @ColumnInfo(name = "strength") val str: Int,
    @ColumnInfo(name = "dexterity") val dex: Int,
    @ColumnInfo(name = "intelligence") val int: Int,
    @ColumnInfo(name = "constitution") val con: Int,
    @ColumnInfo(name = "charism") val cha: Int,
    @ColumnInfo(name = "wisdom") val wis: Int,
)

@Entity(primaryKeys = ["character_id", "item_id"])
data class Inventory(
    @ColumnInfo(name = "character_id") val characterId: Long,
    @ColumnInfo(name = "item_id") val itemId: Long,
    @ColumnInfo(name = "amount") val amount: Double
)

@Entity
data class Item(
    @PrimaryKey val id: Long,
)