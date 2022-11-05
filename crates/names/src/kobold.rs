use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Kobold names are derived from the Draconic tongue and usually relate to a
/// characteristic of the owner, such as scale color, distinctive body parts,
/// or typical behavior. For example, “Red Foot,” “White Claw,” and “Scurry”
/// are Common translations of often-used names. A kobold might change its name
/// when it becomes an adult, or add additional word-syllables after important
/// events such as completing its first hunt, laying its first egg, or
/// surviving its first battle.
#[derive(Debug)]
pub struct Kobold {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Kobold {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Kobold> for Standard {
    /// Generate a new Kobold name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Kobold {
        Kobold {
            name: NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Kobold {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Kobold = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &[
    "Arix", "Eks", "Ett", "Galax", "Garu", "Hagnar", "Hox", "Irtos", "Kashak", "Meepo", "Molo",
    "Ohsoss", "Patsky", "Rotom", "Sagim", "Sik", "Sniv", "Taklak", "Tes", "Urak", "Varn",
];
