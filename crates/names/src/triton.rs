use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Most triton names have two or three syllables. Male names typically end
/// with a vowel and the letter s, and female names traditionally end with an n.
/// Tritons use their home protectorate as a surname, with the name formed by
/// adding a vowel followed by a “th” to the end of the protectorate’s name.
#[derive(Debug)]
pub struct Triton {
    /// Given name
    first_name: &'static str,
    /// Home protectorate
    surname: &'static str,
}

impl fmt::Display for Triton {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.first_name, self.surname)
    }
}

impl Distribution<Triton> for Standard {
    /// Generate a new triton name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triton {
        Triton {
            first_name: *[FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
            surname: *SURNAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Triton {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Triton = rand_utils::rng_from_entropy().gen();
        assert!(!name.first_name.is_empty());
        assert!(!name.surname.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!("{} {}", name.first_name, name.surname)
        );
    }
}

const FEMALE: &[&str] = &[
    "Aryn", "Belthyn", "Duthyn", "Feloren", "Otanyn", "Shalryn", "Vlaryn", "Wolyn",
];

const MALE: &[&str] = &[
    "Corus", "Delnis", "Jhimas", "Keros", "Molos", "Nalos", "Vodos", "Zunis",
];

const SURNAMES: &[&str] = &["Ahlorsath", "Pumanath", "Vuuvaxath"];
