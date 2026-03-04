package io.github.gasparkral.dnd.infra

import io.github.gasparkral.dnd.utils.Result
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*
import kotlinx.serialization.SerializationException

// Alias de conveniencia para no repetir el tipo largo en cada ViewModel/Service
typealias HttpResult<T> = Result<T, HttpError>

object HttpManager {

    lateinit var baseUrl: String
    lateinit var client: HttpClient

    fun init(baseUrl: String, client: HttpClient) {
        this.baseUrl = baseUrl
        this.client = client
    }

    fun isInitialized(): Boolean =
        ::baseUrl.isInitialized && ::client.isInitialized

    // ── GET ──────────────────────────────────────────────────────────────────

    suspend inline fun <reified T> get(
        endpoint: String,
        headers: Map<String, String> = emptyMap(),
        queryParams: Map<String, String> = emptyMap(),
    ): HttpResult<T> = safeRequest {
        client.get("$baseUrl$endpoint") {
            headers.forEach { (k, v) -> header(k, v) }
            queryParams.forEach { (k, v) -> parameter(k, v) }
        }
    }

    // ── POST ─────────────────────────────────────────────────────────────────

    suspend inline fun <reified B, reified T> post(
        endpoint: String,
        body: B,
        headers: Map<String, String> = emptyMap(),
    ): HttpResult<T> = safeRequest {
        client.post("$baseUrl$endpoint") {
            contentType(ContentType.Application.Json)
            setBody(body)
            headers.forEach { (k, v) -> header(k, v) }
        }
    }

    // ── PUT ──────────────────────────────────────────────────────────────────

    suspend inline fun <reified B, reified T> put(
        endpoint: String,
        body: B,
        headers: Map<String, String> = emptyMap(),
    ): HttpResult<T> = safeRequest {
        client.put("$baseUrl$endpoint") {
            contentType(ContentType.Application.Json)
            setBody(body)
            headers.forEach { (k, v) -> header(k, v) }
        }
    }

    // ── PATCH ────────────────────────────────────────────────────────────────

    suspend inline fun <reified B, reified T> patch(
        endpoint: String,
        body: B,
        headers: Map<String, String> = emptyMap(),
    ): HttpResult<T> = safeRequest {
        client.patch("$baseUrl$endpoint") {
            contentType(ContentType.Application.Json)
            setBody(body)
            headers.forEach { (k, v) -> header(k, v) }
        }
    }

    // ── DELETE ───────────────────────────────────────────────────────────────

    suspend inline fun <reified T> delete(
        endpoint: String,
        headers: Map<String, String> = emptyMap(),
    ): HttpResult<T> = safeRequest {
        client.delete("$baseUrl$endpoint") {
            headers.forEach { (k, v) -> header(k, v) }
        }
    }

    // ── Helper interno ───────────────────────────────────────────────────────

    suspend inline fun <reified T> safeRequest(
        crossinline block: suspend () -> HttpResponse,
    ): HttpResult<T> = try {
        val response = block()
        if (response.status.isSuccess()) {
            Result.Ok(response.body<T>())
        } else {
            Result.Err(HttpError.ApiError(response.status, response.bodyAsText()))
        }
    } catch (e: SerializationException) {
        Result.Err(HttpError.ParseError(e.message ?: "Deserialization failed"))
    } catch (e: java.net.ConnectException) {
        Result.Err(HttpError.NetworkError(e.message ?: "Connection refused"))
    } catch (e: java.net.SocketTimeoutException) {
        Result.Err(HttpError.NetworkError("Timeout"))
    } catch (e: Exception) {
        Result.Err(HttpError.NetworkError(e.message ?: "Unknown network error"))
    }
}
