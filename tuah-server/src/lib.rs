use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use axum::{Router};
use config::fetch_config;
use routes::create_app_router;
use types::room::Room;
use tracing::info;

mod config;
mod handlers;
mod routes;
mod services;
mod types;
mod error;
mod entities;

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

pub async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = fetch_config().expect("Failed to fetch configuration");
    let app_state = Arc::new(AppState::default());

    let app = create_app_router(app_state);

    let addr: SocketAddr = config.addr.parse().expect("Invalid address format");
    
    info!("Running at https://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}