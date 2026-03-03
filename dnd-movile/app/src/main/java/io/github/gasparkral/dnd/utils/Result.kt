package io.github.gasparkral.dnd.utils

interface ErrorMessage
sealed class Result<out T, out E : ErrorMessage> {
    data class Ok<T>(val value: T) : Result<T, Nothing>()
    data class Err<E : ErrorMessage>(val error: E) : Result<Nothing, E>()

    // Métodos principales
    fun isOk(): Boolean = this is Ok
    fun isErr(): Boolean = this is Err

    fun ok(): T? = when (this) {
        is Ok -> value
        is Err -> null
    }

    fun err(): E? = when (this) {
        is Ok -> null
        is Err -> error
    }

    // Transformaciones
    fun <U> map(transform: (T) -> U): Result<U, E> = when (this) {
        is Ok -> Ok(transform(value))
        is Err -> Err(error)
    }

    fun <U> mapOr(default: U, transform: (T) -> U): U = when (this) {
        is Ok -> transform(value)
        is Err -> default
    }

    fun <U> mapOrElse(default: (E) -> U, transform: (T) -> U): U = when (this) {
        is Ok -> transform(value)
        is Err -> default(error)
    }

    fun <F : ErrorMessage> mapErr(transform: (E) -> F): Result<T, F> = when (this) {
        is Ok -> Ok(value)
        is Err -> Err(transform(error))
    }

    // Operaciones de and/or
    fun <U> and(res: Result<U, @UnsafeVariance E>): Result<U, E> = when (this) {
        is Ok -> res
        is Err -> Err(error)
    }

    fun <U> andThen(transform: (T) -> Result<U, @UnsafeVariance E>): Result<U, E> = when (this) {
        is Ok -> transform(value)
        is Err -> Err(error)
    }

    fun <F : ErrorMessage> or(res: Result<@UnsafeVariance T, F>): Result<T, F> = when (this) {
        is Ok -> Ok(value)
        is Err -> res
    }

    fun <F : ErrorMessage> orElse(transform: (E) -> Result<@UnsafeVariance T, F>): Result<T, F> = when (this) {
        is Ok -> Ok(value)
        is Err -> transform(error)
    }

    // Desempaquetado con mensajes de error personalizados
    fun unwrap(): T = when (this) {
        is Ok -> value
        is Err -> throw ResultUnwrapException("Called unwrap on an Err value", error)
    }

    fun unwrapOr(default: @UnsafeVariance T): T = when (this) {
        is Ok -> value
        is Err -> default
    }

    fun unwrapOrElse(transform: (E) -> @UnsafeVariance T): T = when (this) {
        is Ok -> value
        is Err -> transform(error)
    }

    fun expect(message: String): T = when (this) {
        is Ok -> value
        is Err -> throw ResultUnwrapException("$message", error)
    }

    // Pattern matching
    fun <R> fold(onOk: (T) -> R, onErr: (E) -> R): R = when (this) {
        is Ok -> onOk(value)
        is Err -> onErr(error)
    }

    // Utilidades
    fun flatten(): Result<T, E> = when (this) {
        is Ok -> {
            @Suppress("UNCHECKED_CAST")
            if (value is Result<*, *>) (value as Result<T, E>).flatten() else this
        }

        is Err -> this
    }

    // Inspección del error
    fun errorMessage(): String? = when (this) {
        is Ok -> null
        is Err -> error.toString()
    }

    fun errorCode(): Int? = when (this) {
        is Ok -> null
        is Err -> (error as? Exception)?.hashCode() // Ejemplo de metadata adicional
    }

    companion object {
        fun <T, E : ErrorMessage> ok(value: T): Result<T, E> = Ok(value)
        fun <T, E : ErrorMessage> err(error: E): Result<T, E> = Err(error)

        // Try-catch utility que convierte excepciones a ErrorMessage
        fun <T> catch(block: () -> T): Result<T, ErrorMessage> = try {
            Ok(block())
        } catch (e: Exception) {
            Err(e as? ErrorMessage ?: object : ErrorMessage {
                override fun toString(): String = "Caught exception: ${e.message}"
            })
        }

        // Versión con mapeo de excepciones
        fun <T, E : ErrorMessage> catchOrMap(
            mapper: (Exception) -> E,
            block: () -> T
        ): Result<T, E> = try {
            Ok(block())
        } catch (e: Exception) {
            Err(mapper(e))
        }
    }
}

class ResultUnwrapException(
    message: String,
    val error: ErrorMessage
) : Exception("$message - Error: $error")