//! Pokemon SDK
//!
//! Public API contract for the pokemon module:
//! - `PokemonClientV1` trait
//! - `Pokemon` model
//! - `PokemonError` error type
//! - OData filter schemas (behind `odata` feature)

pub mod client;
pub mod errors;
pub mod models;

#[cfg(feature = "odata")]
pub mod odata;

pub use client::PokemonClientV1;
#[cfg(feature = "odata")]
pub use client::PokemonStreamingClientV1;
pub use errors::PokemonError;
pub use models::Pokemon;
