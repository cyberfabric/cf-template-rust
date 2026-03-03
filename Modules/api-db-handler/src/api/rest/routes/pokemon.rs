//! Only compiled when the `odata` feature is enabled — the list route requires
//! typed OData filter fields from the SDK's `odata` module.
use super::{License, dto, handlers};
use api_db_handler_sdk::odata::PokemonFilterField;
use axum::Router;
use modkit::api::OpenApiRegistry;
use modkit::api::operation_builder::{OperationBuilder, OperationBuilderODataExt};

pub(super) fn register_pokemon_routes(mut router: Router, openapi: &dyn OpenApiRegistry) -> Router {
    // GET /pokemon/v1/pokemon - List pokemon with cursor-based pagination
    router = OperationBuilder::get("/pokemon/v1/pokemon")
        .operation_id("pokemon.list_pokemon")
        .summary("List pokemon with cursor pagination")
        .description("Retrieve a paginated list of pokemon using cursor-based pagination")
        .tag("pokemon")
        .authenticated()
        .query_param_typed(
            "limit",
            false,
            "Maximum number of pokemon to return",
            "integer",
        )
        .require_license_features::<License>([])
        .query_param("cursor", false, "Cursor for pagination")
        .handler(handlers::list_pokemon)
        .json_response_with_schema::<modkit_odata::Page<dto::PokemonDto>>(
            openapi,
            http::StatusCode::OK,
            "Paginated list of pokemon",
        )
        .with_odata_filter::<PokemonFilterField>()
        .with_odata_select()
        .with_odata_orderby::<PokemonFilterField>()
        .error_400(openapi)
        .error_401(openapi)
        .error_403(openapi)
        .error_500(openapi)
        .register(router, openapi);

    // GET /pokemon/v1/pokemon/{id} - Get a specific pokemon
    router = OperationBuilder::get("/pokemon/v1/pokemon/{id}")
        .operation_id("pokemon.get_pokemon")
        .authenticated()
        .require_license_features::<License>([])
        .summary("Get pokemon by ID")
        .description("Retrieve a specific pokemon by their UUID")
        .tag("pokemon")
        .path_param("id", "Pokemon UUID")
        .handler(handlers::get_pokemon)
        .with_odata_select()
        .json_response_with_schema::<dto::PokemonDto>(
            openapi,
            http::StatusCode::OK,
            "Pokemon found",
        )
        .error_401(openapi)
        .error_403(openapi)
        .error_404(openapi)
        .error_500(openapi)
        .register(router, openapi);

    router
}
