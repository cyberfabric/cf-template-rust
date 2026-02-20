use crate::domain::{Data, DataRepository};
use cf_modkit_http::Client;

mod dto;
use dto::DataDto;

/// Default timeout for HTTP requests in seconds
const DEFAULT_TIMEOUT_SECS: u64 = 10;

/// HTTP client implementation for fetching data
pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

/// Default implementation using template placeholder.
///
/// **Note**: This uses the `{{http_url}}` placeholder which will be replaced
/// by cargo-generate during template instantiation. If using this code
/// directly without cargo-generate, use `HttpClient::new(url)` instead.
impl Default for HttpClient {
    fn default() -> Self {
        Self::new("{{http_url}}".to_string())
    }
}

#[modkit::async_trait]
impl DataRepository for HttpClient {
    async fn fetch_data(&self) -> modkit::Result<Data> {
        tracing::debug!("Fetching data from: {}", self.base_url);

        let response = self.client
            .get(&self.base_url)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!(
                "HTTP error {} from {}: {}",
                response.status(),
                self.base_url,
                response.status().canonical_reason().unwrap_or("Unknown error")
            );
        }

        // Deserialize into DTO (transport layer)
        let dto: DataDto = response.json().await?;

        // Convert DTO to domain model
        let data: Data = dto.into();

        tracing::debug!("Successfully fetched data: {:?}", data);

        Ok(data)
    }
}
