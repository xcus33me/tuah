use std::sync::Arc;

use axum::{response::IntoResponse, Extension};

use crate::{error::HttpError, AppState};

pub async fn get_stream(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    Ok("Stream".to_string())
}

pub async fn get_stream_ws(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    Ok("Stream WebSocket".to_string())
}
