package io.github.gasparkral.dnd.infra.service

import io.github.gasparkral.dnd.infra.dao.CharacterDAO
import io.github.gasparkral.dnd.infra.dbstruct.Character
import io.github.gasparkral.dnd.model.exception.EntityNotFoundException
import io.github.gasparkral.dnd.utils.Result


class CharacterService(
    private val characterRepository: CharacterDAO
) {

    suspend fun getAllCharacter(): Array<Character> {
        return characterRepository.findAll()
    }

    suspend fun getCharacterById(id: Int): Result<Character, EntityNotFoundException> {
        val character = characterRepository.findById(id)
        return if (character == null) {
            Result.Err(EntityNotFoundException())
        } else {
            Result.Ok(character)
        }
    }
}

