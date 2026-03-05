use std::sync::{Arc, OnceLock};

use modkit::api::OpenApiRegistry;
use modkit::{DatabaseCapability, Module, ModuleCtx, RestApiCapability, async_trait};
use modkit_db::DBProvider;
use modkit_db::DbError;
use sea_orm_migration::MigrationTrait;
use tracing::{debug, info};

use api_db_handler_sdk::PokemonClientV1;

use crate::api::rest::routes;
use crate::config::PokemonConfig;
use crate::domain::local_client::client::PokemonLocalClient;
use crate::domain::service::{AppServices, ServiceConfig};
use crate::infra::storage::OrmPokemonRepository;

/// Type alias for the concrete `AppServices` type used with ORM repositories.
pub(crate) type ConcreteAppServices = AppServices<OrmPokemonRepository>;

/// Pokemon module with DDD-light layout and proper `ClientHub` integration
#[modkit::module(
    name = "api-db-handler",
    capabilities = [db, rest]
)]
pub struct PokemonModule {
    service: OnceLock<Arc<ConcreteAppServices>>,
}

impl Default for PokemonModule {
    fn default() -> Self {
        Self {
            service: OnceLock::new(),
        }
    }
}

#[async_trait]
impl Module for PokemonModule {
    async fn init(&self, ctx: &ModuleCtx) -> anyhow::Result<()> {
        let cfg: PokemonConfig = ctx.config()?;
        debug!(
            "Loaded pokemon config: default_page_size={}, max_page_size={}",
            cfg.default_page_size, cfg.max_page_size
        );

        let db: Arc<DBProvider<DbError>> = Arc::new(ctx.db_required()?);

        let service_config = ServiceConfig {
            default_page_size: cfg.default_page_size,
            max_page_size: cfg.max_page_size,
        };

        let limit_cfg = service_config.limit_cfg();
        let pokemon_repo = OrmPokemonRepository::new(limit_cfg);

        let services = Arc::new(AppServices::new(pokemon_repo, db, service_config));

        self.service
            .set(services.clone())
            .map_err(|_| anyhow::anyhow!("{} module already initialized", Self::MODULE_NAME))?;

        let local = PokemonLocalClient::new(services);

        ctx.client_hub()
            .register::<dyn PokemonClientV1>(Arc::new(local));

        Ok(())
    }
}

impl DatabaseCapability for PokemonModule {
    fn migrations(&self) -> Vec<Box<dyn MigrationTrait>> {
        use sea_orm_migration::MigratorTrait;
        info!("Providing pokemon database migrations");
        crate::infra::storage::migrations::Migrator::migrations()
    }
}

impl RestApiCapability for PokemonModule {
    fn register_rest(
        &self,
        _ctx: &ModuleCtx,
        router: axum::Router,
        openapi: &dyn OpenApiRegistry,
    ) -> anyhow::Result<axum::Router> {
        info!("Registering pokemon REST routes");

        let service = self
            .service
            .get()
            .ok_or_else(|| anyhow::anyhow!("Service not initialized"))?
            .clone();

        let router = routes::register_routes(router, openapi, service);

        info!("Pokemon REST routes registered successfully");
        Ok(router)
    }
}
