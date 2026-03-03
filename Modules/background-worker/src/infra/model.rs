use serde::{Deserialize, Serialize};

/// Raw PokeAPI response shape. Kept internal to the infra layer.
/// Mapped to `background_worker_sdk::Pokemon` before crossing the domain boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonResponse {
    pub id: u32,
    pub height: u32,
    pub name: String,
}
