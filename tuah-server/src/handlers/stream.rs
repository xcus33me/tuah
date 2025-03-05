use std::sync::Arc;

use axum::{extract::{ws, State}, response::IntoResponse, routing::get, Extension, Router};

use crate::{error::{ErrorResponse, HttpError}, AppState};

pub fn stream_handler() -> Router {
    Router::new()
        .route("/:ssuid", get(get_stream))
        .route("/:ssuid/websocket", get(get_stream_ws))
}

async fn get_stream(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Stream".to_string())
}

async fn get_stream_ws(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, ErrorResponse> {
    Ok("Stream WebSocket".to_string())
}
