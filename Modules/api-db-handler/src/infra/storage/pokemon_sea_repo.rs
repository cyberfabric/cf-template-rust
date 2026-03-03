use modkit::async_trait;

use crate::infra::storage::db::db_err;
use crate::infra::storage::entity::pokemon::{Column, Entity as PokemonEntity};
#[cfg(feature = "odata")]
use crate::infra::storage::odata_mapper::PokemonODataMapper;
use crate::{domain::error::DomainError, domain::repos::PokemonRepository};
use api_db_handler_sdk::Pokemon;
#[cfg(feature = "odata")]
use api_db_handler_sdk::odata::PokemonFilterField;
use modkit_db::odata::LimitCfg;
#[cfg(feature = "odata")]
use modkit_db::odata::paginate_odata;
use modkit_db::secure::{DBRunner, SecureEntityExt};
#[cfg(feature = "odata")]
use modkit_odata::SortDir;
use modkit_odata::{ODataQuery, Page};
use modkit_security::AccessScope;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::sea_query::Expr;
use uuid::Uuid;

/// ORM-based implementation of the `PokemonRepository` trait.
#[derive(Clone)]
pub struct OrmPokemonRepository {
    limit_cfg: LimitCfg,
}

impl OrmPokemonRepository {
    #[must_use]
    pub fn new(limit_cfg: LimitCfg) -> Self {
        Self { limit_cfg }
    }
}

#[async_trait]
impl PokemonRepository for OrmPokemonRepository {
    async fn get<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        id: Uuid,
    ) -> Result<Option<Pokemon>, DomainError> {
        let found = PokemonEntity::find()
            .filter(sea_orm::Condition::all().add(Expr::col(Column::Id).eq(id)))
            .secure()
            .scope_with(scope)
            .one(conn)
            .await
            .map_err(db_err)?;
        Ok(found.map(Into::into))
    }

    async fn list_page<C: DBRunner>(
        &self,
        conn: &C,
        scope: &AccessScope,
        query: &ODataQuery,
    ) -> Result<Page<Pokemon>, DomainError> {
        #[cfg(feature = "odata")]
        {
            let base_query = PokemonEntity::find().secure().scope_with(scope);

            let page = paginate_odata::<PokemonFilterField, PokemonODataMapper, _, _, _, _>(
                base_query,
                conn,
                query,
                ("id", SortDir::Desc),
                self.limit_cfg,
                Into::into,
            )
            .await
            .map_err(db_err)?;

            Ok(page)
        }

        #[cfg(not(feature = "odata"))]
        {
            let _ = (conn, scope, query);
            Err(DomainError::validation(
                "query",
                "OData feature is disabled",
            ))
        }
    }
}
