use crate::domain::error::DomainError;
use background_worker_sdk::Pokemon;
use modkit::async_trait;

/// Repository port for fetching Pokemon data.
///
/// Implemented by `PokemonHttpRepository` in the infra layer.
#[async_trait]
pub trait PokemonRepository: Send + Sync {
    async fn fetch_random(&self) -> Result<Pokemon, DomainError>;
}
