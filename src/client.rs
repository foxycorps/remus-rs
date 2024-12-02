use crate::{
    Message, MessageFlags, MessageType, ProtocolError,
    compression::compress_if_beneficial,
    discovery::{HealthStatus, ServiceInfo, ServiceRegistry},
    encryption::Encryptor,
    transport::Transport,
};
use bytes::Bytes;
use std::time::Duration;
use tokio::net::TcpStream;

/// High-level client for the Remus protocol
pub struct RemusClient {
    transport: Transport<TcpStream>,
    encryptor: Option<Encryptor>,
    service_registry: ServiceRegistry,
    request_timeout: Duration,
}

impl RemusClient {
    /// Creates a new client with default configuration
    pub async fn connect(address: &str) -> Result<Self, ProtocolError> {
        let stream = TcpStream::connect(address).await?;
        Ok(Self {
            transport: Transport::new(stream),
            encryptor: None,
            service_registry: ServiceRegistry::new(Duration::from_secs(30)),
            request_timeout: Duration::from_secs(30),
        })
    }

    /// Enables encryption for all future communications
    pub fn with_encryption(mut self, key: &[u8; 32]) -> Self {
        self.encryptor = Some(Encryptor::new(key));
        self
    }

    /// Sets the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Sends a request and waits for response
    pub async fn request(&mut self, payload: impl AsRef<[u8]>) -> Result<Bytes, ProtocolError> {
        let payload = self.prepare_payload(payload.as_ref())?;
        
        let request = Message::new(
            MessageType::Request,
            MessageFlags::IDEMPOTENT,
            rand::random(),
            payload,
        );

        self.transport.send(request).await?;

        let response = tokio::time::timeout(
            self.request_timeout,
            self.transport.receive()
        ).await.map_err(|_| ProtocolError::InvalidFormat("Request timeout".into()))??;

        Ok(response.payload)
    }

    /// Creates a streaming request
    pub async fn stream(&mut self, payload: impl AsRef<[u8]>) -> Result<MessageStream, ProtocolError> {
        let payload = self.prepare_payload(payload.as_ref())?;
        let (tx, stream) = MessageStream::new(32);

        let request = Message::new(
            MessageType::Stream,
            MessageFlags::STREAM_END,
            stream.stream_id(),
            payload,
        );

        self.transport.send(request).await?;
        Ok(stream)
    }

    /// Discovers available services
    pub async fn discover_services(&self) -> Result<Vec<ServiceInfo>, ProtocolError> {
        Ok(self.service_registry.get_healthy_services().await)
    }

    // Helper method to prepare payload with compression and encryption
    fn prepare_payload(&self, data: &[u8]) -> Result<Bytes, ProtocolError> {
        let compressed = compress_if_beneficial(data)?;
        if let Some(encryptor) = &self.encryptor {
            encryptor.encrypt(&compressed)
        } else {
            Ok(compressed)
        }
    }
}

/// Example usage in documentation:
/// ```rust,no_run
/// use remus::RemusClient;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Connect to a Remus server
///     let mut client = RemusClient::connect("localhost:8080")
///         .await?
///         .with_encryption(&Encryptor::generate_key());
/// 
///     // Send a simple request
///     let response = client.request("Hello, World!").await?;
///     println!("Got response: {:?}", response);
/// 
///     // Create a stream
///     let mut stream = client.stream("Start streaming").await?;
///     while let Some(msg) = stream.next().await {
///         println!("Got stream message: {:?}", msg);
///     }
/// 
///     // Discover services
///     let services = client.discover_services().await?;
///     for service in services {
///         println!("Found service: {} at {}", service.name, service.address);
///     }
/// 
///     Ok(())
/// }
/// ``` 