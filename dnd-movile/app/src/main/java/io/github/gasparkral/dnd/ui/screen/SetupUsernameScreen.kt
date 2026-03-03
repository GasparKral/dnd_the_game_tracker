package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp

/**
 * Pantalla que se muestra la primera vez que el usuario abre la app,
 * antes de RequestUrlConnectionScreen. Obliga a introducir un nombre
 * de jugador que se persiste en SharedPreferences.
 *
 * El guardado real se implementará en el ViewModel / UseCase correspondiente.
 */
@Composable
fun SetupUsernameScreen(
    modifier: Modifier = Modifier,
    onUsernameSaved: (String) -> Unit
) {
    var username by remember { mutableStateOf(TextFieldValue("")) }
    var showError by remember { mutableStateOf(false) }

    Column(
        modifier = modifier.padding(24.dp),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = "Bienvenido aventurero",
            style = MaterialTheme.typography.headlineMedium
        )

        Spacer(Modifier.height(8.dp))

        Text(
            text = "¿Cómo te llaman en la taberna?",
            style = MaterialTheme.typography.bodyMedium
        )

        Spacer(Modifier.height(24.dp))

        OutlinedTextField(
            value = username,
            onValueChange = {
                username = it
                showError = false
            },
            label = { Text("Tu nombre de jugador") },
            singleLine = true,
            isError = showError,
            supportingText = {
                if (showError) Text("El nombre no puede estar vacío")
            }
        )

        Spacer(Modifier.height(32.dp))

        Button(
            onClick = {
                if (username.text.isBlank()) {
                    showError = true
                } else {
                    // TODO: persistir en SharedPreferences / DataStore
                    onUsernameSaved(username.text.trim())
                }
            },
            modifier = Modifier.fillMaxWidth(0.6f)
        ) {
            Text("Continuar")
        }
    }
}
