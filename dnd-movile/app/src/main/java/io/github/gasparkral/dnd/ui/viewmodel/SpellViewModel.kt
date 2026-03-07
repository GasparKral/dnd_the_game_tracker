package io.github.gasparkral.dnd.ui.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.model.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

// ---------------------------------------------------------------------------
// Estado UI
// ---------------------------------------------------------------------------

data class SpellsUiState(
    val isLoading: Boolean = true,
    val error: String? = null,
    // Datos
    val slots: List<SpellSlotLevel> = emptyList(),
    val knownSpells: List<Spell> = emptyList(),
    val preparedSpells: List<Spell> = emptyList(),
    // Diálogos
    val showAddDialog: Boolean = false,
    val isSaving: Boolean = false,
)

// ---------------------------------------------------------------------------
// ViewModel
// ---------------------------------------------------------------------------

class SpellViewModel(
    private val characterId: String,
    private val repo: DraftRepository,
) : ViewModel() {

    private val _state = MutableStateFlow(SpellsUiState())
    val state: StateFlow<SpellsUiState> = _state.asStateFlow()

    init { load() }

    // ── Carga inicial ────────────────────────────────────────────────────────

    fun load() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            repo.getSpells(characterId).fold(
                onOk = { resp ->
                    _state.update {
                        it.copy(
                            isLoading = false,
                            slots = resp.spellSlots,
                            knownSpells = resp.knownSpells,
                            preparedSpells = resp.preparedSpells,
                        )
                    }
                },
                onErr = { _state.update { it.copy(isLoading = false, error = "Error al cargar hechizos") } }
            )
        }
    }

    // ── Diálogo añadir ───────────────────────────────────────────────────────

    fun openAddDialog()  = _state.update { it.copy(showAddDialog = true) }
    fun closeAddDialog() = _state.update { it.copy(showAddDialog = false) }

    fun addSpell(req: AddSpellRequest) {
        viewModelScope.launch {
            _state.update { it.copy(isSaving = true) }
            repo.addSpell(characterId, req).fold(
                onOk = { spell ->
                    _state.update { s ->
                        val newKnown = s.knownSpells + spell
                        val newPrep  = if (spell.prepared) s.preparedSpells + spell else s.preparedSpells
                        s.copy(
                            isSaving = false,
                            showAddDialog = false,
                            knownSpells = newKnown,
                            preparedSpells = newPrep,
                        )
                    }
                },
                onErr = { _state.update { it.copy(isSaving = false) } }
            )
        }
    }

    // ── Toggle preparado ─────────────────────────────────────────────────────

    fun togglePrepared(spell: Spell) {
        viewModelScope.launch {
            repo.togglePrepared(characterId, spell.id).fold(
                onOk = { response ->
                    val nowPrepared = response.prepared
                    _state.update { s ->
                        val updatedKnown = s.knownSpells.map {
                            if (it.id == spell.id) it.copy(prepared = nowPrepared) else it
                        }
                        val updatedPrep = if (nowPrepared) {
                            if (s.preparedSpells.none { it.id == spell.id })
                                s.preparedSpells + spell.copy(prepared = true)
                            else s.preparedSpells
                        } else {
                            s.preparedSpells.filter { it.id != spell.id }
                        }
                        s.copy(knownSpells = updatedKnown, preparedSpells = updatedPrep)
                    }
                },
                onErr = {}
            )
        }
    }

    // ── Eliminar hechizo ─────────────────────────────────────────────────────

    fun removeSpell(spell: Spell) {
        viewModelScope.launch {
            repo.removeSpell(characterId, spell.id).fold(
                onOk = {
                    _state.update { s ->
                        s.copy(
                            knownSpells   = s.knownSpells.filter   { it.id != spell.id },
                            preparedSpells = s.preparedSpells.filter { it.id != spell.id },
                        )
                    }
                },
                onErr = {}
            )
        }
    }

    // ── Espacios de hechizo ──────────────────────────────────────────────────

    fun updateSlot(level: Int, total: Int? = null, remaining: Int? = null) {
        _state.update { s ->
            val current = s.slots.find { it.level == level }
                ?: SpellSlotLevel(level, 0, 0)
            val newTotal     = total     ?: current.total
            val newRemaining = (remaining ?: current.remaining).coerceAtMost(newTotal)
            val updated = s.slots
                .filter { it.level != level }
                .plus(SpellSlotLevel(level, newTotal, newRemaining))
                .sortedBy { it.level }
            s.copy(slots = updated)
        }
    }

    fun saveSlots() {
        viewModelScope.launch {
            val req = UpdateSpellSlotsRequest(slots = _state.value.slots)
            repo.updateSpellSlots(characterId, req).fold(
                onOk = { updated -> _state.update { it.copy(slots = updated) } },
                onErr = {}
            )
        }
    }
}
