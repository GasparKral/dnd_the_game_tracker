package io.github.gasparkral.dnd.ui.screen

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.infra.ClientMessage
import io.github.gasparkral.dnd.infra.HttpManager
import io.github.gasparkral.dnd.infra.SocketManager
import io.github.gasparkral.dnd.infra.DndJson
import io.github.gasparkral.dnd.infra.httpClient
import io.github.gasparkral.dnd.infra.webSocketClient
import io.github.gasparkral.dnd.ui.component.DndDivider
import io.github.gasparkral.dnd.ui.component.DndLabel
import io.github.gasparkral.dnd.ui.theme.*
import io.github.gasparkral.dnd.utils.ErrorMessage
import io.github.gasparkral.dnd.utils.Result
import io.github.gasparkral.dnd.utils.globals.UrlConnection
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

@Composable
fun RequestUrlConnectionScreen(
    modifier: Modifier = Modifier,
    onConnected: () -> Unit
) {
    var text by remember { mutableStateOf(TextFieldValue("")) }
    var errorMsg by remember { mutableStateOf("") }

    Box(
        modifier = modifier
            .background(
                Brush.radialGradient(
                    colors = listOf(Dungeon, Void),
                    radius = 1200f
                )
            )
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(horizontal = 28.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {

            // ── Título ────────────────────────────────────────────────────
            Text(
                text = "Dungeons & Dragons",
                style = MaterialTheme.typography.displaySmall,
                textAlign = TextAlign.Center,
            )
            Spacer(Modifier.height(4.dp))
            Text(
                text = "The Game Tracker",
                style = MaterialTheme.typography.headlineSmall,
                color = Ash,
                textAlign = TextAlign.Center,
            )

            Spacer(Modifier.height(32.dp))
            DndDivider(symbol = "⚔")
            Spacer(Modifier.height(32.dp))

            // ── Instrucción ───────────────────────────────────────────────
            Text(
                text = "Introduce la URL que te indique el Dungeon Master",
                style = MaterialTheme.typography.bodyMedium,
                color = Bone,
                textAlign = TextAlign.Center,
            )

            Spacer(Modifier.height(20.dp))

            // ── Campo de texto ────────────────────────────────────────────
            DndLabel(text = "Dirección del servidor")
            Spacer(Modifier.height(6.dp))
            OutlinedTextField(
                value = text,
                onValueChange = {
                    text = it
                    UrlConnection.URL = it.text
                    errorMsg = ""
                },
                placeholder = {
                    Text(
                        "https://texto-aleatorio.trycloudflare.com",
                        style = MaterialTheme.typography.bodySmall,
                        color = Ash.copy(alpha = 0.6f)
                    )
                },
                singleLine = true,
                isError = errorMsg.isNotEmpty(),
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri),
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor   = Gold,
                    unfocusedBorderColor = Iron,
                    errorBorderColor     = Ember,
                    focusedTextColor     = Parchment,
                    unfocusedTextColor   = Parchment,
                    cursorColor          = Aurum,
                ),
                shape = RoundedCornerShape(3.dp),
                modifier = Modifier.fillMaxWidth()
            )

            // ── Mensaje de error ──────────────────────────────────────────
            if (errorMsg.isNotEmpty()) {
                Spacer(Modifier.height(6.dp))
                Text(
                    text = "✦ $errorMsg",
                    color = Ember,
                    style = MaterialTheme.typography.bodySmall,
                )
            }

            Spacer(Modifier.height(36.dp))

            // ── Botón conectar ────────────────────────────────────────────
            Button(
                onClick = {
                    tryConnection().fold(
                        onOk = {
                            errorMsg = ""
                            onConnected()
                        },
                        onErr = { error ->
                            errorMsg = when (error) {
                                UIConnectionError.JetUnhandledError -> "Error inesperado"
                                UIConnectionError.ErrorOnConnection -> "No se pudo conectar con el servidor"
                            }
                        }
                    )
                },
                enabled = text.text.isNotBlank(),
                colors = ButtonDefaults.buttonColors(
                    containerColor = Gold,
                    contentColor   = Void,
                    disabledContainerColor = Iron,
                    disabledContentColor   = Ash,
                ),
                shape = RoundedCornerShape(3.dp),
                modifier = Modifier
                    .fillMaxWidth(0.7f)
                    .height(50.dp)
            ) {
                Text(
                    "Conectar",
                    style = MaterialTheme.typography.labelLarge,
                )
            }
        }
    }
}

fun tryConnection(): Result<Unit, UIConnectionError> {
    val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
    ClientMessage.Join("Gaspar", "Test")
    val socket = SocketManager(client = webSocketClient, scope = scope, json = DndJson)
    try {
        val url = UrlConnection.URL.replace("https", "wss")
        scope.launch { socket.connect("$url/ws/game") }
        HttpManager.init(baseUrl = UrlConnection.URL, client = httpClient)
    } catch (e: Exception) {
        return Result.Err(UIConnectionError.ErrorOnConnection)
    }
    return Result.Ok(Unit)
}

enum class UIConnectionError : ErrorMessage {
    ErrorOnConnection,
    JetUnhandledError
}
