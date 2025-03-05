use std::net::SocketAddr;

use axum::{Router, extract::ws};
use config::{Config, fetch_config};
use handlers::{room::room_handler, stream::stream_handler};
use tokio::net::TcpListener;
use tracing::info;

mod chat;
mod config;
mod handlers;
mod error;
mod webrtc;
mod room;

#[derive(Debug, Clone)]
pub struct AppState {

}

async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = fetch_config().expect("Failed to fetch configuration");

    let app = Router::new()
        .nest("/room", room_handler())
        .nest("/stream", stream_handler());


    let addr: SocketAddr = config.addr.parse().expect("Invalid address format");

    info!("Running at https://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
