# Remus

A Rust library for efficient message handling with built-in compression, encryption, and transport capabilities.

## Features

- **Message Framing**: Efficient binary message format with flags and type support
- **Encryption**: Built-in AES-GCM encryption for secure message transport
- **Compression**: Automatic zstd compression when beneficial
- **Async Transport**: Asynchronous message sending and receiving with backpressure handling
- **Zero-Copy**: Utilizes `bytes::Bytes` for efficient memory usage

## Documentation

- [Developer Guide](./docs/GUIDE.md) - Detailed usage guide and best practices
- [Protocol Specification](./PROTOCOL.md) - Technical details of the message protocol
- [API Documentation](https://docs.rs/remus) - Full API documentation
- [Examples](./examples) - Working code examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
remus = "0.1.0"
```

## Quick Start

```rust
use remus::{Message, MessageType, MessageFlags, Transport};
use bytes::Bytes;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a message
    let payload = Bytes::from("Hello, Remus!");
    let message = Message::new(
        MessageType::Request,
        MessageFlags::NONE,
        1, // request_id
        payload,
    );

    // Set up transport (example with TCP)
    let mut transport = Transport::new(stream);
    
    // Send message
    transport.send(message).await?;
    
    // Receive message
    let received = transport.receive().await?;
    println!("Received message: {:?}", received);
    
    Ok(())
}
```

## Examples

Run the examples using cargo:

```bash
# Basic messaging example
cargo run --example basic_messaging

# Secure messaging with encryption
cargo run --example secure_messaging

# Compression example
cargo run --example compressed_messaging
```

## Core Components

### Message

The `Message` struct is the core type for handling data:

```rust
let msg = Message::new(
    MessageType::Request,
    MessageFlags::ENCRYPTED | MessageFlags::COMPRESSED,
    request_id,
    payload,
);
```

### Transport

The `Transport` trait provides async message sending and receiving:

```rust
let mut transport = Transport::new(stream);
transport.send(message).await?;
let received = transport.receive().await?;
```

For more detailed examples and best practices, check out our [Developer Guide](./docs/GUIDE.md).

## Contributing

We welcome contributions! Here's how you can help:

1. Check out our [issues](https://github.com/yourusername/remus/issues)
2. Fork the repository
3. Create a feature branch
4. Add your changes
5. Run tests: `cargo test`
6. Submit a pull request

Please make sure to update tests and documentation as needed.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Support

- [Documentation](./docs/GUIDE.md)
- [Discussions](https://github.com/yourusername/remus/discussions)
- [Bug Reports](https://github.com/yourusername/remus/issues)
