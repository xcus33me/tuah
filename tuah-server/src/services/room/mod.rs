use std::sync::Arc;

use redis::RedisResult;
use webrtc::turn::client;

use crate::{handlers::room, types::room::Room};

pub struct RoomService {
    pub client: Arc<redis::Client>,
}   

impl RoomService {
    pub fn from_client(client: redis::Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn create_or_get_room(uuid: String) -> RedisResult<Room> {
        let mut con = self.client.get_async_connection().await?;
        let key = format!("room:{}", uuid);

        let room_json: Option<String> = con.get(&key).await?;

        match room_json {
            Some(json) => {

            }
            None => {
                let room = Room::new();
                let roomjson = serde_json::to_string(&room)?;
                con.set(&key, room_json).await?;
                Ok(room)
            }
        }
    }

    pub async fn delete_room(uuid: String) {

    }
}