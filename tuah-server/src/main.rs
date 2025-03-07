use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{Extension, Router};
use config::fetch_config;
use futures::lock::Mutex;
use handlers::{room::room_handler, stream::stream_handler};
use room::Room;
use tower_http::trace::TraceLayer;
use tracing::info;

mod chat;
mod config;
mod handlers;
mod error;
mod webrtc;
mod room;
mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    rooms: Arc<Mutex<HashMap<String, Room>>>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = fetch_config().expect("Failed to fetch configuration");

    let app_state = Arc::new(AppState::default());

    let app = Router::new()
        .nest("/room", room_handler())
        .nest("/stream", stream_handler())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));


    let addr: SocketAddr = config.addr.parse().expect("Invalid address format");

    info!("Running at https://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    run().await;
}
