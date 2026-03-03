//! Request-ID middleware — generates a unique `x-request-id` header for every
//! inbound request (if absent) and records it on the active tracing span.
//!
//! This matches the implementation used in `cf-api-gateway` so that request IDs
//! are consistent across the system.

use axum::http::{HeaderName, Request};
use axum::{body::Body, middleware::Next, response::Response};
use tower_http::request_id::{MakeRequestId, RequestId};

/// Extension type that stores the request ID in `Request::extensions` so that
/// business-logic handlers can read it without parsing the header directly.
#[derive(Clone, Debug)]
pub struct XRequestId(pub String);

/// Returns the canonical `x-request-id` header name.
#[must_use]
pub fn header() -> HeaderName {
    HeaderName::from_static("x-request-id")
}

/// `MakeRequestId` implementation that generates a short unique ID via `nanoid`.
#[derive(Clone, Default)]
pub struct MakeReqId;

impl MakeRequestId for MakeReqId {
    fn make_request_id<B>(&mut self, _req: &Request<B>) -> Option<RequestId> {
        let id = nanoid::nanoid!();
        Some(RequestId::new(id.parse().ok()?))
    }
}

/// Axum middleware that copies the `x-request-id` header value into
/// `Request::extensions` and records it on the current tracing span.
///
/// Must run *inside* (i.e. be layered before) `TraceLayer` so that the span
/// already exists when this middleware fires.
pub async fn push_req_id_to_extensions(mut req: Request<Body>, next: Next) -> Response {
    let hdr = header();
    if let Some(rid) = req
        .headers()
        .get(&hdr)
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string)
    {
        req.extensions_mut().insert(XRequestId(rid.clone()));
        tracing::Span::current().record("request_id", rid.as_str());
    }

    next.run(req).await
}
