# {{project-name}}-sdk

Client SDK for the {{module_name}} module.

## Overview

This SDK provides a client trait for interacting with the {{module_name}} module from other modules in your CyberFabric application.

## Usage

Add this SDK as a dependency in modules that need to communicate with {{module_name}}:

```toml
[dependencies]
{{project-name}}-sdk = { path = "../{{project-name}}-sdk" }
```

Then use the client trait:

```rust
use {{crate_name}}_sdk::{{struct_module_name}}Client;

// In your module's code
async fn example(hub: &ClientHub) -> modkit::Result<()> {
    let client = hub.get::<dyn {{struct_module_name}}Client>()?;
    let data = client.fetch_data().await?;
    Ok(())
}
```

## Implementation

The {{module_name}} module should implement the `{{struct_module_name}}Client` trait and register itself with the ClientHub.

See the main module's documentation for implementation details.

## Features

- Type-safe client interface
- Async/await support
- Re-exports domain types for easy access
