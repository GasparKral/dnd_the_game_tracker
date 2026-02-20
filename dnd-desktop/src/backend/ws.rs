use super::messages::*;
use axum::extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use uuid::Uuid;

use crate::states::AppState;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let player_id = Uuid::new_v4();
    let (mut tx, mut rx) = socket.split();

    // --- Handshake: esperar identificación del jugador ---
    let player_name = match rx.next().await {
        Some(Ok(Message::Text(text))) => {
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

    // Confirmar conexión al jugador
    // let welcome = ServerMessage::Welcome {
    //     player_id,
    //     player_name: player_name.clone(),
    // };
    // if tx
    //     .send(Message::Text(serde_json::to_string(&welcome).unwrap()))
    //     .await
    //     .is_err()
    // {
    //     return;
    // }

    // Suscripción al canal broadcast
    let mut broadcast_rx = state.ws_pool.subscribe();

    // --- Loop principal ---
    loop {
        tokio::select! {
            // Mensajes broadcast → reenviar al jugador
          //  Ok(msg) = broadcast_rx.recv() => {
          //      let json = serde_json::to_string(&msg).unwrap();
          //      if tx.send(Message::Text(json)).await.is_err() {
          //          break;
          //      }
          //  }

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
        ClientMessage::RollDice { dice, result } => {
            tracing::info!("Jugador {} tiró {}: {}", player_id, dice, result);
            // Broadcast a todos para que vean la tirada
            //state
            //    .ws_pool
            //    .broadcast(shared::messages::ServerMessage::DiceRoll {
            //        player_id,
            //        dice,
            //        result,
            //    });
        }
        ClientMessage::RequestSync => {
            // El jugador pide su estado actual (reconexión)
            // El DM tendrá que confirmar - por ahora solo logueamos
            tracing::info!("Jugador {} solicita sincronización", player_id);
        }
        _ => {}
    }
}
