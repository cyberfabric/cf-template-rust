use std::sync::Arc;

use modkit::async_trait;
use modkit_macros::domain_model;
use modkit_odata::{ODataQuery, Page};
use modkit_security::SecurityContext;
use uuid::Uuid;

#[cfg(feature = "odata")]
use api_db_handler_sdk::PokemonStreamingClientV1;
use api_db_handler_sdk::{Pokemon, PokemonClientV1, PokemonError};

#[cfg(feature = "odata")]
use crate::domain::local_client::streaming::LocalPokemonStreamingClient;
use crate::module::ConcreteAppServices;

/// Local implementation of the object-safe `PokemonClientV1`.
#[domain_model]
#[derive(Clone)]
pub struct PokemonLocalClient {
    services: Arc<ConcreteAppServices>,
}

impl PokemonLocalClient {
    #[must_use]
    pub(crate) fn new(services: Arc<ConcreteAppServices>) -> Self {
        Self { services }
    }
}

#[async_trait]
impl PokemonClientV1 for PokemonLocalClient {
    #[cfg(feature = "odata")]
    fn pokemon(&self) -> Box<dyn PokemonStreamingClientV1> {
        Box::new(LocalPokemonStreamingClient::new(Arc::clone(&self.services)))
    }

    async fn get_pokemon(&self, ctx: SecurityContext, id: Uuid) -> Result<Pokemon, PokemonError> {
        self.services
            .pokemon
            .get_pokemon(&ctx, id)
            .await
            .map_err(PokemonError::from)
    }

    async fn list_pokemon(
        &self,
        ctx: SecurityContext,
        query: ODataQuery,
    ) -> Result<Page<Pokemon>, PokemonError> {
        self.services
            .pokemon
            .list_pokemon_page(&ctx, &query)
            .await
            .map_err(PokemonError::from)
    }
}
