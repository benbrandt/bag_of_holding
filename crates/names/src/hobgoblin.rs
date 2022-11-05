use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

// Hobgoblins only have a single name
#[derive(Debug)]
pub struct Hobgoblin {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Hobgoblin {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Hobgoblin> for Standard {
    /// Generate a new Hobgoblin name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hobgoblin {
        Hobgoblin {
            name: NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Hobgoblin {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Hobgoblin = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &["Fraht", "Garla", "Grunka", "Targor Bloodsword"];
