//! HTTP handler functions for the REST host's built-in endpoints.
//!
//! Both endpoints are always public (no authentication required) so they can be
//! used by load-balancers and container orchestrators without credentials.

use axum::response::Json;
use chrono::{SecondsFormat, Utc};
use serde_json::{Value, json};

/// `GET /health` — detailed liveness/readiness probe.
///
/// Returns a JSON body with `status`, an ISO-8601 `timestamp` so that
/// monitoring tools can see when the response was generated, and a `version`
/// string taken from the crate's `CARGO_PKG_VERSION` at compile time.
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// `GET /healthz` — minimal Kubernetes-style liveness probe.
///
/// Returns the plain-text string `"ok"` with a `200 OK` status.
pub async fn healthz() -> &'static str {
    "ok"
}
