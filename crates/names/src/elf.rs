use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Elves are considered children until they declare themselves adults, some
/// time after the hundredth birthday, and before this period they are called
/// by child names.
///
/// On declaring adulthood, an elf selects an adult name, although those who
/// knew him or her as a youngster might continue to use the child name. Each
/// elf’s adult name is a unique creation, though it might reflect the names of
/// respected individuals or other family members. Little distinction exists
/// between male names and female names; the groupings here reflect only
/// general tendencies. In addition, every elf bears a family name, typically a
/// combination of other Elvish words. Some elves traveling among humans
/// translate their family names into Common, but others retain the Elvish
/// version.
#[derive(Debug)]
pub struct Elf {
    /// Childhood name
    child_name: &'static str,
    /// Name chosen on declaration of adulthood
    pub(crate) adult_name: &'static str,
    /// Name of the family, typically a combination of other Elvish words
    pub(crate) family_name: &'static str,
}

impl fmt::Display for Elf {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} \"{}\" {}",
            self.adult_name, self.child_name, self.family_name
        )
    }
}

impl Distribution<Elf> for Standard {
    /// Generate a new Elf name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Elf {
        Elf {
            adult_name: *[FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
            child_name: *CHILD.choose(rng).unwrap(),
            family_name: *FAMILY.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Elf {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Elf = rand_utils::rng_from_entropy().gen();
        assert!(!name.adult_name.is_empty());
        assert!(!name.child_name.is_empty());
        assert!(!name.family_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!(
                "{} \"{}\" {}",
                name.adult_name, name.child_name, name.family_name
            )
        );
    }
}

const CHILD: &[&str] = &[
    "Ara", "Bryn", "Del", "Eryn", "Faen", "Innil", "Lael", "Mella", "Naill", "Naeris", "Phann",
    "Rael", "Rinn", "Sai", "Syllin", "Thia", "Vall",
];

const FAMILY: &[&str] = &[
    "Amakiir (Gemflower)",
    "Amastacia (Starflower)",
    "Edermath",
    "Floshin",
    "Galanodel (Moonwhisper)",
    "Graymantle",
    "Haevault",
    "Holimion (Diamonddew)",
    "Ilphelkiir (Gemblossom)",
    "Liadon (Silverfrond)",
    "Liethennson",
    "Meliamne (Oakenheel)",
    "Moonflower",
    "Na\u{ef}lo (Nightbreeze)",
    "Siannodel (Moonbrook)",
    "Silverhand",
    "Torval",
    "Truesilver",
    "Truff",
    "Xiloscient (Goldpetal)",
];

const FEMALE: &[&str] = &[
    "Adrie",
    "Aedyn",
    "Althaea",
    "Alustrial",
    "Amlaruil",
    "Anastrianna",
    "Andraste",
    "Antinua",
    "Bethrynna",
    "Birel",
    "Caelynn",
    "Drusilia",
    "Enna",
    "Felosial",
    "Garaele",
    "Ielenia",
    "Iverna",
    "Jelenneth",
    "Keyleth",
    "Leshanna",
    "Lia",
    "Mergen",
    "Meriele",
    "Mialee",
    "Miraal",
    "Morgwais",
    "Naivara",
    "Ordalf",
    "Quelenna",
    "Quillathe",
    "Sariel",
    "Shanairra",
    "Shava",
    "Silaqui",
    "Theirastra",
    "Thia",
    "Vadania",
    "Valanthe",
    "Vyldara",
    "Xanaphia",
];

const MALE: &[&str] = &[
    "Adran",
    "Aelar",
    "Araithe",
    "Aramil",
    "Arannis",
    "Aust",
    "Beiro",
    "Bendari",
    "Berrian",
    "Carric",
    "Cymbiir",
    "Daran",
    "Darfin",
    "Ecamane",
    "Enialis",
    "Erdan",
    "Erevan",
    "Galinndan",
    "Hadarai",
    "Heian",
    "Himo",
    "Immeral",
    "Ivellios",
    "Laucian",
    "Mindartis",
    "Nezznar",
    "Orvis",
    "Paelias",
    "Peren",
    "Quarion",
    "Riardon",
    "Rolen",
    "Soveliss",
    "Spivey",
    "Thamior",
    "Tharivol",
    "Theren",
    "Varis",
];
