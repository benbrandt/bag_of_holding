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
    #[strum(serialize = "STR")]
    Strength,
    /// Measuring agility
    #[strum(serialize = "DEX")]
    Dexterity,
    /// Measuring endurance
    #[strum(serialize = "CON")]
    Constitution,
    /// Measuring reasoning and memory
    #[strum(serialize = "INT")]
    Intelligence,
    /// Measuring perception and insight
    #[strum(serialize = "WIS")]
    Wisdom,
    /// Measuring force of personality
    #[strum(serialize = "CHA")]
    Charisma,
}

/// A collection of ability scores
#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct AbilityScores(BTreeMap<Ability, i8>);

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
                    (
                        a,
                        Die::D6
                            .roll_multiple(rng, 4)
                            .sorted()
                            .rev()
                            .take(3)
                            .sum::<u8>()
                            .try_into()
                            .unwrap(),
                    )
                })
                .collect(),
        )
    }

    /// Get a specific ability score.
    ///
    /// ```
    /// use abilities::{Ability, AbilityScores};
    ///
    /// let mut rng = rand::thread_rng();
    /// let scores = AbilityScores::gen(&mut rng);
    /// let strength = scores.score(&Ability::Strength);
    ///
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if a score for the ability doesn't exist (because it should)
    #[must_use]
    #[tracing::instrument]
    pub fn score(&self, ability: &Ability) -> i8 {
        *self
            .0
            .get(ability)
            .unwrap_or_else(|| panic!("Ability score missing for {ability}"))
    }
}
