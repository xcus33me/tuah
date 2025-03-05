use axum::{
    Router,
    extract::{WebSocketUpgrade, ws::WebSocket},
    http::Response,
    response::IntoResponse,
    routing::{get, post},
};

use crate::error::ErrorResonse;

pub fn room_handler() -> Router {
    Router::new()
        .route("/create", post(create_room))
        .route("/:uuid", get(get_room))
        .route("/:uuid/webscoket", get(get_room_ws))
        .route("/:uuid/chat", get(get_room_chat))
        .route("/:uuid/chat/websocket", get(get_room_chat_ws))
        .route("/:uuid/viewer/websocket", get(get_room_viewer_ws))
}

async fn create_room() -> Result<impl IntoResponse, ErrorResonse> {
    Ok("Room created".to_string())
}

async fn get_room() -> Result<impl IntoResponse, ErrorResonse> {
    Ok("Room created".to_string())
}

async fn get_room_ws() -> Result<impl IntoResponse, ErrorResonse> {
    Ok(WebSocketUpgrade::on_upgrade(|socket: WebSocket| async {
        println!("WebSocket connected for room")
    }))
}

async fn get_room_chat() -> Result<impl IntoResponse, ErrorResonse> {
    Ok("Room created".to_string())
}

async fn get_room_chat_ws() -> Result<impl IntoResponse, ErrorResonse> {
    Ok("Room created".to_string())
}

async fn get_room_viewer_ws() -> Result<impl IntoResponse, ErrorResonse> {
    Ok("Room created".to_string())
}
