# BackgroundWorker Module

## Overview

**HTTP Fetcher** - Periodically fetches data from a remote API.

## Features

- Implements CyberFabric modkit patterns
- Background task execution with cancellation support
- Structured logging with tracing
- Clean architecture with domain/infra separation

## Usage

After adding this module to your workspace members in `Cargo.toml` and wiring it into your application, it will:

1. Initialize when the application starts
2. Spawn a background task that runs independently, fetching data every 5 seconds
3. Handle graceful shutdown via cancellation tokens, waiting for the background task to complete

## Customization

Edit the implementation in `src/module.rs` to customize the behavior.

## Dependencies

- `anyhow` - Error handling
- `cf-modkit` - Module framework
- `cf-modkit-http` - HTTP client
- `tokio` - Async runtime
- `tokio-util` - Cancellation tokens
- `tracing` - Structured logging
- `serde` - Serialize and Deserialize
