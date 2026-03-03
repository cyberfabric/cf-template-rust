//! # `rest_host` — Minimal REST host module
//!
//! A lightweight HTTP host built on Axum that exposes health endpoints and
//! a standard middleware stack.  It follows the `ModKit` module pattern used by
//! `cf-api-gateway` but deliberately omits authentication, CORS, rate-limiting,
//! and `OpenAPI` documentation to keep the surface area as small as possible.
//!
//! ## Quick start
//!
//! Add this module to your application YAML:
//!
//! ```yaml
//! modules:
//!   rest_host:
//!     config:
//!       bind_addr: "0.0.0.0:8080"
//!       timeout_secs: 30
//! ```
//!
//! Then register [`RestHost`] with the `ModKit` runtime.
//!
//! ## Built-in endpoints
//!
//! | Method | Path       | Description                             |
//! |--------|------------|-----------------------------------------|
//! | GET    | `/health`  | JSON `{ "status": "healthy", "timestamp": … }` |
//! | GET    | `/healthz` | Plain-text `"ok"` (Kubernetes probe)    |

// ── Module definition (aggregate root) ──────────────────────────────────────
mod module;
pub use module::RestHost;

// ── Internal sub-modules ─────────────────────────────────────────────────────
mod config;
pub mod middleware;
mod web;

// ── Public re-exports ────────────────────────────────────────────────────────
pub use config::RestHostConfig;
