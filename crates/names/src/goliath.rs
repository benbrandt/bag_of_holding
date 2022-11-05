use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Every goliath has three names: a birth name assigned by the newborn’s
/// mother and father, a nickname assigned by the tribal chief, and a family or
/// clan name. A birth name is up to three syllables long. Clan names are five
/// syllables or more and end in a vowel.
///
/// Birth names are rarely linked to gender. Goliaths see females and males as
/// equal in all things, and they find societies with roles divided by gender
/// to be puzzling or worthy of mockery. To a goliath, the person who is best
/// at a job should be the one tasked with doing it.
///
/// A goliath’s nickname is a description that can change on the whim of a
/// chieftain or tribal elder. It refers to a notable deed, either a success or
/// failure, committed by the goliath. Goliaths assign and use nicknames with
/// their friends of other races, and change them to refer to an individual’s
/// notable deeds.
///
/// Goliaths present all three names when identifying themselves, in the order
/// of birth name, nickname, and clan name. In casual conversation, they use
/// their nickname.
#[derive(Debug)]
pub struct Goliath {
    /// Assigned by the newborn's mother and father
    birth_name: &'static str,
    /// Refers to a notable deed, either a success or failure
    nickname: &'static str,
    /// The clan the goliath is a part of
    clan_name: &'static str,
}

impl fmt::Display for Goliath {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} \"{}\" {}",
            self.birth_name, self.nickname, self.clan_name
        )
    }
}

impl Distribution<Goliath> for Standard {
    /// Generate a new Goliath name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Goliath {
        Goliath {
            birth_name: BIRTH_NAMES.choose(rng).unwrap(),
            nickname: NICKNAMES.choose(rng).unwrap(),
            clan_name: CLANS.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Goliath {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Goliath = rand_utils::rng_from_entropy().gen();
        assert!(!name.birth_name.is_empty());
        assert!(!name.nickname.is_empty());
        assert!(!name.clan_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!(
                "{} \"{}\" {}",
                name.birth_name, name.nickname, name.clan_name
            )
        );
    }
}

const BIRTH_NAMES: &[&str] = &[
    "Aukan", "Eglath", "Gae-El", "Gauthak", "Ilikan", "Keothi", "Kuori", "Lo-Kag", "Manneo",
    "Maveith", "Nalla", "Orilo", "Paavu", "Pethani", "Thalai", "Thotham", "Uthal", "Vaunea",
    "Vimak",
];

const CLANS: &[&str] = &[
    "Anakalathai",
    "Elanithino",
    "Gathakanathi",
    "Kalagiano",
    "Katho-Olavi",
    "Kolae-Gileana",
    "Ogolakanu",
    "Thuliaga",
    "Thunukalathi",
    "Vaimei-Laga",
];

const NICKNAMES: &[&str] = &[
    "Bearkiller",
    "Dawncaller",
    "Fearless",
    "Flintfinder",
    "Horncarver",
    "Keeneye",
    "Lonehunter",
    "Longleaper",
    "Rootsmasher",
    "Skywatcher",
    "Steadyhand",
    "Threadtwister",
    "Twice-Orphaned",
    "Twistedlimb",
    "Wordpainter",
];
