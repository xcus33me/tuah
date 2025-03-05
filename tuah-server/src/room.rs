use std::sync::Arc;

use crate::chat;

#[derive(Debug, Clone)]
pub struct RoomState {
    participants: usize,
    chat_hub: Arc<chat::hub::Hub>,
}

impl RoomState {
    pub fn new() -> Self {
        RoomState {
            participants: 0,
            chat_hub: chat::hub::Hub::new(),
        }
    }
}