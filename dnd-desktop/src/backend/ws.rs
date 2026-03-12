use super::models::messages::*;
use crate::states::{AppState, SharedState};
use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub async fn handler(
    State(state): State<SharedState>,
    ws: WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.0))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let player_id = Uuid::new_v4();
    let (mut tx, mut rx) = socket.split();

    // --- Handshake: esperar identificación del jugador ---
    let player_name = match rx.next().await {
        Some(Ok(Message::Text(text))) => {
            info!("Handshake recieve");
            match serde_json::from_str::<ClientMessage>(&text) {
                Ok(ClientMessage::Join {
                    player_name,
                    character_name,
                }) => {
                    tracing::info!("Jugador conectado: {} ({})", player_name, character_name);

                    // Registrar en el pool
                    state.ws_pool.add(player_id, character_name).await;

                    // Suscribirse al broadcast antes de confirmar
                    player_name
                }
                _ => {
                    tracing::warn!("Handshake inválido de {}", player_id);
                    return;
                }
            }
        }
        _ => return,
    };

    //Confirmar conexión al jugador
    let welcome = ServerMessage::Welcome {
        player_id,
        player_name: player_name.clone(),
    };
    let json = serde_json::to_string(&welcome).unwrap();
    if tx.send(Message::Text(json.into())).await.is_err() {
        state.ws_pool.remove(&player_id).await;
        return;
    }

    // Suscripción al canal broadcast
    let mut broadcast_rx = state.ws_pool.subscribe();

    // --- Loop principal ---
    loop {
        tokio::select! {
            // Mensajes broadcast → reenviar al jugador
            Ok(msg) = broadcast_rx.recv() => {
                let json = serde_json::to_string(&msg).unwrap();
                if tx.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }

            // Mensajes entrantes del jugador → procesar
            Some(result) = rx.next() => {
                match result {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(msg) => handle_client_message(msg, player_id, &state).await,
                            Err(e) => tracing::warn!("Mensaje inválido de {}: {}", player_id, e),
                        }
                    }
                    Ok(Message::Close(_)) | Err(_) => break,
                    _ => {}
                }
            }

            else => break,
        }
    }

    // Limpieza
    state.ws_pool.remove(&player_id).await;
    tracing::info!("Jugador {} desconectado", player_id);
}

async fn handle_client_message(msg: ClientMessage, player_id: Uuid, state: &Arc<AppState>) {
    match msg {
        ClientMessage::RollDice { roll_result } => {
            tracing::info!(
                "Jugador {} tiró {}: total={}",
                player_id,
                roll_result.request.label.as_deref().unwrap_or("sin etiqueta"),
                roll_result.total,
            );
            state.ws_pool.broadcast(ServerMessage::DiceRoll { player_id, roll_result });
        }
        ClientMessage::RequestSync => {
            tracing::info!("Jugador {} solicita sincronización", player_id);
        }
        ClientMessage::InventoryUpdated { character_id } => {
            tracing::info!("Jugador {} actualizó inventario de {}", player_id, character_id);
            // Re-emitir como ServerMessage para que el DM (si está suscrito) recargue
            state.ws_pool.broadcast(ServerMessage::InventoryChanged { character_id });
        }
        _ => {}
    }
}
