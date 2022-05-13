//! # Ability Scores
//!
//! Almost everything in D&D ultimately revolves around your characters'
//! abilities.
//!
//! This crate contains ability types, ability scores, modifiers, and more.
//!
#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use std::collections::BTreeMap;

use dice::Die;
use itertools::Itertools;
use rand::Rng;
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
#[derive(Debug, Deserialize, Serialize)]
struct AbilityScore {
    score: u8,
    modifier: i8,
}

impl AbilityScore {
    /// Construct a new ability score and modifier for a given score.
    #[tracing::instrument]
    fn new(score: u8) -> Self {
        let i_score: i8 = score.try_into().unwrap();
        Self {
            score,
            // Lower value to closest even number, reduce by 10, and divide by two
            modifier: (i_score - i_score % 2 - 10) / 2,
        }
    }

    /// Generate base ability score for a character.
    /// The result of rolling 4d6 and taking the top 3 dice.
    ///
    /// # Panics
    ///
    /// Will only panic if we somehow generate an invalid value for D&D
    #[tracing::instrument(skip(rng))]
    fn gen(rng: &mut impl Rng) -> Self {
        Self::new(Die::D6.roll_multiple(rng, 4).sorted().rev().take(3).sum())
    }
}

/// A collection of ability scores
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AbilityScores(BTreeMap<Ability, AbilityScore>);

impl AbilityScores {
    /// Generate base ability scores for a character.
    /// Each ability is the result of rolling 4d6 and taking the top 3.
    ///
    /// ```
    /// use abilities::AbilityScores;
    ///
    /// let mut rng = rand::thread_rng();
    /// let scores = AbilityScores::gen(&mut rng);
    /// ```
    ///
    /// # Panics
    ///
    /// Will only panic if we somehow generate an invalid value for D&D
    #[tracing::instrument(skip(rng))]
    pub fn gen(rng: &mut impl Rng) -> Self {
        Self(
            Ability::iter()
                .map(|a| {
                    let score = AbilityScore::gen(rng);

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
        )
    }

    /// Internal method to access a given ability score
    ///
    /// # Panics
    ///
    /// Will panic if a score for the ability doesn't exist (because it should)
    #[tracing::instrument]
    fn ability_score(&self, ability: Ability) -> &AbilityScore {
        self.0
            .get(&ability)
            .unwrap_or_else(|| panic!("Ability score missing for {ability}"))
    }

    /// Get a specific ability score.
    ///
    /// ```
    /// use abilities::{Ability, AbilityScores};
    ///
    /// let mut rng = rand::thread_rng();
    /// let scores = AbilityScores::gen(&mut rng);
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
    ///
    /// let mut rng = rand::thread_rng();
    /// let scores = AbilityScores::gen(&mut rng);
    /// let strength_mod = scores.modifier(Ability::Strength);
    ///
    /// ```
    #[must_use]
    #[tracing::instrument]
    pub fn modifier(&self, ability: Ability) -> i8 {
        self.ability_score(ability).modifier
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn modifier_logic() {
        assert_eq!(AbilityScore::new(0).modifier, -5);
        assert_eq!(AbilityScore::new(1).modifier, -5);
        assert_eq!(AbilityScore::new(2).modifier, -4);
        assert_eq!(AbilityScore::new(3).modifier, -4);
        assert_eq!(AbilityScore::new(4).modifier, -3);
        assert_eq!(AbilityScore::new(5).modifier, -3);
        assert_eq!(AbilityScore::new(6).modifier, -2);
        assert_eq!(AbilityScore::new(7).modifier, -2);
        assert_eq!(AbilityScore::new(8).modifier, -1);
        assert_eq!(AbilityScore::new(9).modifier, -1);
        assert_eq!(AbilityScore::new(10).modifier, 0);
        assert_eq!(AbilityScore::new(11).modifier, 0);
        assert_eq!(AbilityScore::new(12).modifier, 1);
        assert_eq!(AbilityScore::new(13).modifier, 1);
        assert_eq!(AbilityScore::new(14).modifier, 2);
        assert_eq!(AbilityScore::new(15).modifier, 2);
        assert_eq!(AbilityScore::new(16).modifier, 3);
        assert_eq!(AbilityScore::new(17).modifier, 3);
        assert_eq!(AbilityScore::new(18).modifier, 4);
        assert_eq!(AbilityScore::new(19).modifier, 4);
        assert_eq!(AbilityScore::new(20).modifier, 5);
    }
}
