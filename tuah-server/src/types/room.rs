use std::sync::Arc;

use uuid::Uuid;

use super::chat;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: Uuid,
    pub participants: usize,
    pub chat_hub: Arc<chat::hub::Hub>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            id: Uuid::new_v4(),
            participants: 0,
            chat_hub: chat::hub::Hub::new(),
        }
    }
}