//! Object-safe streaming boundary for the pokemon module.
//!
//! Designed for `ClientHub` registration as `Arc<dyn PokemonClientV1>`.

#[cfg(feature = "odata")]
use futures_core::Stream;
use modkit::async_trait;
#[cfg(feature = "odata")]
use modkit_sdk::odata::QueryBuilder;
#[cfg(feature = "odata")]
use std::pin::Pin;
use uuid::Uuid;

use crate::errors::PokemonError;
use crate::models::Pokemon;

#[cfg(feature = "odata")]
use crate::odata::PokemonSchema;

/// Boxed stream type returned by streaming client facades.
#[cfg(feature = "odata")]
pub type PokemonStream<T> = Pin<Box<dyn Stream<Item = Result<T, PokemonError>> + Send + 'static>>;

/// Object-safe client for inter-module consumption via `ClientHub` (Version 1).
#[async_trait]
pub trait PokemonClientV1: Send + Sync {
    #[cfg(feature = "odata")]
    fn pokemon(&self) -> Box<dyn PokemonStreamingClientV1>;

    /// Get a single pokemon by ID.
    async fn get_pokemon(&self, id: Uuid) -> Result<Pokemon, PokemonError>;

    /// List pokemon with cursor-based pagination.
    async fn list_pokemon(
        &self,
        query: modkit_odata::ODataQuery,
    ) -> Result<modkit_odata::Page<Pokemon>, PokemonError>;
}

/// Streaming interface for pokemon (Version 1).
#[cfg(feature = "odata")]
pub trait PokemonStreamingClientV1: Send + Sync {
    fn stream(&self, query: QueryBuilder<PokemonSchema>) -> PokemonStream<Pokemon>;
}
