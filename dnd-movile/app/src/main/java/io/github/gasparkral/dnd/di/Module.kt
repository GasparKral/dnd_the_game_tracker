package io.github.gasparkral.dnd.di

import io.github.gasparkral.dnd.infra.repository.DraftRepository
import io.github.gasparkral.dnd.ui.viewmodel.CharacterCreationViewModel
import io.github.gasparkral.dnd.ui.viewmodel.InventoryViewModel
import org.koin.androidx.viewmodel.dsl.viewModel
import org.koin.dsl.module

val appModule = module {

    // ── Repositorios ──────────────────────────────────────────────────────────
    // DraftRepository no tiene estado — singleton es suficiente.
    single { DraftRepository() }

    // ── ViewModels ────────────────────────────────────────────────────────────
    // factory porque cada sesión de creación es independiente.
    // El playerName llega como parámetro desde la pantalla.
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
        )
    }
}
