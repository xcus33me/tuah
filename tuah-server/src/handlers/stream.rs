use axum::{extract::ws, response::IntoResponse, routing::get, Router};

use crate::error::{ErrorResonse, HttpError};

pub fn stream_handler -> Router {
    Router::new()
        .route("/:ssuid", get(get_stream))
        .route("/:ssuid/websocket", get(get_stream_ws));
}

async fn get_stream() -> Result<impl IntoResponse, ErrorResonse> {}

async fn get_stream_ws() -> Result<impl IntoResponse, ErrorResonse> {}
