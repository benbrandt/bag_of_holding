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

use dwarf::Dwarf;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

mod dwarf;

/// Implements the ability to generate a name for a given race.
/// Can contain whatever information is necessary for a given name
/// (such as gender, ethnicity, child names, etc)
///
/// Display impl should format the name in a format suitable for a character
/// sheet.
pub trait NameGenerator: fmt::Display + Sized
where
    Standard: Distribution<Self>,
{
}

/// Available race options to choose names from
#[derive(Debug, Deserialize, Display, EnumIter, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Name {
    /// Names for dwarven characters
    Dwarf,
}

impl Name {
    /// Generate a new name for the given race
    ///
    /// ```
    /// use names::Name;
    /// use rand::Rng;
    ///
    /// let name = Name::Dwarf.gen(&mut rand::thread_rng());
    /// ```
    pub fn gen(&self, rng: &mut impl Rng) -> impl NameGenerator {
        metrics::increment_counter!("names", &[("generator", self.to_string())]);

        match self {
            Self::Dwarf => rng.gen::<Dwarf>(),
        }
    }
}
