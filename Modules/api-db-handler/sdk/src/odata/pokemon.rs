//! OData filter field definitions for Pokemon resources.

use modkit_odata_macros::ODataFilterable;
use modkit_sdk::odata::{FieldRef, Schema};
use time::OffsetDateTime;
use uuid::Uuid;

use modkit_odata::filter::FilterField as _;

/// Pokemon filterable fields schema.
#[derive(ODataFilterable)]
pub struct PokemonQuery {
    #[odata(filter(kind = "Uuid"))]
    pub id: Uuid,

    #[odata(filter(kind = "String"))]
    pub name: String,

    #[odata(filter(kind = "DateTimeUtc"))]
    pub created_at: OffsetDateTime,
}

/// Type alias for the generated filter field enum.
pub use PokemonQueryFilterField as PokemonFilterField;

#[derive(Debug, Clone, Copy)]
pub struct PokemonSchema;

impl Schema for PokemonSchema {
    type Field = PokemonFilterField;

    fn field_name(field: Self::Field) -> &'static str {
        field.name()
    }
}

pub const POKEMON_ID: FieldRef<PokemonSchema, Uuid> = FieldRef::new(PokemonFilterField::Id);
pub const POKEMON_NAME: FieldRef<PokemonSchema, String> = FieldRef::new(PokemonFilterField::Name);
pub const POKEMON_CREATED_AT: FieldRef<PokemonSchema, OffsetDateTime> =
    FieldRef::new(PokemonFilterField::CreatedAt);
