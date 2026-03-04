package io.github.gasparkral.dnd.infra

import io.github.gasparkral.dnd.utils.ErrorMessage
import io.ktor.http.HttpStatusCode

sealed class HttpError : ErrorMessage {

    /** El servidor respondió pero con un código de error (4xx, 5xx). */
    data class ApiError(
        val status: HttpStatusCode,
        val body: String,
    ) : HttpError() {
        override fun toString() = "ApiError(${status.value} ${status.description}): $body"
    }

    /** No se pudo alcanzar el servidor (sin red, timeout, DNS...). */
    data class NetworkError(val cause: String) : HttpError() {
        override fun toString() = "NetworkError: $cause"
    }

    /** La respuesta llegó pero no se pudo deserializar al tipo esperado. */
    data class ParseError(val cause: String) : HttpError() {
        override fun toString() = "ParseError: $cause"
    }
}
