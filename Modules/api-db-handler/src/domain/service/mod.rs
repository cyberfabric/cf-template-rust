//! Domain service layer - business logic and rules.

use std::sync::Arc;

use modkit_macros::domain_model;

use crate::domain::repos::PokemonRepository;
use modkit_db::DBProvider;
use modkit_db::odata::LimitCfg;

mod pokemon;

pub(crate) use pokemon::PokemonService;

pub(crate) type DbProvider = DBProvider<modkit_db::DbError>;

/// Configuration for the domain service
#[domain_model]
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub default_page_size: u32,
    pub max_page_size: u32,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            default_page_size: 50,
            max_page_size: 1000,
        }
    }
}

impl ServiceConfig {
    #[must_use]
    pub fn limit_cfg(&self) -> LimitCfg {
        LimitCfg {
            default: u64::from(self.default_page_size),
            max: u64::from(self.max_page_size),
        }
    }
}

/// DI Container - aggregates all domain services
#[domain_model]
pub(crate) struct AppServices<PR>
where
    PR: PokemonRepository + 'static,
{
    pub(crate) pokemon: PokemonService<PR>,
}

impl<PR> AppServices<PR>
where
    PR: PokemonRepository + 'static,
{
    pub fn new(pokemon_repo: PR, db: Arc<DbProvider>, config: ServiceConfig) -> Self {
        let pokemon_repo = Arc::new(pokemon_repo);

        Self {
            pokemon: PokemonService::new(db, pokemon_repo, config),
        }
    }
}
