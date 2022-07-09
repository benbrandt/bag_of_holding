use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// A halfling has a given name, a family name, and possibly a nickname. Family
/// names are often nicknames that stuck so tenaciously they have been passed
/// down through the generations.
#[derive(Debug)]
pub struct Halfling {
    /// First name, given by family.
    given_name: &'static str,
    /// Nicknames that stuck so tenaciously they have been passed down
    family_name: &'static str,
}

impl fmt::Display for Halfling {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.given_name, self.family_name)
    }
}

impl Distribution<Halfling> for Standard {
    /// Generate a new halfling name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Halfling {
        Halfling {
            given_name: *[FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
            family_name: *FAMILY.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Halfling {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Halfling = rand_utils::rng_from_entropy().gen();
        assert!(!name.given_name.is_empty());
        assert!(!name.family_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!("{} {}", name.given_name, name.family_name)
        );
    }
}

const FAMILY: &[&str] = &[
    "Adalgrim",
    "Addlespur",
    "Alderleaf",
    "Bosh",
    "Brightshine",
    "Brushgather",
    "Fishskipper",
    "Goodbarrel",
    "Greenbottle",
    "High-hill",
    "Hightopple",
    "Hilltopple",
    "Leagallow",
    "McRoyne",
    "Scalesweep",
    "Tealeaf",
    "Thorngage",
    "Tosscobble",
    "Underbough",
    "Wanderfoot",
];

const FEMALE: &[&str] = &[
    "Amarandine",
    "Andry",
    "Bree",
    "Callie",
    "Cora",
    "Euphemia",
    "Jillian",
    "Kithri",
    "Lavinia",
    "Lidda",
    "Merla",
    "Merrygold",
    "Molley",
    "Nedda",
    "Nib",
    "Qelline",
    "Paela",
    "Portia",
    "Seraphina",
    "Shaena",
    "Silla",
    "Streams",
    "Trym",
    "Vani",
    "Verna",
];

const MALE: &[&str] = &[
    "Alton", "Ander", "Cade", "Carp", "Corrin", "Eldon", "Errich", "Finnan", "Garret", "Harkin",
    "Lindal", "Lyle", "Marks", "Merric", "Milo", "Osborn", "Otis", "Patsy", "Perrin", "Pieter",
    "Quinn", "Reed", "Rosco", "Roscoe", "Wellby",
];
