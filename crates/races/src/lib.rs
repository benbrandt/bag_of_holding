//! # Races
//!
//! Crate to generate races for character players.
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

use enum_dispatch::enum_dispatch;
use names::Name;
use rand::{
    distributions::Standard,
    prelude::{Distribution, IteratorRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use sources::{Book, Sources};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::dragonborn::Dragonborn;

mod dragonborn;

/// Implements the ability to generate a race option, with all the necessary
/// decisions made for features of that race.
#[enum_dispatch]
pub trait RaceGenerator: Clone + fmt::Display + Sized + Sources
where
    Standard: Distribution<Self>,
{
    /// Name generator to use for this race
    fn name_generator(&self) -> Name;
    /// Generate a name for this race
    fn gen_name<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        self.name_generator().gen(rng)
    }
}

/// Supported races to choose from
#[derive(Debug, Deserialize, Display, EnumIter, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum RaceOption {
    /// Born of dragons, as their name proclaims, the dragonborn walk proudly
    /// through a world that greets them with fearful incomprehension. Shaped
    /// by draconic gods or the dragons themselves, dragonborn originally
    /// hatched from dragon eggs as a unique race, combining the best
    /// attributes of dragons and humanoids. Some dragonborn are faithful
    /// servants to true dragons, others form the ranks of soldiers in great
    /// wars, and still others find themselves adrift, with no clear calling
    /// in life.
    Dragonborn,
}

impl RaceOption {
    /// Given a specified race option, generate a random race
    #[tracing::instrument(skip(rng))]
    pub fn gen<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        metrics::increment_counter!("races", &[("generator", self.to_string())]);

        match self {
            Self::Dragonborn => rng.gen::<Dragonborn>().into(),
        }
    }
}

/// Available race options to generate races from
#[enum_dispatch(RaceGenerator)]
#[derive(Clone, Debug, Serialize)]
#[serde(into = "RaceSerializer")]
pub enum Race {
    /// Born of dragons, as their name proclaims, the dragonborn walk proudly
    /// through a world that greets them with fearful incomprehension. Shaped
    /// by draconic gods or the dragons themselves, dragonborn originally
    /// hatched from dragon eggs as a unique race, combining the best
    /// attributes of dragons and humanoids. Some dragonborn are faithful
    /// servants to true dragons, others form the ranks of soldiers in great
    /// wars, and still others find themselves adrift, with no clear calling
    /// in life.
    Dragonborn(Dragonborn),
}

impl Sources for Race {
    fn sources(&self) -> &[Book] {
        match self {
            Self::Dragonborn(d) => d.sources(),
        }
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dragonborn(r) => write!(f, "{r}"),
        }
    }
}

impl Distribution<Race> for Standard {
    /// Generate a random race
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        RaceOption::iter().choose(rng).unwrap().gen(rng)
    }
}

/// Serializable public interface of a given race
#[derive(Serialize)]
struct RaceSerializer {
    /// Human-readable string version of the race
    race: String,
    /// Sources for the generated race
    sources: Vec<Book>,
}

impl From<Race> for RaceSerializer {
    fn from(race: Race) -> Self {
        Self {
            race: race.to_string(),
            sources: race.sources().to_vec(),
        }
    }
}
