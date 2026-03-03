package io.github.gasparkral.dnd.infra.dbstruct

import androidx.room.*
import io.github.gasparkral.dnd.infra.dao.CharacterDAO


@Database(entities = [Character::class], version = 1, exportSchema = false)
abstract class AppDatabase : RoomDatabase() {
    abstract fun characterDao(): CharacterDAO
}

@Entity
data class Character(
    @PrimaryKey val id: Int,
    @ColumnInfo(name = "character_name") val characterName: String,
)