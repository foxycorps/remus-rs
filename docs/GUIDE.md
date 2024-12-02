# Remus Developer Guide

This guide provides detailed information about using the Remus message library effectively.

## Table of Contents
- [Message Types](#message-types)
- [Message Flags](#message-flags)
- [Transport Layer](#transport-layer)
- [Compression](#compression)
- [Encryption](#encryption)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Message Types

Remus supports different message types for various purposes:

```rust
pub enum MessageType {
    Data,      // Regular data messages
    Control,   // Control messages for transport management
    Heartbeat, // Keep-alive messages
    Error,     // Error notifications
}
```

## Message Flags

Messages can have different flags set to indicate their properties:

- `COMPRESSED` (0x01): Message payload is compressed
- `ENCRYPTED` (0x02): Message payload is encrypted
- `FRAGMENTED` (0x04): Message is part of a larger message

Example of setting flags:
```rust
let mut msg = Message::new(MessageType::Data, payload);
msg.set_compressed(true);
msg.set_encrypted(true);
```

## Transport Layer

The transport layer handles message framing and transmission. It supports:

- Automatic message framing
- Backpressure handling
- Async send/receive operations

Example of creating a transport:
```rust
// TCP transport
let stream = TcpStream::connect("127.0.0.1:8080").await?;
let transport = Transport::new(stream).await?;

// Custom transport
struct MyTransport {
    // your transport implementation
}

impl Transport for MyTransport {
    async fn send(&self, message: Message) -> Result<(), Error> {
        // implementation
    }
    
    async fn receive(&self) -> Result<Message, Error> {
        // implementation
    }
}
```

## Compression

Remus uses zstd compression with automatic threshold detection:

```rust
// Automatic compression if beneficial
let compressed = compress_if_beneficial(&data)?;

// Manual compression control
let mut msg = Message::new(MessageType::Data, payload);
if data.len() > 1024 {  // Only compress larger messages
    msg.set_compressed(true);
}
```

## Encryption

Remus uses AES-GCM for encryption:

```rust
// Generate a new key
let key = generate_key();

// Encrypt data
let encrypted = encrypt(&key, &data)?;

// Decrypt data
let decrypted = decrypt(&key, &encrypted)?;
```

## Error Handling

Remus provides detailed error types:

```rust
pub enum Error {
    Transport(TransportError),
    Compression(CompressionError),
    Encryption(EncryptionError),
    InvalidMessage(String),
}
```

Best practices for error handling:
```rust
match transport.receive().await {
    Ok(msg) => {
        // Handle message
    }
    Err(Error::Transport(e)) => {
        // Handle transport error
    }
    Err(Error::InvalidMessage(e)) => {
        // Handle invalid message
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Best Practices

1. **Message Size**
   - Keep messages under 1MB when possible
   - Use compression for large messages
   - Consider message fragmentation for very large data

2. **Performance**
   - Reuse Transport instances when possible
   - Use appropriate buffer sizes
   - Enable compression only when beneficial

3. **Security**
   - Always use encryption for sensitive data
   - Rotate encryption keys regularly
   - Validate message integrity

4. **Error Handling**
   - Implement proper error recovery
   - Log transport errors for debugging
   - Handle connection failures gracefully

5. **Testing**
   - Test with different message sizes
   - Verify compression ratios
   - Test network failures
   - Validate encryption/decryption
