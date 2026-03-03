use crate::infra::storage::entity;
use api_db_handler_sdk::Pokemon;

/// Convert a database entity to a contract model (owned version)
impl From<entity::pokemon::Model> for Pokemon {
    fn from(e: entity::pokemon::Model) -> Self {
        Self {
            id: e.id,
            tenant_id: e.tenant_id,
            name: e.name,
            height: e.height,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

/// Convert a database entity to a contract model (by-ref version)
impl From<&entity::pokemon::Model> for Pokemon {
    fn from(e: &entity::pokemon::Model) -> Self {
        Self {
            id: e.id,
            tenant_id: e.tenant_id,
            name: e.name.clone(),
            height: e.height,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}
