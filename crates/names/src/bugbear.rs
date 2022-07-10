use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Bugbears only have a single name
#[derive(Debug)]
pub struct Bugbear {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Bugbear {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Bugbear> for Standard {
    /// Generate a new Bugbear name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bugbear {
        Bugbear {
            name: *NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Bugbear {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Bugbear = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &["Grol", "Jutt", "Klarg", "Meff", "Mosk"];
