use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Each tabaxi has a single name, determined by clan and based on a complex
/// formula that involves astrology, prophecy, clan history, and other esoteric
/// factors. Tabaxi names can apply to both males and females, and most use
/// nicknames derived from or inspired by their full names.
///
/// Clan names are usually based on a geographical feature located in or near
/// the clan’s territory.
#[derive(Debug)]
pub struct Tabaxi {
    /// Determined by clan and based on a complex formula that involves
    /// astrology, prophecy, clan history, and other esoteric factors.
    name: &'static str,
    /// Clan names are usually based on a geographical feature located in or
    /// near the clan’s territory.
    clan_name: &'static str,
}

impl fmt::Display for Tabaxi {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.clan_name)
    }
}

impl Distribution<Tabaxi> for Standard {
    /// Generate a new tabaxi name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tabaxi {
        Tabaxi {
            name: *NAMES.choose(rng).unwrap(),
            clan_name: *CLANS.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Tabaxi {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Tabaxi = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        assert!(!name.clan_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!("{} {}", name.name, name.clan_name)
        );
    }
}

const CLANS: &[&str] = &[
    "Bright Cliffs",
    "Distant Rain",
    "Mountain Tree",
    "Rumbling River",
    "Snoring Mountain",
];

const NAMES: &[&str] = &[
    "Cloud on the Mountaintop (Cloud)",
    "Five Timber (Timber)",
    "Jade Shoe (Jade)",
    "Left Handed Hummingbird (Bird)",
    "Seven Thundercloud (Thunder)",
    "Skirt of Snakes (Snake)",
    "Smoking Mirror (Smoke)",
    "Stands in Tar (Tar)",
    "Two Dry Cloaks (Cloak)",
];
