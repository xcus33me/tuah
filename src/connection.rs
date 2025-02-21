use std::sync::Arc;

use anyhow::Result;
use webrtc::{api::APIBuilder, peer_connection::RTCPeerConnection, rtp::codecs, rtp_transceiver::rtp_codec::RTCRtpCodecParameters, track::track_local::{track_local_static_rtp::TrackLocalStaticRTP, TrackLocal}};

use crate::{capture::ScreenCapture, signaling::SignalingClient};

pub struct WebRTCManager {
    peer_connection: RTCPeerConnection,
    signaling: SignalingClient,
    screen_capture: Option<ScreenCapture>,
    connection_code: String,
}

impl WebRTCManager {
    pub fn new_host() -> Result<Self> {
        let api = APIBuilder::new().build();
        let config = webrtc::peer_connection::configuration::RTCConfiguration::default();
        let peer_connection = tokio::runtime::Runtime::new()?.block_on(api.new_peer_connection(config))?;
        
        let signaling = SignalingClient::new()?;
        let connection_code = signaling.genereate_code();

        let screen_capture = ScreenCapture::new()?;

        let codec_capability = webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability {
            mime_type: "video/h264".to_string(),
            clock_rate: 90000,
            channels: 0, 
            sdp_fmtp_line: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42001f".to_string(),
            ..Default::default()
        };

        /*let codec = RTCRtpCodecParameters {
            capability: webrtc::rtp_transceiver::rtp_codec::RtpCodecCapability {
                mime_type: "video/h264".to_string(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: Some("level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42001f".to_string()),
                ..Default::default()
            },
            payload_type: 102,
            ..Default::default()
        };*/

        let video_track = TrackLocalStaticRTP::new(
            codec_capability,
            "video0".to_string(),
            "stream0".to_string(),
        );

        peer_connection
            .add_track(video_track.clone() as Arc<dyn TrackLocal + Send + Sync>)
            .await?;

        tokio::spawn(async move {
            screen_capture.start_stream(&peer_connection).await;
        });

        Ok(Self {
            peer_connection,
            signaling,
            screen_capture: Some(screen_capture),
            connection_code,
        })
    }

    pub fn new_client(code: &str, username: &str) -> Result<Self> {
        let api = APIBuilder::new().build();
        let config = webrtc::peer_connection::configuration::RTCConfiguration::default();
        let peer_connection = tokio::runtime::Runtime::new()?.block_on(api.new_peer_connection(config))?;
        let signaling = SignalingClient::connect(code, username)?;

        Ok(Self {
            peer_connection,
            signaling,
            screen_capture: None,
            connection_code: code.to_string(),
        })
    }

    pub fn get_connection_code(&self) -> String {
        self.connection_code.clone()
    }
}