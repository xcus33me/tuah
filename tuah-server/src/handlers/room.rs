use std::sync::Arc;

use axum::{
    extract::WebSocketUpgrade, 
    response::IntoResponse, 
    routing::{get, post}, 
    Extension, 
    Router
};

use crate::{error::ErrorResponse, AppState};

pub fn room_handler() -> Router {
    Router::new()
        .route("/create", post(create_room))
        .route("/:uuid", get(get_room))
        .route("/:uuid/websoket", get(get_room_ws))
        .route("/:uuid/chat", get(get_room_chat))
        .route("/:uuid/chat/websocket", get(get_room_chat_ws))
        .route("/:uuid/viewer/websocket", get(get_room_viewer_ws))
}

pub async fn create_room(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Room created".to_string())
}

pub async fn get_room(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Room created".to_string())
}

pub async fn get_room_ws(
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Room created".to_string())
}

pub async fn get_room_chat(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Room created".to_string())
}

pub async fn get_room_chat_ws(
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Room created".to_string())
}

pub async fn get_room_viewer_ws(
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Room created".to_string())
}
