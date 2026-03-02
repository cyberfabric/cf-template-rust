use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub height: u32,
    pub name: String,
}
