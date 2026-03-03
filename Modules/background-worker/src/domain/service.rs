use std::sync::Arc;

use background_worker_sdk::Pokemon;

use crate::domain::error::DomainError;
use crate::domain::ports::PokemonRepository;

/// Domain service that orchestrates Pokemon fetching.
///
/// Delegates to the `PokemonRepository` port (implemented by `PokemonHttpRepository`).
pub struct PokemonService {
    repository: Arc<dyn PokemonRepository>,
}

impl PokemonService {
    pub fn new(repository: Arc<dyn PokemonRepository>) -> Self {
        Self { repository }
    }

    pub async fn fetch_random_pokemon(&self) -> Result<Pokemon, DomainError> {
        self.repository.fetch_random().await
    }
}
