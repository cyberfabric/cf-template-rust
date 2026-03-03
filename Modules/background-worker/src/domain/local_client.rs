use std::sync::Arc;

use background_worker_sdk::{Pokemon, PokemonClientV1, PokemonError};
use modkit::async_trait;

use crate::domain::service::PokemonService;

/// SDK boundary adapter: implements `PokemonClientV1` by delegating to `PokemonService`.
///
/// Registered into `ClientHub` during module `init()` so other modules can call:
/// ```ignore
/// let client = hub.get::<dyn PokemonClientV1>()?;
/// let pokemon = client.fetch_random_pokemon().await?;
/// ```
pub struct PokemonLocalClient {
    service: Arc<PokemonService>,
}

impl PokemonLocalClient {
    pub fn new(service: Arc<PokemonService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl PokemonClientV1 for PokemonLocalClient {
    async fn fetch_random_pokemon(&self) -> Result<Pokemon, PokemonError> {
        self.service
            .fetch_random_pokemon()
            .await
            .map_err(|e| PokemonError::internal(e.to_string()))
    }
}
