package io.github.gasparkral.dnd.infra.dao

import androidx.room.*
import io.github.gasparkral.dnd.infra.dbstruct.Character

@Dao
interface CharacterDAO {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertCharacter(vararg character: Character)

    @Delete
    suspend fun deleteCharacter(vararg character: Character)

    @Query("SELECT * FROM character")
    suspend fun findAll(): Array<Character>

    @Query("SELECT * from character WHERE id = :id")
    suspend fun findById(id: Int): Character?
}