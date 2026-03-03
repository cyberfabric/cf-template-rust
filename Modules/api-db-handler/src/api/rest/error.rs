use modkit::api::problem::Problem;

use crate::domain::error::DomainError;
use crate::errors::ErrorCode;

/// Map domain error to RFC9457 Problem using thiserror-backed API error codes.
pub fn domain_error_to_problem(e: &DomainError, instance: &str) -> Problem {
    let trace_id = tracing::Span::current()
        .id()
        .map(|id| id.into_u64().to_string());

    match &e {
        DomainError::NotFound { id } => ErrorCode::pokemon_not_found_v1().with_context(
            format!("Pokemon with id {id} was not found"),
            instance,
            trace_id,
        ),
        DomainError::Validation { .. } => {
            ErrorCode::pokemon_validation_v1().with_context(format!("{e}"), instance, trace_id)
        }
        DomainError::Database { .. } => {
            tracing::error!(error = ?e, "Database error occurred");
            ErrorCode::pokemon_internal_database_v1().with_context(
                "An internal database error occurred",
                instance,
                trace_id,
            )
        }
        DomainError::InternalError => {
            tracing::error!(error = ?e, "Internal error occurred");
            ErrorCode::internal_server_error_v1().with_context(
                "An internal error occurred",
                instance,
                trace_id,
            )
        }
    }
}

/// Implement Into<Problem> for `DomainError` so `?` works in handlers
impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        domain_error_to_problem(&e, "/")
    }
}
