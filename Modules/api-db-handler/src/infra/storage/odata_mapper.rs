//! Infrastructure layer mapping from type-safe `FilterNode` to SeaORM Conditions.
//! Only compiled when the `odata` feature is enabled.
use modkit_db::odata::sea_orm_filter::{
    FieldToColumn, ODataFieldMapping, filter_node_to_condition,
};
use modkit_odata::filter::FilterNode;
use sea_orm::Condition;

use crate::infra::storage::entity::{Column, Entity, Model};
use api_db_handler_sdk::odata::PokemonFilterField;

/// Complete OData mapper for pokemon.
pub struct PokemonODataMapper;

impl FieldToColumn<PokemonFilterField> for PokemonODataMapper {
    type Column = Column;

    fn map_field(field: PokemonFilterField) -> Column {
        match field {
            PokemonFilterField::Id => Column::Id,
            PokemonFilterField::Name => Column::Name,
            PokemonFilterField::CreatedAt => Column::CreatedAt,
        }
    }
}

impl ODataFieldMapping<PokemonFilterField> for PokemonODataMapper {
    type Entity = Entity;

    fn extract_cursor_value(model: &Model, field: PokemonFilterField) -> sea_orm::Value {
        match field {
            PokemonFilterField::Id => sea_orm::Value::Uuid(Some(Box::new(model.id))),
            PokemonFilterField::Name => sea_orm::Value::String(Some(Box::new(model.name.clone()))),
            PokemonFilterField::CreatedAt => {
                sea_orm::Value::TimeDateTimeWithTimeZone(Some(Box::new(model.created_at)))
            }
        }
    }
}

/// Map a `FilterNode`<PokemonFilterField> to a SeaORM Condition.
pub fn filter_to_condition(filter: &FilterNode<PokemonFilterField>) -> Result<Condition, String> {
    filter_node_to_condition::<PokemonFilterField, PokemonODataMapper>(filter)
}
