package io.github.gasparkral.dnd.ui.screen

import android.util.Log
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
import io.github.gasparkral.dnd.infra.*
import io.github.gasparkral.dnd.ui.component.DndDivider
import io.github.gasparkral.dnd.ui.component.DndLabel
import io.github.gasparkral.dnd.ui.theme.*
import io.github.gasparkral.dnd.utils.ErrorMessage
import io.github.gasparkral.dnd.utils.Result
import io.github.gasparkral.dnd.utils.globals.UrlConnection
import kotlinx.coroutines.*

@Composable
fun RequestUrlConnectionScreen(
    modifier: Modifier = Modifier,
    onConnected: () -> Unit
) {
    var text by remember { mutableStateOf(TextFieldValue("")) }
    var errorMsg by remember { mutableStateOf("") }
    var isConnecting by remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()

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
                    focusedBorderColor = Gold,
                    unfocusedBorderColor = Iron,
                    errorBorderColor = Ember,
                    focusedTextColor = Parchment,
                    unfocusedTextColor = Parchment,
                    cursorColor = Aurum,
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
                    scope.launch {
                        isConnecting = true
                        errorMsg = ""
                        val result = withContext(Dispatchers.IO) { tryConnection() }
                        isConnecting = false
                        result.fold(
                            onOk = { onConnected() },
                            onErr = { error ->
                                errorMsg = when (error) {
                                    UIConnectionError.JetUnhandledError -> "Error inesperado"
                                    UIConnectionError.ErrorOnConnection -> "No se pudo conectar con el servidor"
                                }
                            }
                        )
                    }
                },
                enabled = text.text.isNotBlank() && !isConnecting,
                colors = ButtonDefaults.buttonColors(
                    containerColor = Gold,
                    contentColor = Void,
                    disabledContainerColor = Iron,
                    disabledContentColor = Ash,
                ),
                shape = RoundedCornerShape(3.dp),
                modifier = Modifier
                    .fillMaxWidth(0.7f)
                    .height(50.dp)
            ) {
                if (isConnecting) {
                    CircularProgressIndicator(
                        color = Void,
                        modifier = Modifier.size(20.dp),
                        strokeWidth = 2.dp,
                    )
                } else {
                    Text(
                        "Conectar",
                        style = MaterialTheme.typography.labelLarge,
                    )
                }
            }
        }
    }
}

fun tryConnection(): Result<Unit, UIConnectionError> {
    val normalized = normalizeServerUrl(UrlConnection.URL)
        ?: return Result.Err(UIConnectionError.ErrorOnConnection)

    // Verificar conectividad HTTP antes de continuar
    val reachable = runCatching {
        val url = java.net.URL("$normalized/api/campaign")
        val conn = url.openConnection() as java.net.HttpURLConnection
        conn.connectTimeout = 5_000
        conn.readTimeout = 5_000
        conn.requestMethod = "GET"
        val code = conn.responseCode
        conn.disconnect()
        code in 200..499  // solo consideramos reachable si no es un error de gateway/tunnel
    }.getOrDefault(false)

    Log.d("CONNECT", "$reachable:$normalized")

    if (!reachable) return Result.Err(UIConnectionError.ErrorOnConnection)

    val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
    val socket = SocketManager(client = webSocketClient, scope = scope, json = DndJson)
    val wsUrl = normalized.replace("https://", "wss://").replace("http://", "ws://")
    scope.launch { socket.connect("$wsUrl/ws/game") }
    HttpManager.init(baseUrl = normalized, client = httpClient)
    return Result.Ok(Unit)
}

/**
 * Normaliza la URL introducida por el usuario:
 * - Elimina espacios y saltos de línea
 * - Añade "https://" si no tiene esquema
 * - Elimina la barra final
 * - Devuelve null si el resultado no tiene un host válido
 */
fun normalizeServerUrl(raw: String): String? {
    val trimmed = raw.trim()
    if (trimmed.isBlank()) return null

    // Añadir esquema si falta
    val withScheme = when {
        trimmed.startsWith("http://") || trimmed.startsWith("https://") -> trimmed
        trimmed.startsWith("wss://") -> trimmed.replace("wss://", "https://")
        trimmed.startsWith("ws://") -> trimmed.replace("ws://", "http://")
        else -> "https://$trimmed"
    }

    // Eliminar trailing slash
    val clean = withScheme.trimEnd('/')

    // Validar que hay algo después del esquema
    val host = clean.removePrefix("https://").removePrefix("http://")
    if (host.isBlank() || host.contains(" ")) return null

    return clean
}

enum class UIConnectionError : ErrorMessage {
    ErrorOnConnection,
    JetUnhandledError
}
