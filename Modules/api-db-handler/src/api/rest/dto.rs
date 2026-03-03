use api_db_handler_sdk::Pokemon;
use time::OffsetDateTime;
use uuid::Uuid;

/// REST DTO for pokemon representation with serde/utoipa
#[derive(Debug, Clone)]
#[modkit_macros::api_dto(request, response)]
pub struct PokemonDto {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    /// Height in decimetres.
    pub height: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl From<Pokemon> for PokemonDto {
    fn from(p: Pokemon) -> Self {
        Self {
            id: p.id,
            tenant_id: p.tenant_id,
            name: p.name,
            height: p.height,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
