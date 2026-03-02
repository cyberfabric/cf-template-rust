use anyhow::{Context, bail};
use modkit_http::HttpClient;
use std::time::{Duration, UNIX_EPOCH};

mod model;
use model::Pokemon;

/// HTTP client implementation for fetching data
pub struct Client {
    client: HttpClient,
}

const API_URL: &str = "https://pokeapi.co/api/v2/pokemon/";

impl Client {
    pub fn new() -> modkit::Result<Self> {
        Ok(Self {
            client: HttpClient::builder()
                .no_redirects()
                .timeout(Duration::from_secs(5))
                .build()
                .context("problem while building http client")?,
        })
    }
    pub async fn fetch_data(&self) -> modkit::Result<Pokemon> {
        let url = format!(
            "{}{}",
            API_URL,
            (UNIX_EPOCH.elapsed()?.subsec_nanos() % 150) + 1
        );
        tracing::debug!("Fetching data from: {url}");

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            bail!(
                "HTTP error {} from {}: {}",
                response.status(),
                url,
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            );
        }

        let data: Pokemon = response.json().await?;

        tracing::info!("Successfully fetched data: {:?}", data);

        Ok(data)
    }
}
