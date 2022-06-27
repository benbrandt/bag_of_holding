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

use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::{
    bugbear::Bugbear, dragonborn::Dragonborn, dwarf::Dwarf, elf::Elf, githyanki::Githyanki,
    githzerai::Githzerai, gnome::Gnome, goblin::Goblin, hobgoblin::Hobgoblin,
};

mod bugbear;
mod dragonborn;
mod dwarf;
mod elf;
mod githyanki;
mod githzerai;
mod gnome;
mod goblin;
mod hobgoblin;

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
    /// Names for Bugbear characters
    Bugbear,
    /// Names for Dragonborn characters
    Dragonborn,
    /// Names for Dwarven characters
    Dwarf,
    /// Names for characters of races that use Elven names
    Elf,
    /// Names for Githzyanki characters
    Githyanki,
    /// Names for Githzerai characters
    Githzerai,
    /// Names for Gnome characters
    Gnome,
    /// Names for Goblin characters
    Goblin,
    /// Names for Hobgoblin characters
    Hobgoblin,
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
    #[tracing::instrument(skip(rng))]
    pub fn gen(&self, rng: &mut impl Rng) -> String {
        metrics::increment_counter!("names", &[("generator", self.to_string())]);

        match self {
            Self::Bugbear => rng.gen::<Bugbear>().to_string(),
            Self::Dragonborn => rng.gen::<Dragonborn>().to_string(),
            Self::Dwarf => rng.gen::<Dwarf>().to_string(),
            Self::Elf => rng.gen::<Elf>().to_string(),
            Self::Githyanki => rng.gen::<Githyanki>().to_string(),
            Self::Githzerai => rng.gen::<Githzerai>().to_string(),
            Self::Gnome => rng.gen::<Gnome>().to_string(),
            Self::Goblin => rng.gen::<Goblin>().to_string(),
            Self::Hobgoblin => rng.gen::<Hobgoblin>().to_string(),
        }
    }
}
