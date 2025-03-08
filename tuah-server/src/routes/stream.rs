use axum::{routing::get, Router};

use crate::handlers::stream::{get_stream, get_stream_ws};

pub fn stream_route() -> Router {
    Router::new()
        .route("/{ssuid}", get(get_stream))
        .route("/{ssuid}/websocket", get(get_stream_ws))
}