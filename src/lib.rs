use bitflags::bitflags;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

// Protocol version constants
pub const PROTOCOL_VERSION_MAJOR: u16 = 2;
pub const PROTOCOL_VERSION_MINOR: u16 = 0;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MessageFlags: u8 {
        const NONE = 0;
        const ENCRYPTED = 0x01;
        const COMPRESSED = 0x02;
        const URGENT = 0x04;
        const REQUIRES_ACK = 0x08;
        const IDEMPOTENT = 0x10;
        const HIGH_PRIORITY = 0x20;
        const REQUIRES_AUTH = 0x40;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
    Event,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub msg_type: MessageType,
    pub flags: MessageFlags,
    pub payload: Bytes,
    pub timestamp: u64,
    pub request_id: u64,
    pub priority: u8,
    pub ttl: u32,
    pub routing_info: Option<String>,
    pub context: Option<String>,
}

impl Message {
    pub fn new(
        msg_type: MessageType,
        flags: MessageFlags,
        request_id: u64,
        payload: Bytes,
    ) -> Self {
        Self {
            msg_type,
            flags,
            payload,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
            request_id,
            priority: 0,
            ttl: 30000, // Default 30 second TTL
            routing_info: None,
            context: None,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        
        // Write message type
        buf.extend_from_slice(&[self.msg_type as u8]);
        
        // Write flags
        buf.extend_from_slice(&[self.flags.bits()]);
        
        // Write timestamp
        buf.extend_from_slice(&self.timestamp.to_be_bytes());
        
        // Write request ID
        buf.extend_from_slice(&self.request_id.to_be_bytes());
        
        // Write priority
        buf.extend_from_slice(&[self.priority]);
        
        // Write TTL
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        
        // Write payload length and payload
        let payload_len = self.payload.len() as u32;
        buf.extend_from_slice(&payload_len.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        
        buf
    }

    pub fn decode(buf: &[u8]) -> Result<Self, ProtocolError> {
        if buf.len() < 24 { // Minimum message size
            return Err(ProtocolError::InvalidFormat("Message too short".into()));
        }

        let mut pos = 0;

        // Read message type
        let msg_type = match buf[pos] {
            0 => MessageType::Request,
            1 => MessageType::Response,
            2 => MessageType::Event,
            3 => MessageType::Error,
            _ => return Err(ProtocolError::InvalidFormat("Invalid message type".into())),
        };
        pos += 1;

        // Read flags
        let flags = MessageFlags::from_bits_truncate(buf[pos]);
        pos += 1;

        // Read timestamp
        let timestamp = u64::from_be_bytes(buf[pos..pos+8].try_into().unwrap());
        pos += 8;

        // Read request ID
        let request_id = u64::from_be_bytes(buf[pos..pos+8].try_into().unwrap());
        pos += 8;

        // Read priority
        let priority = buf[pos];
        pos += 1;

        // Read TTL
        let ttl = u32::from_be_bytes(buf[pos..pos+4].try_into().unwrap());
        pos += 4;

        // Read payload length
        let payload_len = u32::from_be_bytes(buf[pos..pos+4].try_into().unwrap()) as usize;
        pos += 4;

        if buf.len() < pos + payload_len {
            return Err(ProtocolError::InvalidFormat("Invalid payload length".into()));
        }

        // Read payload
        let payload = Bytes::copy_from_slice(&buf[pos..pos+payload_len]);

        Ok(Message {
            msg_type,
            flags,
            payload,
            timestamp,
            request_id,
            priority,
            ttl,
            routing_info: None,
            context: None,
        })
    }
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    #[error("Protocol version mismatch")]
    VersionMismatch,
    #[error("Authentication required")]
    AuthenticationRequired,
    #[error("Compression error: {0}")]
    CompressionError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

// Add to existing lib.rs
pub mod compression;
pub mod discovery;
pub mod edge;
pub mod encryption;
pub mod observability;
pub mod state;
pub mod transport;

// Re-export commonly used types
pub use compression::{compress, decompress};
pub use discovery::{HealthStatus, ServiceInfo, ServiceRegistry};
pub use edge::{EdgeCompute, EdgeComputeResult, EdgeFunction};
pub use encryption::Encryptor;
pub use observability::{Metric, Telemetry, Trace};
pub use state::{StateManager, StateVersion};
pub use transport::Transport;

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_message_flags() {
        let flags = MessageFlags::ENCRYPTED | MessageFlags::COMPRESSED;
        assert!(flags.contains(MessageFlags::ENCRYPTED));
        assert!(flags.contains(MessageFlags::COMPRESSED));
        assert!(!flags.contains(MessageFlags::URGENT));
    }

    #[test]
    fn test_message_encoding_decoding() {
        let original = Message {
            msg_type: MessageType::Request,
            flags: MessageFlags::empty(),
            payload: Bytes::from("test payload"),
            timestamp: 12345,
            request_id: 67890,
            priority: 1,
            ttl: 3600,
            routing_info: None,
            context: None,
        };

        let encoded = original.encode();
        let decoded = Message::decode(&encoded).unwrap();

        assert_eq!(decoded.msg_type, original.msg_type);
        assert_eq!(decoded.flags, original.flags);
        assert_eq!(decoded.payload, original.payload);
        assert_eq!(decoded.timestamp, original.timestamp);
        assert_eq!(decoded.request_id, original.request_id);
        assert_eq!(decoded.priority, original.priority);
        assert_eq!(decoded.ttl, original.ttl);
    }
}
