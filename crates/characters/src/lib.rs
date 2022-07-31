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
use races::Race;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::Serialize;
use sources::Sources;

/// Full character information.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(into = "CharacterSheet")]
pub struct Character {
    /// Ability scores of the character
    pub ability_scores: Option<AbilityScores>,
    /// Race of the character
    pub race: Option<Race>,
}

impl Character {
    /// Creates a new default [`Character`].
    ///
    /// ```
    /// use characters::Character;
    ///
    /// let character = Character::new();
    /// ```
    #[must_use]
    #[tracing::instrument]
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate and add base ability scores for your character.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new().gen_ability_scores(&mut rng);
    /// ```
    #[must_use]
    #[tracing::instrument(skip(rng))]
    pub fn gen_ability_scores<R: Rng + ?Sized>(mut self, rng: &mut R) -> Self {
        self.ability_scores = Some(rng.gen::<AbilityScores>());
        self
    }

    /// Generate a race for your character.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new().gen_ability_scores(&mut rng).gen_race(&mut rng);
    /// ```
    #[must_use]
    #[tracing::instrument(skip(rng))]
    pub fn gen_race<R: Rng + ?Sized>(mut self, rng: &mut R) -> Self {
        self.race = Some(rng.gen::<Race>());
        self
    }
}

impl Distribution<Character> for Standard {
    /// Generate a fully random character.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        Character::new().gen_ability_scores(rng).gen_race(rng)
    }
}

/// Serializable, public interface for a character
#[derive(Serialize)]
struct CharacterSheet {
    /// Ability scores of the character
    pub ability_scores: Option<AbilityScores>,
    /// Chosen race of the character
    pub race: Option<String>,
}

impl From<Character> for CharacterSheet {
    fn from(character: Character) -> Self {
        Self {
            ability_scores: character.ability_scores,
            race: character.race.map(|r| r.citation()),
        }
    }
}
