use serde::{Deserialize, Serialize};

/// Configuration for the pokemon module
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PokemonConfig {
    #[serde(default = "default_page_size")]
    pub default_page_size: u32,
    #[serde(default = "default_max_page_size")]
    pub max_page_size: u32,
}

impl Default for PokemonConfig {
    fn default() -> Self {
        Self {
            default_page_size: default_page_size(),
            max_page_size: default_max_page_size(),
        }
    }
}

fn default_page_size() -> u32 {
    50
}

fn default_max_page_size() -> u32 {
    1000
}
