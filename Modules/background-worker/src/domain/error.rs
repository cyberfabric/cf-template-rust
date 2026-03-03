use thiserror::Error;

/// Internal domain errors for the `background-worker` module.
///
/// These are not exposed to external consumers — `PokemonLocalClient` maps
/// these to the public `PokemonError` at the SDK boundary.
#[derive(Error, Debug)]
pub enum DomainError {
    /// An HTTP-level error occurred while fetching from the external API.
    #[error("HTTP error: {0}")]
    Http(String),

    /// The API response could not be deserialized.
    #[error("Failed to parse API response: {0}")]
    Parse(String),
}
