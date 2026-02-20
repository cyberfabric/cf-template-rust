//! # {{module_name}} SDK
//!
//! Client SDK for interacting with the {{module_name}} module.
//!
//! ## Usage
//!
//! ```ignore
//! use {{crate_name}}_sdk::{{struct_module_name}}Client;
//! 
//! // Get client from ClientHub
//! let client = hub.get::<dyn {{struct_module_name}}Client>()?;
//! ```

{% if module_type == "http-fetcher" %}
// Re-export domain types for SDK consumers
pub use {{crate_name}}::domain::Data;
{% endif %}

/// Client trait for {{module_name}} module
/// 
/// This trait defines the public API for interacting with the {{module_name}} module.
/// Other modules can depend on this SDK and use the client to communicate.
#[modkit::async_trait]
pub trait {{struct_module_name}}Client: Send + Sync {
    {% if module_type == "http-fetcher" %}
    /// Fetch data from the module
    /// 
    /// # Errors
    /// 
    /// Returns an error if the fetch operation fails.
    async fn fetch_data(&self) -> modkit::Result<Data>;
    {% elsif module_type == "simple-periodic" %}
    /// Trigger a task execution
    /// 
    /// # Errors
    /// 
    /// Returns an error if the task execution fails.
    async fn trigger_task(&self) -> modkit::Result<()>;
    {% else %}
    /// Example method - implement your custom API
    /// 
    /// # Errors
    /// 
    /// Returns an error if the operation fails.
    async fn example_operation(&self) -> modkit::Result<()>;
    {% endif %}
}

// TODO: Implement the client trait for your module
// Example:
// 
// impl {{struct_module_name}}Client for {{struct_module_name}}Module {
{% if module_type == "http-fetcher" %}//     async fn fetch_data(&self) -> modkit::Result<Data> {
//         // Implementation
//     }
{% elsif module_type == "simple-periodic" %}//     async fn trigger_task(&self) -> modkit::Result<()> {
//         // Implementation
//     }
{% else %}//     async fn example_operation(&self) -> modkit::Result<()> {
//         // Implementation
//     }
{% endif %}// }
