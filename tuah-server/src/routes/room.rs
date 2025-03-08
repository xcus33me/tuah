use axum::{routing::{get, post}, Router};

use crate::handlers::room::{create_room, get_room, get_room_chat, get_room_chat_ws, get_room_viewer_ws, get_room_ws};

pub fn room_route() -> Router {
    Router::new()
        .route("/create", post(create_room))
        .route("/{uuid}", get(get_room))
        .route("/{uuid}/websocket", get(get_room_ws))
        .route("/{uuid}/chat", get(get_room_chat))
        .route("/{uuid}/chat/websocket", get(get_room_chat_ws))
        .route("/{uuid}/viewer/websocket", get(get_room_viewer_ws))
}