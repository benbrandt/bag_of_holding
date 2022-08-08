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
    f64::consts::E,
    hash::Hash,
};

use dice::Die;
use itertools::Itertools;
use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom, Rng};
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
    #[tracing::instrument]
    fn new(ability: Ability, score: u8) -> Self {
        Self { ability, score }
    }
}

/// An individual score value and its corresponding modifier
///
/// Cached so that the modifier doesn't have to be calculated constantly.
#[derive(Clone, Copy, Debug, Serialize)]
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

/// A collection of ability scores
#[derive(Clone, Debug, Serialize)]
#[serde(into = "AbilityScoreStats")]
pub struct AbilityScores {
    /// Base scores generated for this character
    base_scores: HashSet<AbilityScore>,
    /// Ability score increases chosen for the race
    racial_increases: HashSet<AbilityScore>,
    /// Cached calculated total scores and modifiers
    cache: BTreeMap<Ability, AbilityScoreTotal>,
}

impl AbilityScores {
    /// Generate a new set of ability scores with a given set of base scores.
    ///
    /// Most likely you will generate this with `rng.gen()`, but can be created
    /// manually as well if necessary.
    #[must_use]
    #[tracing::instrument]
    pub fn new(base_scores: HashSet<AbilityScore>) -> Self {
        let mut scores = Self {
            base_scores,
            racial_increases: HashSet::new(),
            cache: BTreeMap::new(),
        };
        // Generate cache
        scores.regenerate_cache();
        scores
    }

    /// Recalculate cached values by summing up all scores and increases
    #[tracing::instrument]
    fn regenerate_cache(&mut self) -> &mut Self {
        self.cache = Ability::iter()
            .map(|a| {
                let base = self
                    .base_scores
                    .iter()
                    .find(|s| s.ability == a)
                    .unwrap_or_else(|| panic!("Ability score missing for {a}"))
                    .score;

                let increase = self
                    .racial_increases
                    .iter()
                    .find(|s| s.ability == a)
                    .map(|s| s.score)
                    .unwrap_or_default();

                (a, AbilityScoreTotal::new(base + increase))
            })
            .collect();
        self
    }

    /// Internal method to access a given ability score
    ///
    /// # Panics
    ///
    /// Will panic if a score for the ability doesn't exist (because it should)
    #[tracing::instrument]
    fn ability(&self, ability: Ability) -> &AbilityScoreTotal {
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
    #[tracing::instrument]
    pub fn modifier(&self, ability: Ability) -> i8 {
        self.ability(ability).modifier
    }

    /// Method to get the weight for a particular ability when doing a `choose_weighted` call.
    fn weight(&self, ability: Ability) -> f64 {
        let min_modifier = Ability::iter()
            .map(|a| self.modifier(a))
            .min()
            .expect("No ability scores present");
        let modifier = self.modifier(ability);

        // Subtract min modifier from this to offset by minimum score.
        E.powi(i32::from(modifier - min_modifier))
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
    pub fn gen_racial_increases<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        increases: &[u8],
    ) -> &mut Self {
        let mut abilities = Ability::iter().collect::<HashSet<_>>();

        for increase in increases {
            let all_ability_choices = abilities
                .iter()
                .copied()
                // Filter out options that aren't valid
                .filter(|&a| self.ability(a).score + increase <= 20)
                .collect::<Vec<_>>();
            let optimal_ability_choices = all_ability_choices
                .iter()
                .copied()
                // See if any would cause an increase in modifier score
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

            abilities.remove(ability);
            self.racial_increases
                .insert(AbilityScore::new(*ability, *increase));
        }

        self.regenerate_cache();
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
        Self(scores.cache)
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
