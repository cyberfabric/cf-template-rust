//! Public models for the pokemon module.

use time::OffsetDateTime;
use uuid::Uuid;

/// A pokemon entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    /// Height in decimetres.
    pub height: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
