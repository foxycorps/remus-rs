use crate::{Message, MessageFlags, MessageType, ProtocolError};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

/// Extension trait for working with serializable payloads
pub trait MessageExt {
    /// Creates a new request message with a serializable payload
    fn request<T: Serialize>(payload: &T) -> Result<Message, ProtocolError>;
    
    /// Creates a new response message with a serializable payload
    fn response<T: Serialize>(request_id: u32, payload: &T) -> Result<Message, ProtocolError>;
    
    /// Deserializes the payload into the specified type
    fn deserialize<T: DeserializeOwned>(&self) -> Result<T, ProtocolError>;
}

impl MessageExt for Message {
    fn request<T: Serialize>(payload: &T) -> Result<Message, ProtocolError> {
        let bytes = serde_json::to_vec(payload)
            .map_err(|e| ProtocolError::InvalidFormat(e.to_string()))?;
            
        Ok(Message::new(
            MessageType::Request,
            MessageFlags::IDEMPOTENT,
            rand::random(),
            Bytes::from(bytes),
        ))
    }

    fn response<T: Serialize>(request_id: u32, payload: &T) -> Result<Message, ProtocolError> {
        let bytes = serde_json::to_vec(payload)
            .map_err(|e| ProtocolError::InvalidFormat(e.to_string()))?;
            
        Ok(Message::new(
            MessageType::Response,
            MessageFlags::empty(),
            request_id,
            Bytes::from(bytes),
        ))
    }

    fn deserialize<T: DeserializeOwned>(&self) -> Result<T, ProtocolError> {
        serde_json::from_slice(&self.payload)
            .map_err(|e| ProtocolError::InvalidFormat(e.to_string()))
    }
}

/// Example usage:
/// ```rust
/// use remus::MessageExt;
/// 
/// #[derive(Serialize, Deserialize)]
/// struct MyRequest {
///     name: String,
///     value: i32,
/// }
/// 
/// #[derive(Serialize, Deserialize)]
/// struct MyResponse {
///     result: String,
/// }
/// 
/// async fn handle_request(msg: Message) -> Result<Message, ProtocolError> {
///     // Deserialize the request
///     let request: MyRequest = msg.deserialize()?;
///     
///     // Process the request
///     let response = MyResponse {
///         result: format!("Processed {}: {}", request.name, request.value),
///     };
///     
///     // Create and return the response
///     Message::response(msg.request_id, &response)
/// }
/// ``` 