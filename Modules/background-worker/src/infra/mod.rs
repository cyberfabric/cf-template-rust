use anyhow::Context;
use modkit::async_trait;
use modkit_http::HttpClient;
use std::time::{Duration, UNIX_EPOCH};

use background_worker_sdk::Pokemon;

use crate::domain::error::DomainError;
use crate::domain::ports::PokemonRepository;

mod model;
use model::PokemonResponse;

const API_URL: &str = "https://pokeapi.co/api/v2/pokemon/";

/// Infra implementation of `PokemonRepository` — fetches Pokemon over HTTP from PokeAPI.
pub struct PokemonHttpRepository {
    client: HttpClient,
}

impl PokemonHttpRepository {
    pub fn new() -> modkit::Result<Self> {
        Ok(Self {
            client: HttpClient::builder()
                .no_redirects()
                .timeout(Duration::from_secs(5))
                .build()
                .context("problem while building http client")?,
        })
    }
}

#[async_trait]
impl PokemonRepository for PokemonHttpRepository {
    async fn fetch_random(&self) -> Result<Pokemon, DomainError> {
        let url = format!(
            "{}{}",
            API_URL,
            (UNIX_EPOCH
                .elapsed()
                .map_err(|e| DomainError::Http(e.to_string()))?
                .subsec_nanos()
                % 150)
                + 1
        );
        tracing::debug!("Fetching pokemon from: {url}");

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Http(e.to_string()))?;

        if !response.status().is_success() {
            return Err(DomainError::Http(format!(
                "HTTP {} from {}: {}",
                response.status(),
                url,
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            )));
        }

        let raw: PokemonResponse = response
            .json()
            .await
            .map_err(|e| DomainError::Parse(e.to_string()))?;

        tracing::info!("Successfully fetched pokemon: {:?}", raw);

        Ok(Pokemon {
            id: raw.id,
            name: raw.name,
            height: raw.height,
        })
    }
}
