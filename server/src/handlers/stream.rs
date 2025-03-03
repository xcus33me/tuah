use axum::Router;

pub fn stream_handler -> Router {
    Router::new()
        .route("/:ssuid",)
        .route("/:ssuid/websocket")
}
