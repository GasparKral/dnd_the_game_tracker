package io.github.gasparkral.dnd.ui.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.github.gasparkral.dnd.infra.ClientMessage
import io.github.gasparkral.dnd.infra.ServerMessage
import io.github.gasparkral.dnd.infra.SocketManager
import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.model.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

data class InventoryUiState(
    val isLoading: Boolean = true,
    val error: String? = null,
    val items: List<InventoryItem> = emptyList(),
    val currency: Currency = Currency(),
    val totalWeight: Float = 0f,
    val showAddDialog: Boolean = false,
    val isSaving: Boolean = false,
    val showCurrencyDialog: Boolean = false,
    /** Stats efectivos = base + bonuses de equipo. Null hasta que se cargue el personaje. */
    val effectiveStats: EffectiveStats? = null,
)

class InventoryViewModel(
    private val characterId: String,
    private val repo: DraftRepository,
    private val socketManager: SocketManager,
) : ViewModel() {

    /** Atributos base del personaje — se cargan una vez al inicio y se conservan. */
    private var baseAttributes: AttributesDto? = null

    private val _state = MutableStateFlow(InventoryUiState())
    val state: StateFlow<InventoryUiState> = _state.asStateFlow()

    init {
        loadInventory()
        observeServerChanges()
    }

    // Notificar al DM via WS que el jugador cambio algo — best effort
    @OptIn(ExperimentalUuidApi::class)
    private fun notifyDm() {
        viewModelScope.launch {
            try {
                socketManager.send(ClientMessage.InventoryUpdated(characterId = Uuid.parse(characterId)))
            } catch (_: Exception) { /* WS puede no estar activo, no es critico */
            }
        }
    }

    // Escuchar InventoryChanged del servidor (cambios del DM) y recargar
    @OptIn(ExperimentalUuidApi::class)
    private fun observeServerChanges() {
        viewModelScope.launch {
            socketManager.messages.collect { msg ->
                if (msg is ServerMessage.InventoryChanged) {
                    if (msg.characterId.toString() == characterId) {
                        loadInventory()
                    }
                }
            }
        }
    }

    fun loadInventory() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }

            // Cargar atributos base del personaje si no los tenemos aún
            if (baseAttributes == null) {
                repo.getCharacter(characterId).fold(
                    onOk = { baseAttributes = it.attributes },
                    onErr = { /* si falla, el cálculo usará base vacía */ }
                )
            }

            repo.getInventory(characterId).fold(
                onOk = { resp ->
                    val attrs = baseAttributes ?: AttributesDto.DEFAULT
                    val effective = calculateEffectiveStats(attrs, resp.items)
                    _state.update {
                        it.copy(
                            isLoading = false,
                            items = resp.items,
                            currency = resp.currency,
                            totalWeight = resp.totalWeight,
                            effectiveStats = effective,
                        )
                    }
                },
                onErr = { _state.update { it.copy(isLoading = false, error = it.error) } }
            )
        }
    }

    /** Recalcula los [EffectiveStats] sobre la lista de ítems en memoria. */
    private fun recalculateEffective(items: List<InventoryItem>) {
        val attrs = baseAttributes ?: AttributesDto.DEFAULT
        _state.update { it.copy(effectiveStats = calculateEffectiveStats(attrs, items)) }
    }

    // Añadir objeto
    fun openAddDialog() = _state.update { it.copy(showAddDialog = true) }
    fun closeAddDialog() = _state.update { it.copy(showAddDialog = false) }

    fun addItem(
        name: String,
        category: ItemCategory,
        description: String,
        quantity: Int,
        weight: Float?,
        accessoryType: String? = null,
        notes: String,
    ) {
        if (name.isBlank() || quantity < 1) return
        viewModelScope.launch {
            _state.update { it.copy(isSaving = true) }
            repo.addItem(
                characterId,
                AddItemRequest(
                    name.trim(),
                    category,
                    description.trim(),
                    quantity,
                    weight,
                    accessoryType,
                    notes = notes.trim()
                ),
            ).fold(
                onOk = { item ->
                    val newItems = _state.value.items + item
                    _state.update {
                        it.copy(
                            isSaving = false,
                            showAddDialog = false,
                            items = newItems,
                            totalWeight = it.totalWeight + (item.weight ?: 0f) * item.quantity,
                        )
                    }
                    recalculateEffective(newItems)
                    notifyDm()
                },
                onErr = { _state.update { it.copy(isSaving = false) } }
            )
        }
    }

    // Equipar / desequipar
    fun toggleEquipped(item: InventoryItem) {
        viewModelScope.launch {
            repo.updateItem(characterId, item.id, UpdateItemRequest(equipped = !item.equipped)).fold(
                onOk = { updated ->
                    val newItems = _state.value.items.map { if (it.id == updated.id) updated else it }
                    _state.update { s -> s.copy(items = newItems) }
                    recalculateEffective(newItems)
                    notifyDm()
                },
                onErr = {}
            )
        }
    }

    // Cambiar cantidad
    fun updateQuantity(item: InventoryItem, newQuantity: Int) {
        if (newQuantity < 0) return
        viewModelScope.launch {
            if (newQuantity == 0) {
                deleteItem(item); return@launch
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
                    notifyDm()
                },
                onErr = {}
            )
        }
    }

    // Eliminar
    fun deleteItem(item: InventoryItem) {
        viewModelScope.launch {
            repo.deleteItem(characterId, item.id).fold(
                onOk = {
                    val newItems = _state.value.items.filter { it.id != item.id }
                    _state.update { s ->
                        s.copy(
                            items = newItems,
                            totalWeight = s.totalWeight - (item.weight ?: 0f) * item.quantity,
                        )
                    }
                    recalculateEffective(newItems)
                    notifyDm()
                },
                onErr = {}
            )
        }
    }

    // Monedas
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
                    notifyDm()
                },
                onErr = { _state.update { it.copy(isSaving = false) } }
            )
        }
    }
}
