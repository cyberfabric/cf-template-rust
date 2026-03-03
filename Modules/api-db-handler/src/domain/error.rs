use api_db_handler_sdk::PokemonError;
use modkit_db::DbError;
use modkit_db::secure::InfraError;
use modkit_db::secure::ScopeError;
use modkit_macros::domain_model;
use thiserror::Error;
use uuid::Uuid;

/// Domain-specific errors
#[domain_model]
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Pokemon not found: {id}")]
    NotFound { id: Uuid },

    #[error("Database error: {message}")]
    Database { message: String },

    #[error("Validation failed: {field}: {message}")]
    Validation { field: String, message: String },

    #[error("Internal error")]
    InternalError,
}

impl DomainError {
    #[must_use]
    pub fn not_found(id: Uuid) -> Self {
        Self::NotFound { id }
    }

    pub fn database(message: impl Into<String>) -> Self {
        Self::Database {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn database_infra(e: InfraError) -> Self {
        Self::database(e.to_string())
    }

    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl From<DomainError> for PokemonError {
    fn from(domain_error: DomainError) -> Self {
        match domain_error {
            DomainError::NotFound { id } => PokemonError::not_found(id),
            DomainError::Validation { field, message } => {
                PokemonError::validation(format!("{field}: {message}"))
            }
            DomainError::Database { .. } | DomainError::InternalError => PokemonError::internal(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for DomainError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        tracing::debug!(error = %value, "Converting boxed error to DomainError");
        DomainError::InternalError
    }
}

impl From<DbError> for DomainError {
    fn from(e: DbError) -> Self {
        DomainError::database(e.to_string())
    }
}

impl From<ScopeError> for DomainError {
    fn from(e: ScopeError) -> Self {
        DomainError::validation("scope", e.to_string())
    }
}
