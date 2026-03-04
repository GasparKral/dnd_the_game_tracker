// ARCHIVO ELIMINADO — EntityCollisionException ya no existe con Room.
// EntityNotFoundException se mantiene por si acaso se usa en otro lugar.
package io.github.gasparkral.dnd.model.exception

import io.github.gasparkral.dnd.utils.ErrorMessage

class EntityNotFoundException : ErrorMessage, Exception()
