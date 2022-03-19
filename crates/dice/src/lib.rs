//! # Dice
//!
//! `dice` contains everything you need to roll some dice.
//! Supports d4, d6, d8, d10, d12, d20, d100
#![warn(clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Available dice types for rolling
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl Die {
    /// Roll the die and return the result
    ///
    /// ```
    /// use dice::Die;
    ///
    /// let mut rng = rand::thread_rng();
    /// let roll = Die::D20.roll(&mut rng);
    ///
    /// assert!((1..=20).contains(&roll));
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn roll(self, rng: &mut impl Rng) -> u32 {
        let roll = rng.gen_range(1u32..=self.into());

        metrics::increment_counter!(
            "dice_roll_total",
            &[("die", self.to_string()), ("roll", roll.to_string())]
        );

        roll
    }

    /// Roll a number of a given dice and return the results
    ///
    /// ```
    /// use dice::Die;
    ///
    /// let mut rng = rand::thread_rng();
    /// let rolls = Die::D20.roll_multiple(&mut rng, 2);
    ///
    /// assert_eq!(rolls.len(), 2);
    /// assert!(rolls.iter().all(|roll| (1..=20).contains(roll)));
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn roll_multiple(self, rng: &mut impl Rng, amount: usize) -> Vec<u32> {
        (1..=amount).map(|_| self.roll(rng)).collect_vec()
    }
}

impl From<Die> for u32 {
    /// Number of sides for a given die
    fn from(die: Die) -> Self {
        match die {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
        }
    }
}
