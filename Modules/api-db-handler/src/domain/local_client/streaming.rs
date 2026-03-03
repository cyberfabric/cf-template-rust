use std::pin::Pin;
use std::sync::Arc;

use api_db_handler_sdk::odata::PokemonSchema;
use api_db_handler_sdk::{Pokemon, PokemonError, PokemonStreamingClientV1};
use futures_util::{Stream, StreamExt};
use modkit_macros::domain_model;
use modkit_sdk::odata::{QueryBuilder, items_stream_boxed};
use modkit_sdk::pager::PagerError;
use modkit_security::SecurityContext;

use crate::module::ConcreteAppServices;

#[domain_model]
pub(crate) struct LocalPokemonStreamingClient {
    services: Arc<ConcreteAppServices>,
}

impl LocalPokemonStreamingClient {
    #[must_use]
    pub fn new(services: Arc<ConcreteAppServices>) -> Self {
        Self { services }
    }
}

impl PokemonStreamingClientV1 for LocalPokemonStreamingClient {
    fn stream(
        &self,
        ctx: SecurityContext,
        query: QueryBuilder<PokemonSchema>,
    ) -> Pin<Box<dyn Stream<Item = Result<Pokemon, PokemonError>> + Send + 'static>> {
        let services = Arc::clone(&self.services);
        let stream = items_stream_boxed(
            query,
            Box::new(move |q| {
                let services = Arc::clone(&services);
                let ctx = ctx.clone();
                Box::pin(async move {
                    services
                        .pokemon
                        .list_pokemon_page(&ctx, &q)
                        .await
                        .map_err(PokemonError::from)
                })
            }),
        );
        Box::pin(stream.map(|res| {
            res.map_err(|err| match err {
                PagerError::Fetch(pokemon_err) => pokemon_err,
                PagerError::InvalidCursor(_) => PokemonError::streaming(err.to_string()),
            })
        }))
    }
}
