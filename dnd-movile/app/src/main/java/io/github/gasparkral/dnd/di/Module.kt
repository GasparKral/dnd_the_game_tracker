package io.github.gasparkral.dnd.di

import io.github.gasparkral.dnd.infra.DndJson
import io.github.gasparkral.dnd.infra.SocketManager
import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.infra.webSocketClient
import io.github.gasparkral.dnd.ui.viewmodel.CharacterCreationViewModel
import io.github.gasparkral.dnd.ui.viewmodel.DiceRollerViewModel
import io.github.gasparkral.dnd.ui.viewmodel.InventoryViewModel
import io.github.gasparkral.dnd.ui.viewmodel.SpellViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import org.koin.androidx.viewmodel.dsl.viewModel
import org.koin.dsl.module

val appModule = module {

    // ── Scope de aplicación para el SocketManager ────────────────────────────
    single { CoroutineScope(SupervisorJob() + Dispatchers.IO) }

    // ── WebSocket singleton — una única conexión compartida por toda la app ──
    single { SocketManager(client = webSocketClient, scope = get(), json = DndJson) }

    // ── Repositorios ─────────────────────────────────────────────────────────
    single { DraftRepository() }

    // ── ViewModels ────────────────────────────────────────────────────────────
    viewModel { (playerName: String) ->
        CharacterCreationViewModel(
            repo = get(),
            playerName = playerName,
        )
    }

    viewModel { (characterId: String) ->
        InventoryViewModel(
            characterId = characterId,
            repo = get(),
            socketManager = get(),
        )
    }

    viewModel { (characterId: String) ->
        SpellViewModel(
            characterId = characterId,
            repo = get(),
        )
    }

    viewModel {
        DiceRollerViewModel(
            socketManager = get()
        )
    }
}
