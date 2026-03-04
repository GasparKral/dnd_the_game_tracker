package io.github.gasparkral.dnd.infra.dao

import androidx.room.Dao
import androidx.room.Delete
import androidx.room.Insert
import androidx.room.OnConflictStrategy.Companion.REPLACE
import androidx.room.Query
import io.github.gasparkral.dnd.infra.dbstruct.Character

@Dao
interface CharacterDAO {
    @Insert(onConflict = REPLACE)
    suspend fun insertCharacter(character: Character): Long

    @Delete
    suspend fun deleteCharacter(character: Character)

    @Query("SELECT * FROM character")
    suspend fun findAll(): Array<Character>

    @Query("SELECT * from character WHERE id = :id")
    suspend fun findById(id: Long): Character?
}