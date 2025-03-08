mod room;
mod stream;

use std::sync::Arc;

use axum::{Extension, Router};
use room::room_route;
use stream::stream_route;
use tower_http::trace::TraceLayer;

use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/room", room_route())
        .nest("/stream", stream_route())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_route)
}