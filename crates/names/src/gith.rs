use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

#[derive(Debug)]
/// Githyanki only have a single name
pub struct Githyanki {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Githyanki {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Githyanki> for Standard {
    /// Generate a new Githyanki name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Githyanki {
        Githyanki {
            name: *[GITHYANKI_FEMALE, GITHYANKI_MALE]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

impl NameGenerator for Githyanki {}

#[derive(Debug)]
/// Githzerai only have a single name
pub struct Githzerai {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Githzerai {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Githzerai> for Standard {
    /// Generate a new Githzerai name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Githzerai {
        Githzerai {
            name: *[GITHZERAI_FEMALE, GITHZERAI_MALE]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
        }
    }
}

impl NameGenerator for Githzerai {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn githyanki() {
        let name: Githyanki = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }

    #[test]
    fn githzerai() {
        let name: Githzerai = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

pub const GITHYANKI_FEMALE: &[&str] = &[
    "Aaryl",
    "B'noor",
    "Fenelzi'ir",
    "Jen'lig",
    "Pah'zel",
    "Quorstyl",
    "Sirruth",
    "Vaira",
    "Yessune",
    "Zar'ryth",
];

pub const GITHYANKI_MALE: &[&str] = &[
    "Elirdain",
    "Gaath",
    "Ja'adoc",
    "Kar'i'nas",
    "Lykus",
    "Meldavh",
    "Quith",
    "Ris'a'an",
    "Tropos",
    "Viran",
    "Xamodas",
];

pub const GITHZERAI_FEMALE: &[&str] = &[
    "Adaka", "Izera", "Adeya", "Janara", "Ella", "Loraya", "Ezhelya", "Uweya", "Immilzin", "Vithka",
];
pub const GITHZERAI_MALE: &[&str] = &[
    "Dak", "Kalla", "Duurth", "Muurg", "Ferzth", "Nurm", "Greth", "Shrakk", "Hurm", "Xorm",
];
