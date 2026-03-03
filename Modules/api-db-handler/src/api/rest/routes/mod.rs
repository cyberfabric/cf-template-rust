//! REST API route definitions - OpenAPI and Axum routing.

#[cfg(feature = "odata")]
use crate::api::rest::{dto, handlers};
use crate::module::ConcreteAppServices;
use axum::Router;
use modkit::api::OpenApiRegistry;
#[cfg(feature = "odata")]
use modkit::api::operation_builder::LicenseFeature;
use std::sync::Arc;

#[cfg(feature = "odata")]
mod pokemon;

#[cfg(feature = "odata")]
pub(super) struct License;

#[cfg(feature = "odata")]
impl AsRef<str> for License {
    fn as_ref(&self) -> &'static str {
        "gts.x.core.lic.feat.v1~x.core.global.base.v1"
    }
}

#[cfg(feature = "odata")]
impl LicenseFeature for License {}

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
