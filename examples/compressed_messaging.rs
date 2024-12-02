use remus::{Message, MessageType, MessageFlags, Transport};
use bytes::Bytes;
use tokio::net::TcpListener;
use std::error::Error;

/// This example demonstrates automatic message compression
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start TCP server
    let listener = TcpListener::bind("127.0.0.1:8082").await?;
    println!("Server listening on 127.0.0.1:8082");

    // Accept connection
    let (stream, _) = listener.accept().await?;
    let mut transport = Transport::new(stream);

    // Create a message with compressible data (repeated text)
    let payload = Bytes::from("Hello ".repeat(1000));
    let original_size = payload.len();
    
    // Create a compressed message
    let mut message = Message::new(
        MessageType::Request,
        MessageFlags::COMPRESSED,
        1, // request_id
        payload,
    );
    
    println!(
        "Original size: {} bytes, Message size with compression flag: {} bytes",
        original_size,
        message.encode().len()
    );

    println!("Sending compressed message...");
    transport.send(message).await?;

    // Receive message
    println!("Waiting for message...");
    let received = transport.receive().await?;
    println!(
        "Received message is compressed: {}", 
        received.flags.contains(MessageFlags::COMPRESSED)
    );

    Ok(())
}
