//! Background Worker SDK
//!
//! This crate provides the public API for the `background-worker` module:
//! - `PokemonClientV1` trait — the inter-module client interface
//! - `Pokemon` — the public model type
//! - `PokemonError` — the public error type
//!
//! ## Usage
//!
//! Consumers obtain the client from `ClientHub`:
//! ```ignore
//! use background_worker_sdk::PokemonClientV1;
//!
//! // Get the client from ClientHub
//! let client = hub.get::<dyn PokemonClientV1>()?;
//!
//! // Use the API
//! let pokemon = client.fetch_random_pokemon().await?;
//! println!("Got: {} (height={})", pokemon.name, pokemon.height);
//! ```

pub mod client;
pub mod errors;
pub mod models;

// Re-export main types at crate root for convenience
pub use client::PokemonClientV1;
pub use errors::PokemonError;
pub use models::Pokemon;
