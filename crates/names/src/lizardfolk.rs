use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Lizardfolk take their names from the Draconic language. They use simple
/// descriptives granted by the tribe based on an individual’s notable deeds or
/// actions. For example, Garurt translates as “axe,” a name given to a
/// lizardfolk warrior who defeated an orc and claimed his foe’s weapon. A
/// lizardfolk who likes to hide in a stand of reeds before ambushing an animal
/// might be called Achuak, which means “green” to describe how she blends into
/// the foliage.
///
/// Lizardfolk make no distinction between male and female in their naming
/// conventions.
#[derive(Debug)]
pub struct Lizardfolk {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Lizardfolk {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Lizardfolk> for Standard {
    /// Generate a new Lizardfolk name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lizardfolk {
        Lizardfolk {
            name: *NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Lizardfolk {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Lizardfolk = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &[
    "Achuak (green)",
    "Aryte (war)",
    "Baeshra (animal)",
    "Bracus",
    "Darastrix (dragon)",
    "Garurt (axe)",
    "Hissain",
    "Irhtos (secret)",
    "Jhank (hammer)",
    "Kepesk (storm)",
    "Kethend (gem)",
    "Korth (danger)",
    "Kosj (small)",
    "Kothar (demon)",
    "Litrix (armor)",
    "Mirik (song)",
    "Othokent (smart)",
    "Sauriv (eye)",
    "Slosh",
    "Thetsis",
    "Throden (many)",
    "Thurkear (night)",
    "Usk (iron)",
    "Valignat (burn)",
    "Vargach (battle)",
    "Verthica (mountain)",
    "Vutha (black)",
    "Vyth (steel)",
];
