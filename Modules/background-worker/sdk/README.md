# {{module_name}}-sdk

Client SDK for the {{module_name}} module.

## Overview

This SDK provides a client trait for interacting with the {{module_name}} module from other modules in your CyberFabric application.

## Usage

Add this SDK as a dependency in modules that need to communicate with {{module_name}}:

```toml
[dependencies]
{{module_name}}-sdk = { path = "../{{module_name}}-sdk" }
```

Then use the client trait:

```rust
use {{crate_name}}_sdk::{{struct_module_name}}Client;

// In your module's code
async fn example(hub: &ClientHub) -> modkit::Result<()> {
    let client = hub.get::<dyn {{struct_module_name}}Client>()?;
    {% if module_type == "http-fetcher" %}
    let data = client.fetch_data().await?;
    {% elsif module_type == "simple-periodic" %}
    client.trigger_task().await?;
    {% else %}
    client.example_operation().await?;
    {% endif %}
    Ok(())
}
```

## Implementation

The {{module_name}} module should implement the `{{struct_module_name}}Client` trait and register itself with the ClientHub.

See the main module's documentation for implementation details.

## Features

- Type-safe client interface
- Async/await support
- {% if module_type == "http-fetcher" %}Re-exports domain types for easy access{% else %}Minimal stub for custom implementation{% endif %}
