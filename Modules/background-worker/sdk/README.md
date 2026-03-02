# background-worker-sdk

## Overview

This SDK provides a client trait for interacting with the background-worker module from other modules in your CyberFabric application.

## Usage

Add this SDK as a dependency in modules that need to communicate with background-worker.

Then use the client trait:

```rust
use background_worker_sdk::BackgroundWorkerClient;

// In your module's code
async fn example(hub: &ClientHub) -> modkit::Result<()> {
    let client = hub.get::<dyn BackgroundWorkerClient>()?;
    let data = client.fetch_data().await?;
    Ok(())
}
```

## Implementation

The background-worker should implement the `BackgroundWorkerClient` trait and register itself with the ClientHub.

See the main module's documentation for implementation details.
