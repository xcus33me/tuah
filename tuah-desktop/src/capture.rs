use anyhow::Result;
use scrap::{
    Capturer,
    Display,
};
use std::{
    io::ErrorKind::WouldBlock,
    time::Duration,
};
use webrtc::peer_connection::RTCPeerConnection;

pub struct ScreenCapture {
    capturer: Capturer,
}

impl ScreenCapture {
    pub fn new() -> Result<Self> {
        let display = Display::primary()?;
        let capturer = Capturer::new(display)?;
        Ok(Self { capturer })
    }

    pub async fn start_stream(&mut self, peer_connection: &RTCPeerConnection) {
        loop {
            match self.capturer.frame() {
                Ok(frame) => {
                    println!("Captured frame: {} bytes", frame.len());
                }
                Err(e) if e.kind() == WouldBlock => {
                    tokio::time::sleep(Duration::from_millis(16)).await;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
        }
    }
}
