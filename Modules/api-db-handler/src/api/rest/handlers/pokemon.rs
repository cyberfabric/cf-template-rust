use axum::Extension;
use axum::extract::Path;
use tracing::field::Empty;
use uuid::Uuid;

use modkit::api::odata::OData;

use super::{
    ApiResult, Json, JsonBody, JsonPage, PokemonDto, SecurityContext, apply_select, info,
    page_to_projected_json,
};
use crate::module::ConcreteAppServices;

/// List pokemon with cursor-based pagination and optional field projection via $select
#[tracing::instrument(
    skip(svc, query, ctx),
    fields(
        limit = query.limit,
        request_id = Empty,
        user.id = %ctx.subject_id()
    )
)]
pub async fn list_pokemon(
    Extension(ctx): Extension<SecurityContext>,
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    OData(query): OData,
) -> ApiResult<JsonPage<serde_json::Value>> {
    info!(
        user_id = %ctx.subject_id(),
        "Listing pokemon with cursor pagination"
    );

    let page = svc.pokemon.list_pokemon_page(&ctx, &query).await?;
    let page = page.map_items(PokemonDto::from);

    Ok(Json(page_to_projected_json(&page, query.selected_fields())))
}

/// Get a specific pokemon by ID with optional field projection via $select
#[tracing::instrument(
    skip(svc, ctx),
    fields(
        pokemon.id = %id,
        request_id = Empty,
        requester.id = %ctx.subject_id()
    )
)]
pub async fn get_pokemon(
    Extension(ctx): Extension<SecurityContext>,
    Extension(svc): Extension<std::sync::Arc<ConcreteAppServices>>,
    Path(id): Path<Uuid>,
    OData(query): OData,
) -> ApiResult<JsonBody<serde_json::Value>> {
    info!(
        pokemon_id = %id,
        requester_id = %ctx.subject_id(),
        "Getting pokemon details"
    );

    let pokemon = svc.pokemon.get_pokemon(&ctx, id).await?;
    let pokemon_dto = PokemonDto::from(pokemon);
    let projected = apply_select(&pokemon_dto, query.selected_fields());
    Ok(Json(projected))
}
