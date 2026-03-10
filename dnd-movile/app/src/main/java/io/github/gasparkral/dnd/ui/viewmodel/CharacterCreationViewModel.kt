package io.github.gasparkral.dnd.ui.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.github.gasparkral.dnd.infra.HttpManager
import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.model.AttributesDto
import io.github.gasparkral.dnd.model.CatalogEntry
import io.github.gasparkral.dnd.model.CharacterDraft
import io.github.gasparkral.dnd.model.ChoiceSchema
import io.github.gasparkral.dnd.model.CreationStep
import io.github.gasparkral.dnd.model.UpdateDraftRequest
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.async
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonPrimitive

// ---------------------------------------------------------------------------
// Estado de la pantalla
// ---------------------------------------------------------------------------

data class CreationUiState(
    // ── Carga inicial ────────────────────────────────────────────────────────
    val isLoadingCatalogs: Boolean = true,
    val catalogError: String? = null,

    // ── Catálogos ────────────────────────────────────────────────────────────
    val races: List<CatalogEntry> = emptyList(),
    val classes: List<CatalogEntry> = emptyList(),
    val backgrounds: List<CatalogEntry> = emptyList(),
    val feats: List<CatalogEntry> = emptyList(),

    // ── Draft del servidor ───────────────────────────────────────────────────
    val draft: CharacterDraft = CharacterDraft(),

    // ── Selecciones locales (pendientes de enviar al servidor) ───────────────
    val localName: String = "",
    val selectedRaceId: String = "",
    val selectedClassId: String = "",
    val selectedBackgroundId: String = "",
    val attributes: AttributesDto = AttributesDto.DEFAULT,
    /** choices[choiceId] = valor elegido */
    val choices: Map<String, JsonElement> = emptyMap(),
    /** IDs de los dones seleccionados en el paso Feats */
    val selectedFeatIds: List<String> = emptyList(),

    // ── Estado de la llamada al servidor ─────────────────────────────────────
    val isSaving: Boolean = false,
    val stepErrors: List<String> = emptyList(),

    // ── Finalización ─────────────────────────────────────────────────────────
    val isComplete: Boolean = false,
) {
    val currentStep: CreationStep get() = draft.step

    val totalSteps: Int get() = CreationStep.entries.size - 1 // excluye Complete

    val stepIndex: Int get() = CreationStep.entries.indexOf(currentStep).coerceAtLeast(0)

    val canAdvance: Boolean get() = when (currentStep) {
        CreationStep.Name       -> localName.isNotBlank()
        CreationStep.Race       -> selectedRaceId.isNotBlank()
        CreationStep.Class      -> selectedClassId.isNotBlank()
        CreationStep.Background -> selectedBackgroundId.isNotBlank()
        CreationStep.Attributes -> attributes.pointBuyCost() <= 27
        else                    -> true
    }

    val selectedRace: CatalogEntry?       get() = races.find { it.id == selectedRaceId }
    val selectedClass: CatalogEntry?      get() = classes.find { it.id == selectedClassId }
    val selectedBackground: CatalogEntry? get() = backgrounds.find { it.id == selectedBackgroundId }
}

// ---------------------------------------------------------------------------
// ViewModel
// ---------------------------------------------------------------------------

class CharacterCreationViewModel(
    private val repo: DraftRepository,
    private val playerName: String,
) : ViewModel() {

    private val _uiState = MutableStateFlow(CreationUiState())
    val uiState: StateFlow<CreationUiState> = _uiState.asStateFlow()

    init {
        loadCatalogsAndCreateDraft()
    }

    // ── Carga inicial ─────────────────────────────────────────────────────────

    private fun loadCatalogsAndCreateDraft() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoadingCatalogs = true, catalogError = null) }

            // Guardia: HttpManager debe estar inicializado (se hace en RequestUrlConnectionScreen)
            if (!HttpManager.isInitialized()) {
                _uiState.update {
                    it.copy(
                        isLoadingCatalogs = false,
                        catalogError = "No hay conexión con el servidor. Vuelve atrás y conectáte primero.",
                    )
                }
                return@launch
            }

            // Catálogos en paralelo real con async
            val racesDeferred      = async { repo.getRaces() }
            val classesDeferred    = async { repo.getClasses() }
            val backgroundDeferred = async { repo.getBackgrounds() }
            val featsDeferred      = async { repo.getFeats() }

            val racesResult      = racesDeferred.await()
            val classesResult    = classesDeferred.await()
            val backgroundResult = backgroundDeferred.await()
            val featsResult      = featsDeferred.await()

            val races       = racesResult.ok()?.entries      ?: emptyList()
            val classes     = classesResult.ok()?.entries    ?: emptyList()
            val backgrounds = backgroundResult.ok()?.entries ?: emptyList()
            val feats       = featsResult.ok()?.entries      ?: emptyList()

            // El primer error que encontremos, con su causa real
            val catalogError: String? = racesResult.err()?.toString()
                ?: classesResult.err()?.toString()
                ?: backgroundResult.err()?.toString()
                ?: featsResult.err()?.toString()

            if (catalogError != null) {
                _uiState.update {
                    it.copy(
                        isLoadingCatalogs = false,
                        catalogError = catalogError,
                        races = races,
                        classes = classes,
                        backgrounds = backgrounds,
                        feats = feats,
                    )
                }
                return@launch
            }

            // Crear draft en el servidor solo si los catálogos cargaron bien
            val draftResult = repo.createDraft(playerName)

            _uiState.update {
                it.copy(
                    isLoadingCatalogs = false,
                    catalogError = draftResult.err()?.toString(),
                    races = races,
                    classes = classes,
                    backgrounds = backgrounds,
                    feats = feats,
                    draft = draftResult.ok()?.draft ?: CharacterDraft(),
                )
            }
        }
    }

    // ── Mutaciones locales (no llaman al servidor todavía) ────────────────────

    fun onNameChange(value: String) = _uiState.update { it.copy(localName = value) }

    fun onRaceSelected(id: String) = _uiState.update { it.copy(selectedRaceId = id, choices = emptyMap()) }

    fun onClassSelected(id: String) = _uiState.update { it.copy(selectedClassId = id, choices = emptyMap()) }

    fun onBackgroundSelected(id: String) = _uiState.update { it.copy(selectedBackgroundId = id, choices = emptyMap()) }

    fun onChoiceAnswered(choiceId: String, value: JsonElement) =
        _uiState.update { it.copy(choices = it.choices + (choiceId to value)) }

    fun onAttributeChanged(field: String, value: Int) =
        _uiState.update { it.copy(attributes = it.attributes.withField(field, value)) }

    fun onFeatToggled(featId: String) =
        _uiState.update { state ->
            val current = state.selectedFeatIds.toMutableList()
            if (featId in current) current.remove(featId) else current.add(featId)
            state.copy(selectedFeatIds = current)
        }

    // ── Avanzar paso — llama al servidor ─────────────────────────────────────

    fun advance() {
        val state = _uiState.value
        if (!state.canAdvance || state.isSaving) return
        val draftId = state.draft.draftId ?: return

        viewModelScope.launch {
            _uiState.update { it.copy(isSaving = true, stepErrors = emptyList()) }

            val request = buildUpdateRequest(draftId, state)
            val result  = repo.updateDraft(request)

            result.fold(
                onOk = { response ->
                    _uiState.update { it.copy(
                        isSaving    = false,
                        draft       = response.draft,
                        stepErrors  = response.errors,
                        isComplete  = response.finalized,
                    )}
                },
                onErr = { error ->
                    _uiState.update { it.copy(
                        isSaving   = false,
                        stepErrors = listOf(error.toString()),
                    )}
                }
            )
        }
    }

    // ── Retroceder paso ───────────────────────────────────────────────────────

    fun back(): Boolean {
        val step = _uiState.value.currentStep
        if (step == CreationStep.Name) return false   // señal para salir de la pantalla
        // Retroceso local — no notificamos al servidor, solo cambiamos el step localmente
        val prevStep = CreationStep.entries[CreationStep.entries.indexOf(step) - 1]
        _uiState.update { it.copy(draft = it.draft.copy(step = prevStep), stepErrors = emptyList()) }
        return true
    }

    // ── Constructor del request ───────────────────────────────────────────────

    private fun buildUpdateRequest(draftId: String, state: CreationUiState): UpdateDraftRequest =
        UpdateDraftRequest(
            draftId      = draftId,
            step         = state.currentStep,
            // En el paso Review enviamos playerName para que el servidor pueda hacer el auto-save
            playerName   = playerName.takeIf { state.currentStep == CreationStep.Review },
            name         = state.localName.takeIf { state.currentStep == CreationStep.Name },
            raceId       = state.selectedRaceId.takeIf { state.currentStep == CreationStep.Race },
            classId      = state.selectedClassId.takeIf { state.currentStep == CreationStep.Class },
            backgroundId = state.selectedBackgroundId.takeIf { state.currentStep == CreationStep.Background },
            attributes   = state.attributes.takeIf { state.currentStep == CreationStep.Attributes },
            featIds      = state.selectedFeatIds.takeIf { state.currentStep == CreationStep.Feats } ?: emptyList(),
            choices      = state.choices,
        )
}
