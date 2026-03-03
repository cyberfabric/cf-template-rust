//! Public error types for the pokemon module.

use thiserror::Error;
use uuid::Uuid;

/// Errors that can be returned by the `PokemonClient`.
#[derive(Error, Debug, Clone)]
pub enum PokemonError {
    #[error("Resource not found: {id}")]
    NotFound { id: Uuid },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Internal error")]
    Internal,

    #[error("Streaming error: {message}")]
    Streaming { message: String },
}

impl PokemonError {
    #[must_use]
    pub fn not_found(id: Uuid) -> Self {
        Self::NotFound { id }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn internal() -> Self {
        Self::Internal
    }

    pub fn streaming(message: impl Into<String>) -> Self {
        Self::Streaming {
            message: message.into(),
        }
    }
}
