use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
struct SignalingMessage {
    sdp: String,
    candidates: Vec<String>,
}

pub struct SignalingClient;

impl SignalingClient {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn connect(code: &str, username: &str) -> Result<Self> {
        Ok(Self {})
    }

    pub fn genereate_code(&self) -> String {
        "12345".to_string()
    }
}
