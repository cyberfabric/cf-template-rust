//! `RestHostConfig` — runtime configuration for the REST host module.
//!
//! Loaded by `ModKit` from the host application's config file under the
//! `modules.rest_host.config` key (or whichever key the integrator chooses).

use serde::{Deserialize, Serialize};

fn default_bind_addr() -> String {
    "127.0.0.1:8080".to_owned()
}

fn default_timeout_secs() -> u64 {
    30
}

/// Configuration for the REST host module.
///
/// Example YAML:
/// ```yaml
/// modules:
///   rest_host:
///     config:
///       bind_addr: "0.0.0.0:8080"
///       timeout_secs: 30
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestHostConfig {
    /// Socket address the HTTP server binds to (e.g. `"127.0.0.1:8080"`).
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// Per-request timeout in seconds. Requests that exceed this limit receive
    /// `408 Request Timeout`. Defaults to 30 s.
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

impl Default for RestHostConfig {
    fn default() -> Self {
        Self {
            bind_addr: default_bind_addr(),
            timeout_secs: default_timeout_secs(),
        }
    }
}
