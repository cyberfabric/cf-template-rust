//! REST API route definitions - OpenAPI and Axum routing.

#[cfg(feature = "odata")]
use crate::api::rest::{dto, handlers};
use crate::module::ConcreteAppServices;
use axum::Router;
use modkit::api::OpenApiRegistry;
use std::sync::Arc;

#[cfg(feature = "odata")]
mod pokemon;

/// Register all routes for the pokemon module
pub(crate) fn register_routes(
    mut router: Router,
    openapi: &dyn OpenApiRegistry,
    services: Arc<ConcreteAppServices>,
) -> Router {
    #[cfg(feature = "odata")]
    {
        router = pokemon::register_pokemon_routes(router, openapi);
    }

    #[cfg(not(feature = "odata"))]
    let _ = openapi;

    router = router.layer(axum::Extension(services));

    router
}
