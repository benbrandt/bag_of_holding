use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Dragonborn have personal names given at birth, but they put their clan
/// names first as a mark of honor. A childhood name or nickname is often used
/// among clutchmates as a descriptive term or a term of endearment. The name
/// might recall an event or center on a habit.
#[derive(Debug)]
pub struct Dragonborn {
    /// Personal name given at birth
    first_name: &'static str,
    /// Childhood name or nickname
    child_name: &'static str,
    /// Name of their clan, mark of honor
    clan_name: &'static str,
}

impl fmt::Display for Dragonborn {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} \"{}\" {}",
            self.first_name, self.child_name, self.clan_name
        )
    }
}

impl Distribution<Dragonborn> for Standard {
    /// Generate a new dragonborn name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dragonborn {
        Dragonborn {
            first_name: [FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
            child_name: CHILD.choose(rng).unwrap(),
            clan_name: CLAN.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Dragonborn {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Dragonborn = rand_utils::rng_from_entropy().gen();
        assert!(!name.first_name.is_empty());
        assert!(!name.child_name.is_empty());
        assert!(!name.clan_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!(
                "{} \"{}\" {}",
                name.first_name, name.child_name, name.clan_name
            )
        );
    }
}

const CHILD: &[&str] = &[
    "Climber",
    "Earbender",
    "Leaper",
    "Pious",
    "Shieldbiter",
    "Zealous",
];

const CLAN: &[&str] = &[
    "Blaakberz",
    "Clethtinthiallor",
    "Daardendrian",
    "Delmirev",
    "Drachedandion",
    "Fenkenkabradon",
    "Kepeshkmolik",
    "Kerrhylon",
    "Kimbatuul",
    "Linxakasendalor",
    "Myastan",
    "Nemmonis",
    "Norixius",
    "Ophinshtalajiir",
    "Prexijandilin",
    "Shestendeliath",
    "Turnuroth",
    "Verthisathurgiesh",
    "Yarjerit",
];

const FEMALE: &[&str] = &[
    "Akra", "Biri", "Daar", "Farideh", "Harann", "Havilar", "Jheri", "Kava", "Korinn", "Mishann",
    "Nala", "Perra", "Raiann", "Sora", "Surina", "Thava", "Uadjit",
];

const MALE: &[&str] = &[
    "Arjhan",
    "Balasar",
    "Bharash",
    "Donaar",
    "Drahkso",
    "Ghesh",
    "Heskan",
    "Kristoffen",
    "Kriv",
    "Medrash",
    "Mehen",
    "Nadarr",
    "Pandjed",
    "Patrin",
    "Rhogar",
    "Shamash",
    "Shedinn",
    "Tarhun",
    "Torinn",
];
