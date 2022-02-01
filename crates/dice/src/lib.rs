//! # Dice
//!
//! `dice` contains everything you need to roll some dice.
//! Supports d4, d6, d8, d10, d12, d20, d100

use itertools::Itertools;
use rand::Rng;

/// Result of a di(c)e roll
pub struct Roll {
    /// Individual rolls for each die rolled
    pub die_rolls: Vec<u32>,
    /// Sum of all die rolls
    pub total: u32,
}

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
    pub fn roll(&self, rng: &mut impl Rng) -> u32 {
        rng.gen_range(1..self.sides())
    }

    /// Roll a number of a given dice and return the results
    pub fn roll_multiple(&self, rng: &mut impl Rng, amount: usize) -> Vec<u32> {
        (1..=amount).map(|_| self.roll(rng)).collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn roll_d4() {
        let mut rng = Pcg64::from_entropy();
        let roll = Die::D4.roll(&mut rng);
        assert!((1..=4).contains(&roll));
    }

    #[test]
    fn roll_multiple_d4() {
        let mut rng = Pcg64::from_entropy();
        // Roll a normal range of dice
        for i in 1..=12 {
            let rolls = Die::D4.roll_multiple(&mut rng, i);
            assert_eq!(rolls.len(), i as usize);
            assert!(rolls.iter().all(|d| (1..=4).contains(d)));
        }
    }
}
