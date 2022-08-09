//! # Dice
//!
//! `dice` contains everything you need to roll some dice.
//! Supports d4, d6, d8, d10, d12, d20, d100
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

use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Available dice types for rolling
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Die {
    /// Four-sided die
    D4,
    /// Six-sided die
    D6,
    /// Eight-sided die
    D8,
    /// Ten-sided die
    D10,
    /// Twelve-sided die
    D12,
    /// Twenty-sided die
    D20,
    /// Equivalent of rolling two d10s for a d100 table.
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
    pub fn roll<R: Rng + ?Sized>(self, rng: &mut R) -> u8 {
        let roll = rng.gen_range(1u8..=self.into());

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
    /// assert_eq!(rolls.count(), 2);
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn roll_multiple<R: Rng + ?Sized>(
        self,
        rng: &mut R,
        amount: usize,
    ) -> impl Iterator<Item = u8> + '_ {
        (1..=amount).map(move |_| self.roll(rng))
    }
}

macro_rules! impl_int_from_die {
    ($($int_type: ty),*) => {
        $(
            impl From<Die> for $int_type {
                fn from(die: Die) -> $int_type {
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
        )*
    }
}

impl_int_from_die!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_float_from_die {
    ($($float_type: ty),*) => {
        $(
            impl From<Die> for $float_type {
                fn from(die: Die) -> $float_type {
                    match die {
                        Die::D4 => 4.0,
                        Die::D6 => 6.0,
                        Die::D8 => 8.0,
                        Die::D10 => 10.0,
                        Die::D12 => 12.0,
                        Die::D20 => 20.0,
                        Die::D100 => 100.0,
                    }
                }
            }
        )*
    }
}

impl_float_from_die!(f32, f64);

/// A way to represent a roll that should be carried out.
/// Programmatic way of representing 2d6 for example.
///
/// ```
/// use dice::{Die, Roll};
///
/// let mut rng = rand::thread_rng();
/// let rolls = Roll::new(2, Die::D6).gen(&mut rng);
///
/// assert_eq!(rolls.count(), 2);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Roll {
    /// Amount of die to be rolled
    amount: usize,
    /// Die to use in the roll
    die: Die,
}

impl Roll {
    /// Create a new roll to be rolled later
    ///
    /// ```
    /// use dice::{Die, Roll};
    ///
    /// let roll = Roll::new(2, Die::D8);
    /// ````
    #[must_use]
    pub fn new(amount: usize, die: Die) -> Self {
        Self { amount, die }
    }

    /// Roll the specified dice
    ///
    /// ```
    /// use dice::{Die, Roll};
    ///
    /// let mut rng = rand::thread_rng();
    /// let rolls = Roll::new(2, Die::D6).gen(&mut rng);
    ///
    /// assert_eq!(rolls.count(), 2);
    /// ```
    pub fn gen<R: Rng + ?Sized>(self, rng: &mut R) -> impl Iterator<Item = u8> + '_ {
        self.die.roll_multiple(rng, self.amount)
    }
}
