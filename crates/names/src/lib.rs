//! # Names
//!
//! Generate names for any race in the D&D multiverse.
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

use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, IteratorRandom},
    Rng,
};
use serde::Serialize;
use strum::{Display, EnumIter, IntoEnumIterator};

mod dwarf;

pub use dwarf::Dwarf;

/// Implements the ability to generate a name for a given race.
/// Can contain whatever information is necessary for a given name
/// (such as gender, ethnicity, child names, etc)
///
/// Display impl should format the name in a format suitable for a character
/// sheet.
pub trait Name: fmt::Display + Serialize + Sized
where
    Standard: Distribution<Self>,
{
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

impl Distribution<Gender> for Standard {
    /// Choose a random gender for choosing between name lists.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        let gender = Gender::iter().choose(rng).unwrap();

        metrics::increment_counter!("names_gender", &[("gender", gender.to_string())]);

        gender
    }
}
