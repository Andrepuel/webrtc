pub mod dtls_transport;
pub mod ice_transport;
pub mod interceptor;
pub mod rtp;
pub mod track;

use bytes::Bytes;
use tokio::time::{Duration, Instant};

/// A Sample contains encoded media and timing information
pub struct Sample {
    pub data: Bytes,
    pub timestamp: Instant,
    pub duration: Duration,
    pub packet_timestamp: u32,
    pub prev_dropped_packets: u16,
}

impl Default for Sample {
    fn default() -> Self {
        Sample {
            data: Bytes::new(),
            timestamp: Instant::now(),
            duration: Duration::from_secs(0),
            packet_timestamp: 0,
            prev_dropped_packets: 0,
        }
    }
}

/*
// Writer defines an interface to handle
// the creation of media files
type Writer interface {
    // Add the content of an RTP packet to the media
    write_rtp(packet *rtp.Packet) error
    // Close the media
    // Note: Close implementation must be idempotent
    Close() error
}
*/
