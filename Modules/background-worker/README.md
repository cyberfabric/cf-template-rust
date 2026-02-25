# {{module_name}} Module

{{description}}

## Overview

This module was generated from the CyberFabric background-worker template.
**HTTP Fetcher** - Periodically fetches data from `{{http_url}}` every {{fetch_interval_secs}} seconds.

## Features

- Implements CyberFabric modkit patterns
- Background task execution with cancellation support
- Structured logging with tracing
- Clean architecture with domain/infra separation

## Usage

After adding this module to your workspace members in `Cargo.toml` and wiring it into your application, it will:

1. Initialize when the application starts
2. Spawn a background task that runs independently, fetching data every {{fetch_interval_secs}} seconds
3. Handle graceful shutdown via cancellation tokens, waiting for the background task to complete

## Customization

Edit the implementation in `src/module.rs` to customize the behavior.

### Changing the API Endpoint

Update the `base_url` in `src/infra/mod.rs`:

```rust
base_url: "{{http_url}}".to_string()
```

### Changing the Fetch Interval

Update the interval in `src/module.rs`:

```rust
let mut interval = tokio::time::interval(tokio::time::Duration::from_secs({{fetch_interval_secs}}));
```

## Dependencies

- `cf-modkit` - Module framework
- `tokio` - Async runtime
- `tokio-util` - Cancellation tokens
- `tracing` - Structured logging
- `cf-modkit-http` - HTTP client
- `serde` - JSON handling
- `anyhow` - Error handling
