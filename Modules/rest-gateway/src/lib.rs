#![doc = include_str!("../README.md")]

// ── Module definition (aggregate root) ──────────────────────────────────────
mod module;
pub use module::RestHost;

// ── Internal sub-modules ─────────────────────────────────────────────────────
mod config;
pub mod middleware;
mod web;

// ── Public re-exports ────────────────────────────────────────────────────────
pub use config::RestHostConfig;
