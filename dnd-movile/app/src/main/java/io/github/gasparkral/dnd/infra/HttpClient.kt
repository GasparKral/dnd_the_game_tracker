package io.github.gasparkral.dnd.infra

import io.ktor.client.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.logging.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json

val DndJson = Json {
    // Rust puede añadir campos nuevos sin romper la app
    ignoreUnknownKeys = true
    // Permite JSON malformado con tolerancia
    isLenient = true
    // No serializar nulls salvo que sea necesario
    explicitNulls = false
    // El discriminador de sealed class lo marca @JsonClassDiscriminator en cada clase
    // — no hace falta classDiscriminator global aquí
}

val httpClient = HttpClient {
    install(Logging) {
        logger = Logger.DEFAULT
        // BODY para ver el JSON completo en Logcat durante desarrollo
        level = LogLevel.BODY
    }
    install(ContentNegotiation) {
        json(DndJson)
    }
}
