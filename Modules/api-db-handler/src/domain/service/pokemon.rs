use std::sync::Arc;

use api_db_handler_sdk::Pokemon;
use modkit_macros::domain_model;
use modkit_odata::{ODataQuery, Page};
use modkit_security::{AccessScope, SecurityContext};
use tracing::instrument;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::repos::PokemonRepository;
use crate::domain::service::{DbProvider, ServiceConfig};

/// Pokemon service.
#[domain_model]
pub struct PokemonService<R: PokemonRepository + 'static> {
    db: Arc<DbProvider>,
    repo: Arc<R>,
    #[allow(dead_code)] // in case we need it
    config: ServiceConfig,
}

impl<R: PokemonRepository + 'static> PokemonService<R> {
    pub fn new(db: Arc<DbProvider>, repo: Arc<R>, config: ServiceConfig) -> Self {
        Self { db, repo, config }
    }
}

impl<R: PokemonRepository + 'static> PokemonService<R> {
    #[instrument(skip(self, ctx), fields(pokemon_id = %id))]
    pub async fn get_pokemon(
        &self,
        ctx: &SecurityContext,
        id: Uuid,
    ) -> Result<Pokemon, DomainError> {
        tracing::debug!("Getting pokemon by id");

        let conn = self.db.conn().map_err(DomainError::from)?;
        // Restrict row-level access to the caller's own tenant.
        let scope = AccessScope::for_tenant(ctx.subject_tenant_id());

        let pokemon = self
            .repo
            .get(&conn, &scope, id)
            .await?
            .ok_or_else(|| DomainError::not_found(id))?;

        tracing::debug!("Successfully retrieved pokemon");
        Ok(pokemon)
    }

    /// List pokemon with cursor-based pagination
    #[instrument(skip(self, ctx, query))]
    pub async fn list_pokemon_page(
        &self,
        ctx: &SecurityContext,
        query: &ODataQuery,
    ) -> Result<Page<Pokemon>, DomainError> {
        tracing::debug!("Listing pokemon with cursor pagination");

        let conn = self.db.conn().map_err(DomainError::from)?;
        // Restrict row-level access to the caller's own tenant.
        let scope = AccessScope::for_tenant(ctx.subject_tenant_id());

        let page = self.repo.list_page(&conn, &scope, query).await?;

        tracing::debug!("Successfully listed {} pokemon in page", page.items.len());
        Ok(page)
    }
}
