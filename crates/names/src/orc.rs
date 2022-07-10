use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Orc names don’t always have meaning in the Orc language, and most
/// noteworthy orcs are given epithets by their tribe mates.
#[derive(Debug)]
pub struct Orc {
    /// Given name
    name: &'static str,
    /// Additional name given for noteworthy deeds
    epithet: &'static str,
}

impl fmt::Display for Orc {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.epithet)
    }
}

impl Distribution<Orc> for Standard {
    /// Generate a new orc name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Orc {
        Orc {
            name: *[FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
            epithet: *EPITHET.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Orc {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Orc = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        assert!(!name.epithet.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{} {}", name.name, name.epithet));
    }
}

const EPITHET: &[&str] = &[
    "Axe-Biter",
    "Bone Crusher",
    "Death Spear",
    "Doom Hammer",
    "Elf Butcher",
    "Eye Gouger",
    "Flesh Ripper",
    "Iron Tusk",
    "Ironhead",
    "One-Tusk",
    "Red-Eye",
    "Skin Flayer",
    "Skull Cleaver",
    "Spine Snapper",
    "The Brutal",
    "The Filthy",
];

const FEMALE: &[&str] = &[
    "Baggi", "Breltora", "Cooragh", "Drethna", "Emen", "Engong", "Kansif", "Kra", "Myev", "Neega",
    "Ovak", "Ownka", "Shautha", "Sutha", "Vola", "Volen", "Yevelda",
];

const MALE: &[&str] = &[
    "Abzug", "Bajok", "Brughor", "Dench", "Feng", "Flenz", "Gell", "Grannoc", "Grutok", "Henk",
    "Holg", "Imsh", "Jahrukk", "Jolly", "Keth", "Krusk", "Lortar", "Mhurren", "Mobad", "Moesko",
    "Mugrub", "Narux", "Oshgir", "Rhorog", "Ront", "Ruhk", "Shamog", "Shugog", "Shump", "Thokk",
    "Urzul", "Yargath",
];
