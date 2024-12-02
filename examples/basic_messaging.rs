use remus::{Message, MessageType, MessageFlags, Transport};
use bytes::Bytes;
use tokio::net::TcpListener;
use std::error::Error;

/// This example demonstrates basic message sending and receiving using TCP transport
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start a TCP server
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    // Accept a connection
    let (stream, _) = listener.accept().await?;
    let mut transport = Transport::new(stream);

    // Create and send a message
    let payload = Bytes::from("Hello from Remus!");
    let message = Message::new(
        MessageType::Request,
        MessageFlags::NONE,
        1, // request_id
        payload,
    );
    
    println!("Sending message...");
    transport.send(message).await?;

    // Receive a message
    println!("Waiting for message...");
    let received = transport.receive().await?;
    println!("Received message: {:?}", received);

    Ok(())
}
