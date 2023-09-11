use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::{human::Human, NameGenerator};

/// Tiefling names fall into three broad categories. Tieflings born into
/// another culture typically have names reflective of that culture. Some have
/// names derived from the Infernal language, passed down through generations,
/// that reflect their fiendish heritage. And some younger tieflings, striving
/// to find a place in the world, adopt a name that signifies a virtue or other
/// concept and then try to embody that concept. For some, the chosen name is a
/// noble quest. For others, it’s a grim destiny.
#[derive(Debug)]
pub struct Tiefling {
    /// Cultural, abyssal, or virtue name
    first_name: &'static str,
    /// Last name
    surname: Option<&'static str>,
}

impl fmt::Display for Tiefling {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.first_name)?;
        if let Some(surname) = self.surname {
            write!(f, " {surname}")?;
        }
        Ok(())
    }
}

impl Distribution<Tiefling> for Standard {
    /// Generate a new tiefling name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tiefling {
        let human = rng.gen::<Human>();

        let first_name = *[
            [MALE_ABYSSAL, FEMALE_ABYSSAL].choose(rng).unwrap(),
            VIRTUE_NAMES,
            &[human.first_name],
        ]
        .choose(rng)
        .unwrap()
        .choose(rng)
        .unwrap();

        let surname = *[SURNAMES.choose(rng).copied(), human.surname]
            .choose(rng)
            .unwrap();

        Tiefling {
            first_name,
            surname,
        }
    }
}

impl NameGenerator for Tiefling {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Tiefling = rand_utils::rng_from_entropy().gen();
        assert!(!name.first_name.is_empty());
        if let Some(surname) = name.surname {
            assert!(!surname.is_empty());
        }
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!("{} {}", name.first_name, name.surname.unwrap_or_default()).trim()
        );
    }
}

const FEMALE_ABYSSAL: &[&str] = &[
    "Akta",
    "Anakis",
    "Bryseis",
    "Criella",
    "Damaia",
    "Ea",
    "Hania",
    "Incindorita",
    "Kallista",
    "Lerissa",
    "Makaria",
    "Nemeia",
    "Nixoxious",
    "Orianna",
    "Phelaia",
    "Rashaa",
    "Rieta",
    "Talanatha",
    "Zaar",
];

const MALE_ABYSSAL: &[&str] = &[
    "Akmenos", "Amnon", "Barakas", "Damakos", "Ekemon", "Haroun", "Iados", "Ishaq", "Kairon",
    "Leucis", "Melech", "Mordai", "Morthos", "Nizam", "Pelaios", "Skamos", "Therai",
];

const VIRTUE_NAMES: &[&str] = &[
    "Art",
    "Aybtep (\"horned\")",
    "Bahati (\"wise soul\")",
    "Carrion",
    "Chant",
    "Creed",
    "Despair",
    "Devil Dog",
    "Excellence",
    "Fear",
    "Glory",
    "Het (\"smoke\")",
    "Hope",
    "Ideal",
    "Kamen (\"dark\")",
    "Katsu (\"star born\")",
    "Kohl (\"dark eyed\")",
    "Music",
    "Nowhere",
    "Open",
    "Poetry",
    "Quest",
    "Random",
    "Reverence",
    "Sorrow",
    "Temerity",
    "Torment",
    "Weary",
];

const SURNAMES: &[&str] = &["Al-Khem", "Beni-Asmodai", "Zianhur", "Siasobek"];
