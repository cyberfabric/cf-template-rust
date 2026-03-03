//! Public models for the `background-worker` module.
//!
//! Transport-agnostic data structures that define the contract
//! between the `background-worker` module and its consumers.

/// A pokemon entity fetched from the PokeAPI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pokemon {
    pub id: u32,
    pub height: u32,
    pub name: String,
}
