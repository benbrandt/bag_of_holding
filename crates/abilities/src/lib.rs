//! # Ability Scores
//!
//! Almost everything in D&D ultimately revolves around your characters'
//! abilities.
//!
//! This crate contains ability types, ability scores, modifiers, and more.
//!
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

use std::collections::BTreeMap;

use dice::Die;
use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

/// Available Ability types
/// Ordered in the same order as a character sheet.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub enum Ability {
    /// Measuring physical power
    #[serde(rename = "STR")]
    #[strum(serialize = "STR")]
    Strength,
    /// Measuring agility
    #[serde(rename = "DEX")]
    #[strum(serialize = "DEX")]
    Dexterity,
    /// Measuring endurance
    #[serde(rename = "CON")]
    #[strum(serialize = "CON")]
    Constitution,
    /// Measuring reasoning and memory
    #[serde(rename = "INT")]
    #[strum(serialize = "INT")]
    Intelligence,
    /// Measuring perception and insight
    #[serde(rename = "WIS")]
    #[strum(serialize = "WIS")]
    Wisdom,
    /// Measuring force of personality
    #[serde(rename = "CHA")]
    #[strum(serialize = "CHA")]
    Charisma,
}

/// An individual score value and its corresponding modifier
///
/// Cached so that the modifier doesn't have to be calculated constantly.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct AbilityScoreTotal {
    score: u8,
    modifier: i8,
}

impl AbilityScoreTotal {
    /// Construct a new ability score and modifier for a given score.
    ///
    /// # Panics
    ///
    /// Will only panic if we somehow generate an invalid value for D&D
    #[tracing::instrument]
    fn new(score: u8) -> Self {
        let i_score: i8 = score.try_into().unwrap();
        Self {
            score,
            // Lower value to closest even number, reduce by 10, and divide by two
            modifier: (i_score - i_score % 2 - 10) / 2,
        }
    }
}

impl Distribution<AbilityScoreTotal> for Standard {
    /// Generate base ability score for a character.
    /// The result of rolling 4d6 and taking the top 3 dice.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AbilityScoreTotal {
        AbilityScoreTotal::new(Die::D6.roll_multiple(rng, 4).sorted().rev().take(3).sum())
    }
}

/// A collection of ability scores
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AbilityScores {
    /// Cached calculated total scores and modifiers
    cache: BTreeMap<Ability, AbilityScoreTotal>,
}

impl AbilityScores {
    /// Internal method to access a given ability score
    ///
    /// # Panics
    ///
    /// Will panic if a score for the ability doesn't exist (because it should)
    #[tracing::instrument]
    fn ability_score(&self, ability: Ability) -> &AbilityScoreTotal {
        self.cache
            .get(&ability)
            .unwrap_or_else(|| panic!("Ability score missing for {ability}"))
    }

    /// Get a specific ability score.
    ///
    /// ```
    /// use abilities::{Ability, AbilityScores};
    /// use rand::Rng;
    ///
    /// let scores: AbilityScores = rand::thread_rng().gen();
    /// let strength = scores.score(Ability::Strength);
    ///
    /// ```
    #[must_use]
    #[tracing::instrument]
    pub fn score(&self, ability: Ability) -> u8 {
        self.ability_score(ability).score
    }

    /// Get a modifiers for a specific ability. Will be between -5 and 5
    ///
    /// ```
    /// use abilities::{Ability, AbilityScores};
    /// use rand::Rng;
    ///
    /// let scores: AbilityScores = rand::thread_rng().gen();
    /// let strength_mod = scores.modifier(Ability::Strength);
    ///
    /// ```
    #[must_use]
    #[tracing::instrument]
    pub fn modifier(&self, ability: Ability) -> i8 {
        self.ability_score(ability).modifier
    }
}

impl Distribution<AbilityScores> for Standard {
    /// Generate base ability scores for a character.
    /// Each ability is the result of rolling 4d6 and taking the top 3.
    ///
    /// ```
    /// use abilities::AbilityScores;
    /// use rand::Rng;
    ///
    /// let scores: AbilityScores = rand::thread_rng().gen();
    /// ```
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AbilityScores {
        AbilityScores {
            cache: Ability::iter()
                .map(|a| {
                    let score: AbilityScoreTotal = rng.gen();

                    metrics::increment_counter!(
                        "abilities_score",
                        &[
                            ("ability", a.to_string()),
                            ("score", score.score.to_string())
                        ]
                    );

                    (a, score)
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn modifier_logic() {
        assert_eq!(AbilityScoreTotal::new(0).modifier, -5);
        assert_eq!(AbilityScoreTotal::new(1).modifier, -5);
        assert_eq!(AbilityScoreTotal::new(2).modifier, -4);
        assert_eq!(AbilityScoreTotal::new(3).modifier, -4);
        assert_eq!(AbilityScoreTotal::new(4).modifier, -3);
        assert_eq!(AbilityScoreTotal::new(5).modifier, -3);
        assert_eq!(AbilityScoreTotal::new(6).modifier, -2);
        assert_eq!(AbilityScoreTotal::new(7).modifier, -2);
        assert_eq!(AbilityScoreTotal::new(8).modifier, -1);
        assert_eq!(AbilityScoreTotal::new(9).modifier, -1);
        assert_eq!(AbilityScoreTotal::new(10).modifier, 0);
        assert_eq!(AbilityScoreTotal::new(11).modifier, 0);
        assert_eq!(AbilityScoreTotal::new(12).modifier, 1);
        assert_eq!(AbilityScoreTotal::new(13).modifier, 1);
        assert_eq!(AbilityScoreTotal::new(14).modifier, 2);
        assert_eq!(AbilityScoreTotal::new(15).modifier, 2);
        assert_eq!(AbilityScoreTotal::new(16).modifier, 3);
        assert_eq!(AbilityScoreTotal::new(17).modifier, 3);
        assert_eq!(AbilityScoreTotal::new(18).modifier, 4);
        assert_eq!(AbilityScoreTotal::new(19).modifier, 4);
        assert_eq!(AbilityScoreTotal::new(20).modifier, 5);
    }
}
