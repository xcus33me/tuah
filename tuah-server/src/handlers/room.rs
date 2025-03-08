use std::sync::Arc;

use axum::{
    extract::{Path, WebSocketUpgrade}, http::Uri, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Extension, Router
};
use uuid::Uuid;

use crate::{error::HttpError, AppState};

pub async fn create_room(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    let room_uuid = Uuid::new_v4();

    let redirect_path = format!("/room/{}", room_uuid);
    let redirect_uri = Uri::try_from(redirect_path).unwrap();

    Ok(Redirect::to(redirect_uri.to_string().as_str()))
}

pub async fn get_room(
    Path(uuid): Path<String>,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, HttpError> {
    if uuid.is_empty() {
        return Err(HttpError::bad_request("UUID is empty".to_string()))
    }

    // let (uuid, suuid, _) = create_or_get_room(uuid);

    Ok("Letsgp".to_string())
}

pub async fn get_room_ws(
    Path(uuid): Path<String>,
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, HttpError> {
    if uuid.is_empty() {
        return Ok(());
    }

    // let (_, _, room) = create_or_get_room(uuid);

    Ok(())
}

pub async fn get_room_chat(Extension(app_state): Extension<Arc<AppState>>) -> Result<impl IntoResponse, HttpError> {
    Ok("Room created".to_string())
}

pub async fn get_room_chat_ws(
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, HttpError> {
    Ok("Room created".to_string())
}

pub async fn get_room_viewer_ws(
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, HttpError> {
    Ok("Room created".to_string())
}
