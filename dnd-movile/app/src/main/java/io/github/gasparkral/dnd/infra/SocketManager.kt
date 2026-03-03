package io.github.gasparkral.dnd.infra

import android.util.Log
import io.ktor.client.*
import io.ktor.client.plugins.websocket.*
import io.ktor.websocket.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.launch
import kotlinx.serialization.json.Json

class SocketManager(
    private val client: HttpClient,
    private val scope: CoroutineScope,
    private val json: Json
) {
    private val _messages = MutableSharedFlow<ServerMessage>()
    val messages = _messages.asSharedFlow()

    private var session: DefaultClientWebSocketSession? = null

    fun connect(url: String) {
        scope.launch {
            client.webSocket(urlString = url) {
                session = this
                listenIncoming()
            }
        }
    }

    suspend fun send(msg: ClientMessage) {
        session?.sendSerialized(msg)
    }

    suspend fun disconnect() {
        session?.close()
        session = null
    }

    private suspend fun DefaultClientWebSocketSession.listenIncoming() {
        try {
            for (frame in incoming) {
                if (frame is Frame.Text) {
                    val msg = json.decodeFromString<ServerMessage>(frame.readText())
                    _messages.emit(msg)
                }
            }
        } catch (e: Exception) {
            Log.e("Decode Error", e.message!!)
        }
    }
}