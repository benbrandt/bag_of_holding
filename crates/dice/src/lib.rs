//! # Dice
//!
//! `dice` contains everything you need to roll some dice.
//! Supports d4, d6, d8, d10, d12, d20, d100

use itertools::Itertools;
use rand::Rng;

/// Available dice types for rolling
pub enum Die {
    D4,
}

impl Die {
    /// Number of sides for a given die
    fn sides(&self) -> u32 {
        match self {
            Die::D4 => 4,
        }
    }

    /// Roll the die and return the result
    ///
    /// ```
    /// use dice::Die;
    ///
    /// let mut rng = rand::thread_rng();
    /// let roll = Die::D4.roll(&mut rng);
    ///
    /// assert!((1..=4).contains(&roll));
    /// ```
    pub fn roll(&self, rng: &mut impl Rng) -> u32 {
        rng.gen_range(1..self.sides())
    }

    /// Roll a number of a given dice and return the results
    ///
    /// ```
    /// use dice::Die;
    ///
    /// let mut rng = rand::thread_rng();
    /// let rolls = Die::D4.roll_multiple(&mut rng, 2);
    ///
    /// assert_eq!(rolls.len(), 2);
    /// assert!(rolls.iter().all(|roll| (1..=4).contains(roll)));
    /// ```
    pub fn roll_multiple(&self, rng: &mut impl Rng, amount: usize) -> Vec<u32> {
        (1..=amount).map(|_| self.roll(rng)).collect_vec()
    }
}
