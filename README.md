# Retro Chat App

A simple command-line chat application built with Rust and Tokio.

## Features

- Server-client communication
- Message broadcasting to all connected clients
- Basic chat message structure with username, content, timestamp, and message type.

## Getting Started

### Prerequisites

- Rust programming language (install via [rustup](https://rustup.rs/))

### Building the Project

Navigate to the `retro-chat` directory and build the project using Cargo:

```bash
cargo build
```

### Running the Server

From the `retro-chat` directory, run the server:

```bash
cargo run --bin server
```

The server will listen on `127.0.0.1:8082`.

### Running the Client

From the `retro-chat` directory, run the client:

```bash
cargo run --bin client <username>
```

You can open multiple client instances to test the chat functionality.

## Project Structure

- `Cargo.toml`: Defines project dependencies and metadata.
- `src/bin/server.rs`: Contains the server-side logic for handling connections and broadcasting messages.
- `src/bin/client.rs`: Contains the client-side logic for connecting to the server and sending/receiving messages.

## Dependencies

- `tokio`: Asynchronous runtime for Rust.
- `serde`: Serialization/deserialization framework.
- `serde_json`: JSON serialization/deserialization.
- `chrono`: Date and time library.

## License

This project is licensed under the MIT License - see the LICENSE file for details (if applicable).