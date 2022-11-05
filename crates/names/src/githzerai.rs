use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Githzerai only have a single name
#[derive(Debug)]
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
            name: [FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Githzerai {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Githzerai = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const FEMALE: &[&str] = &[
    "Adaka", "Izera", "Adeya", "Janara", "Ella", "Loraya", "Ezhelya", "Uweya", "Immilzin", "Vithka",
];

const MALE: &[&str] = &[
    "Dak", "Kalla", "Duurth", "Muurg", "Ferzth", "Nurm", "Greth", "Shrakk", "Hurm", "Xorm",
];
