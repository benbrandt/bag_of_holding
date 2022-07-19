//! # Characters
//!
//! Crate to generate entire characters. Assembles together all of the other
//! crates together into a final character sheet.
#![warn(
    clippy::pedantic,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
)]

use abilities::AbilityScores;
use rand::{distributions::Standard, prelude::Distribution, Rng};

/// Full character information.
#[derive(Debug)]
pub struct Character {
    /// Ability scores of the character
    pub ability_scores: AbilityScores,
}

impl Distribution<Character> for Standard {
    /// Generate a fully random character.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        Character {
            ability_scores: rng.gen::<AbilityScores>(),
        }
    }
}
