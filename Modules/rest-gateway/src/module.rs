//! `RestHost` module definition.
//!
//! This is the aggregate root of the REST host bounded context.  It owns:
//!
//! * **Configuration** (`RestHostConfig`) — loaded once during `init`.
//! * **Router** — assembled in `rest_prepare` / `rest_finalize`, served in `serve`.
//! * **`OpenAPI` registry** — a pass-through registry so that other `rest`-capable
//!   modules can register their operations with this host.
//!
//! # Middleware stack (outermost → innermost / first to last in the request path)
//!
//! ```text
//! SetRequestId          ← generates x-request-id if absent
//! PropagateRequestId    ← copies x-request-id to the response
//! TraceLayer            ← opens an http_request span
//! push_req_id           ← records x-request-id into the span
//! TimeoutLayer          ← returns 408 after `timeout_secs`
//! Router / handlers
//! ```
//!
//! # DDD mapping
//!
//! | DDD layer       | File(s)                          |
//! |-----------------|----------------------------------|
//! | Interface        | `web.rs` (HTTP handlers)        |
//! | Application      | `module.rs` (this file)         |
//! | Infrastructure   | `config.rs`, `middleware/`      |

use std::net::SocketAddr;
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use parking_lot::Mutex;

use anyhow::Result;
use axum::http::StatusCode;
use axum::{Router, middleware::from_fn, routing::get};
use tokio_util::sync::CancellationToken;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::field::Empty;

use crate::config::RestHostConfig;
use crate::middleware;
use crate::web;

/// REST host module — owns the HTTP server and the shared `axum::Router`.
///
/// Declare this module in your application with:
/// ```yaml
/// modules:
///   rest_host:
///     config:
///       bind_addr: "0.0.0.0:8080"
///       timeout_secs: 30
/// ```
#[modkit::module(
    name = "rest-host",
    capabilities = [rest_host, stateful],
    lifecycle(entry = "serve", stop_timeout = "30s", await_ready)
)]
pub struct RestHost {
    /// Effective configuration; set exactly once during `init`.
    config: OnceLock<RestHostConfig>,

    /// Shared `OpenAPI` registry delegated to other `rest`-capable modules.
    openapi_registry: Arc<modkit::OpenApiRegistryImpl>,

    /// Final assembled router stored by `rest_finalize` and consumed by `serve`.
    final_router: Mutex<Option<Router>>,
}

impl Default for RestHost {
    fn default() -> Self {
        Self {
            config: OnceLock::new(),
            openapi_registry: Arc::new(modkit::OpenApiRegistryImpl::new()),
            final_router: Mutex::new(None),
        }
    }
}

// ── Private helpers ──────────────────────────────────────────────────────────

/// Parses `addr_str` as a [`SocketAddr`], producing a descriptive error.
///
/// Extracted as a free function so that both `RestHost::parse_bind_addr` (used
/// by `serve`) and the fail-fast check in `init` share identical logic.
fn parse_bind_addr_str(addr_str: &str) -> Result<SocketAddr> {
    addr_str
        .parse()
        .map_err(|e| anyhow::anyhow!("RestHostConfig: invalid `bind_addr` '{addr_str}': {e}"))
}

impl RestHost {
    /// Returns the cached config, or an error if `init` was never called.
    ///
    /// In practice this can only be `None` if a capability method is invoked
    /// before the `ModKit` lifecycle has run `init`, which is a framework-level
    /// programming error.
    fn cfg(&self) -> anyhow::Result<&RestHostConfig> {
        self.config.get().ok_or_else(|| {
            anyhow::anyhow!("RestHost::init must complete before any capability method is called")
        })
    }

    /// Wraps `router` with the standard middleware stack.
    ///
    /// Layers are added innermost-first (`layer()` semantics); see the module
    /// doc-comment for the resulting execution order.
    fn apply_middleware_stack(&self, mut router: Router) -> anyhow::Result<Router> {
        let timeout = Duration::from_secs(self.cfg()?.timeout_secs);
        let x_req_id = middleware::request_id::header();

        // ── 5) Timeout ────────────────────────────────────────────────────────
        // Added first so it wraps the router tightly; outermost layers added last.
        // Use 408 Request Timeout (appropriate for a REST host, unlike 504 for a proxy).
        router = router.layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            timeout,
        ));

        // ── 4) Record request_id into span + extensions ───────────────────────
        // Must be inside (inner to) the TraceLayer so the span exists.
        router = router.layer(from_fn(middleware::request_id::push_req_id_to_extensions));

        // ── 3) Trace ──────────────────────────────────────────────────────────
        router = router.layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &axum::http::Request<axum::body::Body>| {
                    tracing::info_span!(
                        "http_request",
                        method  = %req.method(),
                        uri     = %req.uri().path(),
                        request_id = Empty,
                        status     = Empty,
                        latency_ms = Empty,
                    )
                })
                .on_response(
                    |res: &axum::http::Response<axum::body::Body>,
                     latency: Duration,
                     span: &tracing::Span| {
                        span.record("status", res.status().as_u16());
                        span.record(
                            "latency_ms",
                            u64::try_from(latency.as_millis()).unwrap_or(u64::MAX),
                        );
                    },
                ),
        );

        // ── 2) Propagate request-id to response ───────────────────────────────
        router = router.layer(PropagateRequestIdLayer::new(x_req_id.clone()));

        // ── 1) Generate x-request-id if absent (outermost, runs first) ────────
        router = router.layer(SetRequestIdLayer::new(
            x_req_id,
            middleware::request_id::MakeReqId,
        ));

        Ok(router)
    }

    /// Builds a standalone router (health endpoints + middleware) for use when
    /// the `ModKit` REST pipeline has not been run (e.g. in unit tests).
    fn build_router(&self) -> anyhow::Result<Router> {
        let router = Router::new()
            .route("/health", get(web::health_check))
            .route("/healthz", get(web::healthz));
        self.apply_middleware_stack(router)
    }

    /// Returns the finalised router from `rest_finalize` if available; falls
    /// back to building a default router in standalone mode.
    fn take_or_build_router(&self) -> anyhow::Result<Router> {
        let stored = self.final_router.lock().take();

        match stored {
            Some(router) => Ok(router),
            None => self.build_router(),
        }
    }

    /// Background task: bind the listener, signal ready, then serve until cancelled.
    pub(crate) async fn serve(
        self: Arc<Self>,
        cancel: CancellationToken,
        ready: modkit::lifecycle::ReadySignal,
    ) -> anyhow::Result<()> {
        let addr = parse_bind_addr_str(&self.cfg()?.bind_addr)?;
        let router = self.take_or_build_router()?;

        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("REST host listening on {addr}");
        ready.notify(); // Starting → Running

        let shutdown = async move {
            cancel.cancelled().await;
            tracing::info!("REST host shutting down gracefully");
        };

        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}

// ── ModKit trait implementations ─────────────────────────────────────────────

/// `Module` — loads configuration during the init phase.
#[modkit::async_trait]
impl modkit::Module for RestHost {
    async fn init(&self, ctx: &modkit::context::ModuleCtx) -> anyhow::Result<()> {
        let cfg = ctx.config::<RestHostConfig>()?;

        // ── Fail-fast validation ──────────────────────────────────────────────

        // `timeout_secs` must be positive; a zero timeout would immediately
        // abort every request before any handler runs.
        if cfg.timeout_secs == 0 {
            anyhow::bail!("RestHostConfig: `timeout_secs` must be > 0 (got 0)");
        }

        // `bind_addr` must be parseable as a `SocketAddr` — the same check
        // `serve` would perform, surfaced here so the process fails at startup
        // rather than at the moment the listener is bound.
        parse_bind_addr_str(&cfg.bind_addr)?;

        // ── Commit configuration ──────────────────────────────────────────────
        self.config
            .set(cfg.clone())
            .map_err(|_| anyhow::anyhow!("RestHostConfig already set; init called twice?"))?;
        tracing::info!(bind_addr = %cfg.bind_addr, "REST host initialised");
        Ok(())
    }
}

/// `ApiGatewayCapability` (`rest_host`) — assembles the router across the two
/// `ModKit` REST phases without starting the server.
impl modkit::contracts::ApiGatewayCapability for RestHost {
    /// Phase 1: register built-in endpoints on the empty router that will be
    /// passed to every `rest`-capable module in turn.
    fn rest_prepare(
        &self,
        _ctx: &modkit::context::ModuleCtx,
        router: Router,
    ) -> anyhow::Result<Router> {
        let router = router
            .route("/health", get(web::health_check))
            .route("/healthz", get(web::healthz));
        tracing::debug!("REST host: /health and /healthz registered");
        Ok(router)
    }

    /// Phase 2: apply the middleware stack to the fully-populated router and
    /// stash it so `serve` can pick it up.
    fn rest_finalize(
        &self,
        _ctx: &modkit::context::ModuleCtx,
        router: Router,
    ) -> anyhow::Result<Router> {
        let router = self.apply_middleware_stack(router)?;
        *self.final_router.lock() = Some(router.clone());
        tracing::info!("REST host: router finalised with middleware stack");
        Ok(router)
    }

    /// Exposes the `OpenAPI` registry so that `rest`-capable peer modules can
    /// register their operation specs with this host.
    fn as_registry(&self) -> &dyn modkit::OpenApiRegistry {
        &*self.openapi_registry
    }
}
