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

use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
};

use dice::Die;
use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom, Rng};
use rand_utils::exp_weight;
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

/// An individual Ability Score value. Either base or increase.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AbilityScore {
    /// Ability this score is to be used for
    ability: Ability,
    /// Score to add to total for this ability
    score: u8,
}

impl AbilityScore {
    /// Create a new ability score
    fn new(ability: Ability, score: u8) -> Self {
        Self { ability, score }
    }
}

/// An individual score value and its corresponding modifier
///
/// Cached so that the modifier doesn't have to be calculated constantly.
#[derive(Clone, Copy, Debug, Serialize)]
struct AbilityScoreTotal {
    /// Base score rolled at character creation
    base: u8,
    /// Increase provided by race
    racial_increase: u8,
    /// Total score for this ability
    score: u8,
    /// Modifier derived from the total score
    modifier: i8,
}

impl AbilityScoreTotal {
    /// Construct a new ability score and modifier for a given score.
    ///
    /// # Panics
    ///
    /// Will only panic if we somehow generate an invalid value for D&D
    fn new(base: u8, racial_increase: u8) -> Self {
        let score = base + racial_increase;
        let i_score: i8 = score.try_into().unwrap();
        Self {
            base,
            racial_increase,
            score,
            // Lower value to closest even number, reduce by 10, and divide by two
            modifier: (i_score - i_score % 2 - 10) / 2,
        }
    }
}

/// A collection of ability scores
#[derive(Clone, Debug, Serialize)]
#[serde(into = "AbilityScoreStats")]
pub struct AbilityScores {
    /// Base scores generated for this character
    base_scores: HashSet<AbilityScore>,
    /// Ability score increases chosen for the race
    racial_increases: HashSet<AbilityScore>,
}

impl AbilityScores {
    /// Generate a new set of ability scores with a given set of base scores.
    ///
    /// Most likely you will generate this with `rng.gen()`, but can be created
    /// manually as well if necessary.
    #[must_use]
    pub fn new(base_scores: HashSet<AbilityScore>) -> Self {
        Self {
            base_scores,
            racial_increases: HashSet::new(),
        }
    }

    /// Internal method to access a given ability score
    ///
    /// # Panics
    ///
    /// Will panic if a score for the ability doesn't exist (because it should)
    fn ability(&self, ability: Ability) -> AbilityScoreTotal {
        let base = self
            .base_scores
            .iter()
            .find(|s| s.ability == ability)
            .unwrap_or_else(|| panic!("Ability score missing for {ability}"))
            .score;

        let racial_increase = self
            .racial_increases
            .iter()
            .find(|s| s.ability == ability)
            .map(|s| s.score)
            .unwrap_or_default();

        AbilityScoreTotal::new(base, racial_increase)
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
    pub fn score(&self, ability: Ability) -> u8 {
        self.ability(ability).score
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
    pub fn modifier(&self, ability: Ability) -> i8 {
        self.ability(ability).modifier
    }

    /// Method to get the weight for a particular ability when doing a `choose_weighted` call.
    #[tracing::instrument]
    fn weight(&self, ability: Ability) -> f64 {
        let min_modifier = Ability::iter()
            .map(|a| self.modifier(a))
            .min()
            .expect("No ability scores present");
        let modifier = self.modifier(ability);

        // Subtract min modifier from this to offset by minimum score.
        exp_weight(modifier - min_modifier)
    }

    /// Choose a single racial increase.
    ///
    /// Will weight choices where possible towards applying increases to
    /// ability scores that would cause in increase in the modifier.
    ///
    /// # Panics
    ///
    /// Panics if an increase cannot be chosen, which shouldn't be possible.
    #[tracing::instrument(skip(rng))]
    fn gen_racial_increase<R: Rng + ?Sized>(&mut self, rng: &mut R, increase: u8) -> &mut Self {
        // Get all abilities that haven't already been chosen for racial increases.
        let abilities = Ability::iter().collect::<HashSet<_>>();
        let current_racial_increases = self.racial_increases.iter().map(|i| i.ability).collect();
        let remaining_abilities = abilities.difference(&current_racial_increases);

        // Filter out options that aren't valid choices
        let all_ability_choices = remaining_abilities
            .copied()
            .filter(|&a| self.ability(a).score + increase <= 20)
            .collect::<Vec<_>>();
        // See if any would cause an increase in modifier score
        let optimal_ability_choices = all_ability_choices
            .iter()
            .copied()
            .filter(|&a| self.ability(a).score % 2 == increase % 2)
            .collect::<Vec<_>>();

        // Choose from optimal choices if available, otherwise, choose any of them. Weighted by current modifier
        let ability = if optimal_ability_choices.is_empty() {
            all_ability_choices.as_slice()
        } else {
            optimal_ability_choices.as_slice()
        }
        .choose_weighted(rng, |&a| self.weight(a))
        .unwrap();

        self.racial_increases
            .insert(AbilityScore::new(*ability, increase));

        self
    }

    /// Choose racial increases for this character.
    ///
    /// Will weight choices where possible towards applying increases to
    /// ability scores that would cause in increase in the modifier.
    ///
    /// ```
    /// use abilities::AbilityScores;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let mut scores: AbilityScores = rng.gen();
    /// scores.gen_racial_increases(&mut rng, &[2, 1]);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if an increase cannot be chosen, which shouldn't be possible.
    #[tracing::instrument(skip(rng))]
    pub fn gen_racial_increases<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        increases: &[u8],
    ) -> &mut Self {
        for &increase in increases {
            self.gen_racial_increase(rng, increase);
        }

        self
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
        AbilityScores::new(
            Ability::iter()
                .map(|a| {
                    let score: u8 = Die::D6.roll_multiple(rng, 4).sorted().rev().take(3).sum();

                    metrics::increment_counter!(
                        "abilities_score",
                        &[("ability", a.to_string()), ("score", score.to_string())]
                    );

                    AbilityScore::new(a, score)
                })
                .collect(),
        )
    }
}

/// Serializable version of the ability scores (usually for character sheets)
/// A `BTreeMap` so that it stays in the same order
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
struct AbilityScoreStats(BTreeMap<Ability, AbilityScoreTotal>);

impl From<AbilityScores> for AbilityScoreStats {
    fn from(scores: AbilityScores) -> Self {
        Self(Ability::iter().map(|a| (a, scores.ability(a))).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn modifier_logic() {
        assert_eq!(AbilityScoreTotal::new(0, 0).modifier, -5);
        assert_eq!(AbilityScoreTotal::new(1, 0).modifier, -5);
        assert_eq!(AbilityScoreTotal::new(2, 0).modifier, -4);
        assert_eq!(AbilityScoreTotal::new(3, 0).modifier, -4);
        assert_eq!(AbilityScoreTotal::new(4, 0).modifier, -3);
        assert_eq!(AbilityScoreTotal::new(5, 0).modifier, -3);
        assert_eq!(AbilityScoreTotal::new(6, 0).modifier, -2);
        assert_eq!(AbilityScoreTotal::new(7, 0).modifier, -2);
        assert_eq!(AbilityScoreTotal::new(8, 0).modifier, -1);
        assert_eq!(AbilityScoreTotal::new(9, 0).modifier, -1);
        assert_eq!(AbilityScoreTotal::new(10, 0).modifier, 0);
        assert_eq!(AbilityScoreTotal::new(11, 0).modifier, 0);
        assert_eq!(AbilityScoreTotal::new(12, 0).modifier, 1);
        assert_eq!(AbilityScoreTotal::new(13, 0).modifier, 1);
        assert_eq!(AbilityScoreTotal::new(14, 0).modifier, 2);
        assert_eq!(AbilityScoreTotal::new(15, 0).modifier, 2);
        assert_eq!(AbilityScoreTotal::new(16, 0).modifier, 3);
        assert_eq!(AbilityScoreTotal::new(17, 0).modifier, 3);
        assert_eq!(AbilityScoreTotal::new(18, 0).modifier, 4);
        assert_eq!(AbilityScoreTotal::new(19, 0).modifier, 4);
        assert_eq!(AbilityScoreTotal::new(20, 0).modifier, 5);
    }
}
