use serde::{Deserialize, Serialize};
use crate::domain::Data;

/// Data Transfer Object for HTTP serialization/deserialization
/// 
/// This DTO handles the transport layer concerns (JSON serialization)
/// and is separate from the domain model to maintain clean architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDto {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

/// Convert from DTO to domain model
impl From<DataDto> for Data {
    fn from(dto: DataDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            value: dto.value,
        }
    }
}

/// Convert from domain model to DTO
impl From<Data> for DataDto {
    fn from(data: Data) -> Self {
        Self {
            id: data.id,
            name: data.name,
            value: data.value,
        }
    }
}
