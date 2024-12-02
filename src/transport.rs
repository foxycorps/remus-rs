use crate::{Message, ProtocolError};
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub struct Transport<T> {
    inner: T,
    read_buf: BytesMut,
    write_buf: BytesMut,
}

impl<T: AsyncRead + AsyncWrite + Unpin> Transport<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            read_buf: BytesMut::with_capacity(8 * 1024),
            write_buf: BytesMut::with_capacity(8 * 1024),
        }
    }

    pub async fn send(&mut self, message: Message) -> Result<(), ProtocolError> {
        let encoded = message.encode();
        let len = encoded.len() as u32;
        
        // Write length prefix
        self.write_buf.put_u32(len);
        self.write_buf.extend_from_slice(&encoded);
        
        // Write to underlying transport
        while !self.write_buf.is_empty() {
            let bytes_written = self.inner.write(&self.write_buf).await?;
            self.write_buf.advance(bytes_written);
        }
        
        self.inner.flush().await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Message, ProtocolError> {
        loop {
            // Try to read the length prefix
            if self.read_buf.len() < 4 {
                let bytes_read = self.inner.read_buf(&mut self.read_buf).await?;
                if bytes_read == 0 {
                    return Err(ProtocolError::InvalidFormat("Connection closed".into()));
                }
                continue;
            }

            // Parse message length
            let len = (&self.read_buf[..4]).get_u32() as usize;
            
            // Wait for complete message
            if self.read_buf.len() < 4 + len {
                let bytes_read = self.inner.read_buf(&mut self.read_buf).await?;
                if bytes_read == 0 {
                    return Err(ProtocolError::InvalidFormat("Connection closed".into()));
                }
                continue;
            }

            // We have a complete message
            self.read_buf.advance(4); // Skip length prefix
            let message_data = self.read_buf.split_to(len);
            return Message::decode(&message_data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MessageType;
    use tokio::io::duplex;

    #[tokio::test]
    async fn test_transport_send_receive() {
        let (client, server) = duplex(1024);
        let mut client_transport = Transport::new(client);
        let mut server_transport = Transport::new(server);

        // Create test message
        let test_msg = Message {
            msg_type: MessageType::Request,
            flags: crate::MessageFlags::empty(),
            payload: bytes::Bytes::from("Hello, World!"),
            timestamp: 12345,
            request_id: 67890,
            priority: 1,
            ttl: 3600,
            routing_info: None,
            context: None,
        };

        // Send from client to server
        tokio::spawn(async move {
            client_transport.send(test_msg).await.unwrap();
        });

        // Receive on server with timeout
        let received = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            server_transport.receive()
        ).await.unwrap().unwrap();

        assert_eq!(received.msg_type, MessageType::Request);
        assert_eq!(received.payload, bytes::Bytes::from("Hello, World!"));
    }

    #[tokio::test]
    async fn test_transport_backpressure() {
        let (client, server) = duplex(64); // Small buffer to test backpressure
        let mut client_transport = Transport::new(client);
        let mut server_transport = Transport::new(server);

        // Create large message
        let test_msg = Message {
            msg_type: MessageType::Request,
            flags: crate::MessageFlags::empty(),
            payload: bytes::Bytes::from(vec![0u8; 1024]),
            timestamp: 12345,
            request_id: 67890,
            priority: 1,
            ttl: 3600,
            routing_info: None,
            context: None,
        };

        // Send in background task
        let send_task = tokio::spawn(async move {
            client_transport.send(test_msg).await.unwrap();
        });

        // Receive in background task
        let receive_task = tokio::spawn(async move {
            server_transport.receive().await.unwrap();
        });

        // Wait for both tasks with timeout
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            futures::future::join_all(vec![send_task, receive_task])
        ).await.unwrap();
    }
}