package io.github.gasparkral.dnd.infra

import io.github.gasparkral.dnd.utils.ErrorMessage
import io.ktor.http.HttpStatusCode

sealed class HttpError : ErrorMessage {

    /** El servidor respondió pero con un código de error (4xx, 5xx). */
    data class ApiError(
        val status: HttpStatusCode,
        val body: String,
    ) : HttpError() {
        override fun toString(): String {
            val safeBody = humanReadableBody(body, status.value)
            return "ApiError(${status.value}): $safeBody"
        }
    }

    /** No se pudo alcanzar el servidor (sin red, timeout, DNS...). */
    data class NetworkError(val cause: String) : HttpError() {
        override fun toString() = "No se pudo conectar con el servidor"
    }

    /** La respuesta llegó pero no se pudo deserializar al tipo esperado. */
    data class ParseError(val cause: String) : HttpError() {
        override fun toString() = "Respuesta inesperada del servidor"
    }

    companion object {
        /**
         * Convierte el body de un error en un mensaje legible para el usuario.
         * Si el body es HTML (error de Cloudflare, proxy, etc.) devuelve un
         * mensaje genérico en lugar de volcar el HTML crudo.
         */
        fun humanReadableBody(body: String, statusCode: Int): String {
            val trimmed = body.trim()
            // Detectar HTML por tag de apertura o DOCTYPE
            if (trimmed.startsWith("<") || trimmed.startsWith("<!DOCTYPE", ignoreCase = true)) {
                return when (statusCode) {
                    520, 521, 522, 523, 524, 530 ->
                        "El túnel de Cloudflare no está activo o el origen no responde (error $statusCode)"
                    502, 503, 504 ->
                        "El servidor no está disponible en este momento (error $statusCode)"
                    else ->
                        "Error del servidor ($statusCode)"
                }
            }
            // JSON u otro texto plano — truncar si es muy largo
            return if (trimmed.length > 200) trimmed.take(200) + "…" else trimmed
        }
    }
}
