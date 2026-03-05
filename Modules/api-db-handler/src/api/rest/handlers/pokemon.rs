use axum::Extension;
use axum::extract::Path;
use tracing::field::Empty;
use uuid::Uuid;

use modkit::api::odata::OData;

use super::{
    ApiResult, Json, JsonBody, JsonPage, PokemonDto, apply_select, page_to_projected_json,
};
use crate::module::ConcreteAppServices;

/// List pokemon with cursor-based pagination and optional field projection via $select
#[tracing::instrument(
    skip(svc, query),
    fields(
        limit = query.limit,
        request_id = Empty,
    )
)]
pub async fn list_pokemon(
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    OData(query): OData,
) -> ApiResult<JsonPage<serde_json::Value>> {
    let page = svc.pokemon.list_pokemon_page(&query).await?;
    let page = page.map_items(PokemonDto::from);

    Ok(Json(page_to_projected_json(&page, query.selected_fields())))
}

/// Get a specific pokemon by ID with optional field projection via $select
#[tracing::instrument(
    skip(svc),
    fields(
        pokemon.id = %id,
        request_id = Empty,
    )
)]
pub async fn get_pokemon(
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    Path(id): Path<Uuid>,
    OData(query): OData,
) -> ApiResult<JsonBody<serde_json::Value>> {
    let pokemon = svc.pokemon.get_pokemon(id).await?;
    let pokemon_dto = PokemonDto::from(pokemon);
    let projected = apply_select(&pokemon_dto, query.selected_fields());
    Ok(Json(projected))
}
