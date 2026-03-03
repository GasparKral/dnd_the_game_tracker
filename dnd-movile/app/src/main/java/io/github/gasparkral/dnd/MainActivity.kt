package io.github.gasparkral.dnd

import android.content.Context
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import io.github.gasparkral.dnd.ui.screen.CharacterCreationScreen
import io.github.gasparkral.dnd.ui.screen.CharacterInfoScreen
import io.github.gasparkral.dnd.ui.screen.CharacterSelectionScreen
import io.github.gasparkral.dnd.ui.screen.CombatScreen
import io.github.gasparkral.dnd.ui.screen.DashboardScreen
import io.github.gasparkral.dnd.ui.screen.InventoryScreen
import io.github.gasparkral.dnd.ui.screen.LoreScreen
import io.github.gasparkral.dnd.ui.screen.RequestUrlConnectionScreen
import io.github.gasparkral.dnd.ui.screen.SetupUsernameScreen
import io.github.gasparkral.dnd.ui.theme.DndTheme
import kotlinx.serialization.Serializable

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        val prefs = getSharedPreferences("dnd_prefs", Context.MODE_PRIVATE)
        val startDest: Any = if (prefs.getString("player_name", null) == null) SetupUsername else RequestUrl

        setContent {

            val navController = rememberNavController()

            DndTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    NavHost(navController = navController, startDestination = startDest) {

                        composable<SetupUsername> {
                            SetupUsernameScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onUsernameSaved = { name ->
                                    prefs.edit().putString("player_name", name).apply()
                                    navController.navigate(RequestUrl) {
                                        popUpTo(SetupUsername) { inclusive = true }
                                    }
                                }
                            )
                        }

                        composable<RequestUrl> {
                            RequestUrlConnectionScreen(
                                Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                { navController.navigate(route = CharacterSelection) }
                            )
                        }

                        composable<CharacterSelection> {
                            CharacterSelectionScreen(
                                Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                navigateToCreateCharacter = { navController.navigate(route = CharacterCreation) },
                                navigateToCharacterInfo = { c ->
                                    CharacterInfo.character = c
                                    navController.navigate(route = Dashboard)
                                }
                            )
                        }

                        composable<CharacterCreation> {
                            CharacterCreationScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onBack = { navController.popBackStack() },
                                onCharacterCreated = { navController.popBackStack() }
                            )
                        }

                        composable<Dashboard> {
                            DashboardScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                character = CharacterInfo.character,
                                isInCombat = false, // TODO: desde ViewModel global / WS state
                                onNavigateToCharacterInfo = { navController.navigate(CharacterInfo) },
                                onNavigateToInventory     = { navController.navigate(Inventory) },
                                onNavigateToLore          = { navController.navigate(Lore) },
                                onNavigateToCombat        = { navController.navigate(Combat) }
                            )
                        }

                        composable<CharacterInfo> {
                            CharacterInfoScreen(
                                Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                character = CharacterInfo.character
                            )
                        }

                        composable<Lore> {
                            LoreScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onBack = { navController.popBackStack() }
                            )
                        }

                        composable<Inventory> {
                            InventoryScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onBack = { navController.popBackStack() }
                            )
                        }

                        composable<Combat> {
                            CombatScreen(
                                modifier = Modifier
                                    .fillMaxSize()
                                    .padding(innerPadding),
                                onBack = { navController.popBackStack() }
                            )
                        }
                    }
                }
            }
        }
    }
}

@Serializable object SetupUsername
@Serializable object RequestUrl
@Serializable object CharacterSelection
@Serializable object CharacterCreation
@Serializable object Dashboard
@Serializable object Lore
@Serializable object Inventory
@Serializable object Combat

@Serializable
object CharacterInfo {
    lateinit var character: io.github.gasparkral.dnd.infra.dbstruct.Character
}