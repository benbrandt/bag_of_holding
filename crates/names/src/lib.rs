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
    bugbear::Bugbear, dragonborn::Dragonborn, duergar::Duergar, dwarf::Dwarf, elf::Elf,
    githyanki::Githyanki, githzerai::Githzerai, gnome::Gnome, goblin::Goblin, goliath::Goliath,
    halfling::Halfling, hobgoblin::Hobgoblin, human::Human, kenku::Kenku, kobold::Kobold,
    lizardfolk::Lizardfolk, orc::Orc, tabaxi::Tabaxi, triton::Triton, yuan_ti::YuanTi,
};

mod bugbear;
mod dragonborn;
mod duergar;
mod dwarf;
mod elf;
mod githyanki;
mod githzerai;
mod gnome;
mod goblin;
mod goliath;
mod halfling;
mod hobgoblin;
mod human;
mod kenku;
mod kobold;
mod lizardfolk;
mod orc;
mod tabaxi;
mod triton;
mod yuan_ti;

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
    /// Names for Duergar characters
    Duergar,
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
    /// Names for Goliath characters
    Goliath,
    /// Names for Halfling characters
    Halfling,
    /// Names for Hobgoblin characters
    Hobgoblin,
    /// Names for Hobgoblin characters
    Human,
    /// Names for Kenku characters
    Kenku,
    /// Names for Kobold characters
    Kobold,
    /// Names for Lizardfolk characters
    Lizardfolk,
    /// Names for Orc characters
    Orc,
    /// Names for Tabaxi characters
    Tabaxi,
    /// Names for Triton characters
    Triton,
    /// Names for Yuan-ti characters
    YuanTi,
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
            Self::Duergar => rng.gen::<Duergar>().to_string(),
            Self::Dwarf => rng.gen::<Dwarf>().to_string(),
            Self::Elf => rng.gen::<Elf>().to_string(),
            Self::Githyanki => rng.gen::<Githyanki>().to_string(),
            Self::Githzerai => rng.gen::<Githzerai>().to_string(),
            Self::Gnome => rng.gen::<Gnome>().to_string(),
            Self::Goblin => rng.gen::<Goblin>().to_string(),
            Self::Goliath => rng.gen::<Goliath>().to_string(),
            Self::Halfling => rng.gen::<Halfling>().to_string(),
            Self::Hobgoblin => rng.gen::<Hobgoblin>().to_string(),
            Self::Human => rng.gen::<Human>().to_string(),
            Self::Kenku => rng.gen::<Kenku>().to_string(),
            Self::Kobold => rng.gen::<Kobold>().to_string(),
            Self::Lizardfolk => rng.gen::<Lizardfolk>().to_string(),
            Self::Orc => rng.gen::<Orc>().to_string(),
            Self::Tabaxi => rng.gen::<Tabaxi>().to_string(),
            Self::Triton => rng.gen::<Triton>().to_string(),
            Self::YuanTi => rng.gen::<YuanTi>().to_string(),
        }
    }
}
