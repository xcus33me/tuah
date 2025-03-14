use std::sync::Arc;

use sea_orm::entity::prelude::*;
use uuid::Uuid;

use super::chat;

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