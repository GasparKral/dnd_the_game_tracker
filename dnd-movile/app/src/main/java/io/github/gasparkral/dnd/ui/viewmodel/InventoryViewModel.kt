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

data class InventoryUiState(
    val isLoading: Boolean = true,
    val error: String? = null,
    val items: List<InventoryItem> = emptyList(),
    val currency: Currency = Currency(),
    val totalWeight: Float = 0f,
    // Estado del diálogo de añadir objeto
    val showAddDialog: Boolean = false,
    val isSaving: Boolean = false,
    // Estado del diálogo de monedas
    val showCurrencyDialog: Boolean = false,
)

class InventoryViewModel(
    private val characterId: String,
    private val repo: DraftRepository,
) : ViewModel() {

    private val _state = MutableStateFlow(InventoryUiState())
    val state: StateFlow<InventoryUiState> = _state.asStateFlow()

    init {
        loadInventory()
    }

    fun loadInventory() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            repo.getInventory(characterId).fold(
                onOk = { resp ->
                    _state.update {
                        it.copy(
                            isLoading = false,
                            items = resp.items,
                            currency = resp.currency,
                            totalWeight = resp.totalWeight,
                        )
                    }
                },
                onErr = { _state.update { it.copy(isLoading = false, error = it.error) } }
            )
        }
    }

    // ── Añadir objeto ────────────────────────────────────────────────────────

    fun openAddDialog() = _state.update { it.copy(showAddDialog = true) }
    fun closeAddDialog() = _state.update { it.copy(showAddDialog = false) }

    fun addItem(
        name: String,
        category: ItemCategory,
        description: String,
        quantity: Int,
        weight: Float?,
        notes: String,
    ) {
        if (name.isBlank() || quantity < 1) return
        viewModelScope.launch {
            _state.update { it.copy(isSaving = true) }
            repo.addItem(
                characterId,
                AddItemRequest(name.trim(), category, description.trim(), quantity, weight, notes.trim()),
            ).fold(
                onOk = { item ->
                    _state.update {
                        it.copy(
                            isSaving = false,
                            showAddDialog = false,
                            items = it.items + item,
                            totalWeight = it.totalWeight + (item.weight ?: 0f) * item.quantity,
                        )
                    }
                },
                onErr = { _state.update { it.copy(isSaving = false) } }
            )
        }
    }

    // ── Equipar / desequipar ─────────────────────────────────────────────────

    fun toggleEquipped(item: InventoryItem) {
        viewModelScope.launch {
            repo.updateItem(characterId, item.id, UpdateItemRequest(equipped = !item.equipped)).fold(
                onOk = { updated ->
                    _state.update { s ->
                        s.copy(items = s.items.map { if (it.id == updated.id) updated else it })
                    }
                },
                onErr = { /* silencioso — la UI no cambia */ }
            )
        }
    }

    // ── Cambiar cantidad ─────────────────────────────────────────────────────

    fun updateQuantity(item: InventoryItem, newQuantity: Int) {
        if (newQuantity < 0) return
        viewModelScope.launch {
            if (newQuantity == 0) {
                deleteItem(item)
                return@launch
            }
            repo.updateItem(characterId, item.id, UpdateItemRequest(quantity = newQuantity)).fold(
                onOk = { updated ->
                    _state.update { s ->
                        s.copy(
                            items = s.items.map { if (it.id == updated.id) updated else it },
                            totalWeight = s.items
                                .map { if (it.id == updated.id) updated else it }
                                .sumOf { ((it.weight ?: 0f) * it.quantity).toDouble() }
                                .toFloat(),
                        )
                    }
                },
                onErr = { }
            )
        }
    }

    // ── Eliminar ─────────────────────────────────────────────────────────────

    fun deleteItem(item: InventoryItem) {
        viewModelScope.launch {
            repo.deleteItem(characterId, item.id).fold(
                onOk = {
                    _state.update { s ->
                        s.copy(
                            items = s.items.filter { it.id != item.id },
                            totalWeight = s.totalWeight - (item.weight ?: 0f) * item.quantity,
                        )
                    }
                },
                onErr = { }
            )
        }
    }

    // ── Monedas ──────────────────────────────────────────────────────────────

    fun openCurrencyDialog() = _state.update { it.copy(showCurrencyDialog = true) }
    fun closeCurrencyDialog() = _state.update { it.copy(showCurrencyDialog = false) }

    fun updateCurrency(copper: Int, silver: Int, electrum: Int, gold: Int, platinum: Int) {
        viewModelScope.launch {
            _state.update { it.copy(isSaving = true) }
            repo.updateCurrency(
                characterId,
                UpdateCurrencyRequest(copper, silver, electrum, gold, platinum),
            ).fold(
                onOk = { c ->
                    _state.update { it.copy(isSaving = false, showCurrencyDialog = false, currency = c) }
                },
                onErr = { _state.update { it.copy(isSaving = false) } }
            )
        }
    }
}
