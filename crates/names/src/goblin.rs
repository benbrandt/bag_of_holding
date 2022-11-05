use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Goblins only have a single name
#[derive(Debug)]
pub struct Goblin {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Goblin {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Goblin> for Standard {
    /// Generate a new Goblin name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Goblin {
        Goblin {
            name: NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Goblin {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Goblin = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &["Droop", "Gorkoh", "Lhupo", "Splug", "Yeemik", "Yegg"];
