# {{module_name}} Module

{{description}}

## Overview

This module was generated from the CyberFabric background-worker template.
{% if module_type == "simple-periodic" %}**Simple Periodic Task** - Executes a task every {{task_interval_secs}} seconds.
{% elsif module_type == "http-fetcher" %}**HTTP Fetcher** - Periodically fetches data from `{{http_url}}` every {{fetch_interval_secs}} seconds.
{% else %}**Custom Module** - Implement your custom logic.
{% endif %}

## Features

- Implements CyberFabric modkit patterns
- Background task execution with cancellation support
- Structured logging with tracing
- Clean architecture{% if module_type == "http-fetcher" %} with domain/infra separation{% endif %}

## Usage

After adding this module to your workspace members in `Cargo.toml` and wiring it into your application, it will:

1. Initialize when the application starts
2. Spawn a background task that runs independently{% if module_type == "simple-periodic" %}, executing every {{task_interval_secs}} seconds{% elsif module_type == "http-fetcher" %}, fetching data every {{fetch_interval_secs}} seconds{% endif %}
3. Handle graceful shutdown via cancellation tokens, waiting for the background task to complete

## Customization

Edit the implementation in `src/module.rs` to customize the behavior.

{% if module_type == "http-fetcher" %}### Changing the API Endpoint

Update the `base_url` in `src/infra/mod.rs`:

```rust
base_url: "{{http_url}}".to_string()
```

### Changing the Fetch Interval

Update the interval in `src/module.rs`:

```rust
let mut interval = tokio::time::interval(tokio::time::Duration::from_secs({{fetch_interval_secs}}));
```
{% endif %}

{% if module_type == "simple-periodic" %}### Changing the Task Interval

Update the interval in `src/module.rs`:

```rust
_ = tokio::time::sleep(tokio::time::Duration::from_secs({{task_interval_secs}}))
```
{% endif %}

## Dependencies

- `cf-modkit` - Module framework
- `tokio` - Async runtime
- `tokio-util` - Cancellation tokens
- `tracing` - Structured logging{% if module_type == "http-fetcher" %}
- `cf-modkit-http` - HTTP client
- `serde` - JSON handling
- `anyhow` - Error handling{% endif %}
