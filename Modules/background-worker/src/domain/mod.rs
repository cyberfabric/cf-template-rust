/// Domain model representing the fetched data
/// 
/// The #[domain_model] macro enforces DDD boundaries at compile-time,
/// preventing the use of infrastructure types in domain models.
#[cf_modkit_macros::domain_model]
#[derive(Debug, Clone)]
pub struct Data {
    // TODO: Define your data structure based on the API response
    // Example fields:
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
}

impl Data {
    pub fn summary(&self) -> String {
        format!(
            "Data: id={:?}, name={:?}",
            self.id, self.name
        )
    }
}

/// Repository trait for fetching data
/// This allows for different implementations (HTTP, mock, cache, etc.)
#[modkit::async_trait]
pub trait DataRepository: Send + Sync {
    async fn fetch_data(&self) -> modkit::Result<Data>;
}
