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

    /// Given a RNG, and a number of dice to roll, roll the dice and return the result
    pub fn roll(&self, rng: &mut impl Rng, num: u32) -> Roll {
        let die_rolls = (1..=num)
            .map(|_| rng.gen_range(1..=self.sides()))
            .collect_vec();

        Roll {
            total: die_rolls.iter().sum(),
            die_rolls,
        }
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
        // Roll a normal range of dice
        for i in 1..=12 {
            let roll = Die::D4.roll(&mut rng, i);
            assert_eq!(roll.die_rolls.len(), i as usize);
            assert!(roll.die_rolls.iter().all(|d| (1..=4).contains(d)));
            assert!((i..=4 * i).contains(&roll.total));
        }
    }
}
