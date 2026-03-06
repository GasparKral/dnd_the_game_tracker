package io.github.gasparkral.dnd.infra.repository

import io.github.gasparkral.dnd.infra.HttpManager
import io.github.gasparkral.dnd.infra.HttpResult
import io.github.gasparkral.dnd.model.CampaignSummary
import io.github.gasparkral.dnd.model.CatalogResponse
import io.github.gasparkral.dnd.model.CharactersResponse
import io.github.gasparkral.dnd.model.CreateCampaignRequest
import io.github.gasparkral.dnd.model.CreateDraftRequest
import io.github.gasparkral.dnd.model.DraftResponse
import io.github.gasparkral.dnd.model.DraftStatusResponse
import io.github.gasparkral.dnd.model.AddItemRequest
import io.github.gasparkral.dnd.model.Currency
import io.github.gasparkral.dnd.model.InventoryItem
import io.github.gasparkral.dnd.model.InventoryResponse
import io.github.gasparkral.dnd.model.SavedCharacter
import io.github.gasparkral.dnd.model.UpdateCurrencyRequest
import io.github.gasparkral.dnd.model.UpdateItemRequest
import io.github.gasparkral.dnd.model.UpdateDraftRequest

class DraftRepository {

    // ── Campaña ────────────────────────────────────────────────────

    suspend fun getCampaign(): HttpResult<CampaignSummary> =
        HttpManager.get("/api/campaign")

    suspend fun createCampaign(name: String, description: String = ""): HttpResult<CampaignSummary> =
        HttpManager.post(
            endpoint = "/api/campaign",
            body = CreateCampaignRequest(name, description),
        )

    // ── Personajes ──────────────────────────────────────────────────

    /** Personajes de un jugador concreto (o todos si [playerName] es null). */
    suspend fun getCharacters(playerName: String? = null): HttpResult<CharactersResponse> {
        val query = if (playerName != null) mapOf("player" to playerName) else emptyMap()
        return HttpManager.get("/api/characters", queryParams = query)
    }

    suspend fun getCharacter(characterId: String): HttpResult<SavedCharacter> =
        HttpManager.get("/api/characters/$characterId")

    // ── Inventario ──────────────────────────────────────────────────

    suspend fun getInventory(characterId: String): HttpResult<InventoryResponse> =
        HttpManager.get("/api/characters/$characterId/inventory")

    suspend fun addItem(characterId: String, req: AddItemRequest): HttpResult<InventoryItem> =
        HttpManager.post("/api/characters/$characterId/inventory", req)

    suspend fun updateItem(characterId: String, itemId: String, req: UpdateItemRequest): HttpResult<InventoryItem> =
        HttpManager.put("/api/characters/$characterId/inventory/$itemId", req)

    suspend fun deleteItem(characterId: String, itemId: String): HttpResult<Unit> =
        HttpManager.delete("/api/characters/$characterId/inventory/$itemId")

    suspend fun updateCurrency(characterId: String, req: UpdateCurrencyRequest): HttpResult<Currency> =
        HttpManager.put("/api/characters/$characterId/currency", req)

    // ── Catálogos ─────────────────────────────────────────────────────────────

    suspend fun getRaces(): HttpResult<CatalogResponse> =
        HttpManager.get("/api/catalog/races")

    suspend fun getClasses(): HttpResult<CatalogResponse> =
        HttpManager.get("/api/catalog/classes")

    suspend fun getBackgrounds(): HttpResult<CatalogResponse> =
        HttpManager.get("/api/catalog/backgrounds")

    // ── Draft ─────────────────────────────────────────────────────────────────

    /** Inicia un nuevo draft en el servidor. Devuelve el draft con su id asignado. */
    suspend fun createDraft(playerName: String): HttpResult<DraftResponse> =
        HttpManager.post(
            endpoint = "/api/character/draft",
            body = CreateDraftRequest(playerName),
        )

    /** Recupera el estado actual de un draft por su id. */
    suspend fun getDraft(draftId: String): HttpResult<DraftStatusResponse> =
        HttpManager.get("/api/character/draft/$draftId")

    /** Envía los datos del paso actual y avanza el wizard en el servidor. */
    suspend fun updateDraft(request: UpdateDraftRequest): HttpResult<DraftResponse> =
        HttpManager.put(
            endpoint = "/api/character/draft/${request.draftId}",
            body = request,
        )
}
