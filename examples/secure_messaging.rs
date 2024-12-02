use remus::{Message, MessageType, MessageFlags, Transport};
use bytes::Bytes;
use tokio::net::TcpListener;
use std::error::Error;

/// This example demonstrates sending encrypted messages
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start TCP server
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Server listening on 127.0.0.1:8081");

    // Accept connection
    let (stream, _) = listener.accept().await?;
    let mut transport = Transport::new(stream);

    // Create an encrypted message
    let payload = Bytes::from("Secret message!");
    let mut message = Message::new(
        MessageType::Request,
        MessageFlags::ENCRYPTED,
        1, // request_id
        payload,
    );
    
    println!("Sending encrypted message...");
    transport.send(message).await?;

    // Receive a message
    println!("Waiting for message...");
    let received = transport.receive().await?;
    
    if received.flags.contains(MessageFlags::ENCRYPTED) {
        println!("Received encrypted message: {:?}", received);
    }

    Ok(())
}
