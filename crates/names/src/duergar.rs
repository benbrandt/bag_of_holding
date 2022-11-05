use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::{dwarf::Dwarf, NameGenerator};

/// Derivation of normal dwarven names, with different clans
#[derive(Debug)]
pub struct Duergar {
    /// Granted by a clan elder, in accordance with tradition.
    first_name: &'static str,
    /// Clan the dwarf is a part of.
    clan_name: &'static str,
}

impl fmt::Display for Duergar {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.clan_name)
    }
}

impl Distribution<Duergar> for Standard {
    /// Generate a new dwarven name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duergar {
        Duergar {
            first_name: rng.gen::<Dwarf>().first_name,
            clan_name: CLAN.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Duergar {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Duergar = rand_utils::rng_from_entropy().gen();
        assert!(!name.first_name.is_empty());
        assert!(!name.clan_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!("{} {}", name.first_name, name.clan_name)
        );
    }
}

const CLAN: &[&str] = &[
    "Ashlord",
    "Battlegore",
    "Doomfist",
    "Earthlord",
    "Firetamer",
    "Ironmind",
    "Knifemind",
    "Mindeater",
    "Necksnapper",
    "Orehammer",
    "Runehammer",
    "Spikefist",
    "Thundermaster",
    "Underearth",
];
