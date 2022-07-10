use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Given that kenku can duplicate any sound, their names are drawn from a
/// staggering variety of noises and phrases. Kenku names tend to break down
/// into three categories that make no distinction between male and female
/// names.
///
/// Kenku thugs, warriors, and toughs adopt noises made by weapons, such as the
/// clang of a mace against armor or the sound made by a breaking bone. Non-
/// kenku refer to the kenku by describing this noise. Examples of this type of
/// name include Smasher, Clanger, Slicer, and Basher.
///
/// Kenku thieves, con artists, and burglars adopt animal noises, typically
/// those common in urban settings. In this manner, kenku can call out to each
/// other while those who overhear them mistake them for common animals.
/// Non-kenku use names that refer to the sound made or the animal a kenku
/// mimics, such as Rat Scratch, Whistler, Mouser, and Growler.
///
/// Some kenku turn their back on crime to pursue legitimate trades. These
/// kenku adopt noises made as part of their craft. A sailor duplicates the
/// sound of a fluttering sail, while a smith mimics the clanging of a hammer
/// on metal. Non-kenku describe these folk by their trade sounds, such as Sail
/// Snap, Hammerer, and Cutter.
#[derive(Debug)]
pub struct Kenku {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for Kenku {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<Kenku> for Standard {
    /// Generate a new Bugbear name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Kenku {
        Kenku {
            name: *NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Kenku {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Kenku = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &[
    "Smasher",
    "Clanger",
    "Slicer",
    "Basher",
    "Rat Scratch",
    "Whistler",
    "Mouser",
    "Growler",
    "Sail Snap",
    "Hammerer",
    "Cutter",
];
