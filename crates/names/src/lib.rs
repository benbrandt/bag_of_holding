//! # Names
//!
//! Generate names for any race in the D&D multiverse.
#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use rand::{prelude::IteratorRandom, Rng};
use serde::Serialize;
use strum::{Display, EnumIter, IntoEnumIterator};

mod dwarf;

pub use dwarf::Dwarf;

/// Implements the ability to generate a name for a given race.
/// Can contain whatever information is necessary for a given name
/// (such as gender, ethnicity, child names, etc)
pub trait Name {
    /// Generate a name with a given rng
    fn gen(rng: &mut impl Rng) -> Self;
}

/// Some races have names that are usually assigned to a gender.
/// This is only used to randomly decide which list to choose, and surface
/// the relation. Doesn't decide the gender of the character though.
#[derive(Debug, Display, EnumIter, Serialize)]
pub enum Gender {
    /// Used to choose a name that is generally for a female.
    Female,
    /// Used to choose a name that is generally for a male.
    Male,
}

impl Gender {
    /// Choose a random gender for choosing between name lists.
    #[tracing::instrument(skip(rng))]
    fn gen(rng: &mut impl Rng) -> Self {
        let gender = Self::iter().choose(rng).unwrap();

        metrics::increment_counter!("names_gender", &[("gender", gender.to_string())]);

        gender
    }
}
