package io.github.gasparkral.dnd

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.ui.Modifier
import androidx.core.content.edit
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.toRoute
import io.github.gasparkral.dnd.ui.screen.*
import io.github.gasparkral.dnd.ui.theme.DndTheme
import kotlinx.serialization.Serializable

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        val prefs = getSharedPreferences("dnd_prefs", MODE_PRIVATE)
        val playerName = prefs.getString("player_name", null)
        val startDest: Any = if (playerName == null) SetupUsername else RequestUrl

        setContent {
            val navController = rememberNavController()

            DndTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    NavHost(navController = navController, startDestination = startDest) {

                        // ── Setup nombre de jugador ────────────────────────
                        composable<SetupUsername> {
                            SetupUsernameScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onUsernameSaved = { name ->
                                    prefs.edit { putString("player_name", name) }
                                    navController.navigate(RequestUrl) {
                                        popUpTo(SetupUsername) { inclusive = true }
                                    }
                                },
                            )
                        }

                        // ── Configurar URL del servidor ───────────────────
                        composable<RequestUrl> {
                            RequestUrlConnectionScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onConnected = {
                                    navController.navigate(CharacterSelection) {
                                        popUpTo(RequestUrl) { inclusive = true }
                                    }
                                },
                            )
                        }

                        // ── Selección de personaje ─────────────────────────
                        composable<CharacterSelection> {
                            val name = prefs.getString("player_name", "") ?: ""
                            CharacterSelectionScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                playerName = name,
                                navigateToCreateCharacter = {
                                    navController.navigate(CharacterCreation)
                                },
                                navigateToDashboard = { draftId ->
                                    navController.navigate(Dashboard(draftId))
                                },
                            )
                        }

                        // ── Creación de personaje (wizard) ─────────────────
                        composable<CharacterCreation> {
                            val name = prefs.getString("player_name", "") ?: ""
                            CharacterCreationScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                playerName = name,
                                onBack = { navController.popBackStack() },
                                onCharacterCreated = {
                                    navController.navigate(CharacterSelection) {
                                        popUpTo(CharacterCreation) { inclusive = true }
                                    }
                                },
                            )
                        }

                        // ── Dashboard ──────────────────────────────────────
                        composable<Dashboard> { backStackEntry ->
                            val route = backStackEntry.toRoute<Dashboard>()
                            DashboardScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                draftId = route.draftId,
                                onNavigateToInventory = { navController.navigate(Inventory(route.draftId)) },
                                onNavigateToLore = { navController.navigate(Lore) },
                                onNavigateToCombat = { navController.navigate(Combat(route.draftId)) },
                            )
                        }

                        // ── Lore ───────────────────────────────────────────
                        composable<Lore> {
                            LoreScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onBack = { navController.popBackStack() },
                            )
                        }

                        // ── Inventario ─────────────────────────────────────
                        composable<Inventory> { backStackEntry ->
                            val route = backStackEntry.toRoute<Inventory>()
                            InventoryScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                draftId = route.draftId,
                                onBack = { navController.popBackStack() },
                            )
                        }

                        // ── Combate ────────────────────────────────────────
                        composable<Combat> { backStackEntry ->
                            val route = backStackEntry.toRoute<Combat>()
                            CombatScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                draftId = route.draftId,
                                onBack = { navController.popBackStack() },
                            )
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Rutas de navegación
// ---------------------------------------------------------------------------

@Serializable
object SetupUsername

@Serializable
object RequestUrl

@Serializable
object CharacterSelection

@Serializable
object CharacterCreation

@Serializable
object Lore

/** Dashboard, Inventario y Combate reciben el draftId para consultar la API. */
@Serializable
data class Dashboard(val draftId: String)

@Serializable
data class Inventory(val draftId: String)

@Serializable
data class Combat(val draftId: String)
