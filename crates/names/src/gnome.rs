use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Gnomes love names, and most have half a dozen or so. A gnome’s mother,
/// father, clan elder, aunts, and uncles each give the gnome a name, and
/// various nicknames from just about everyone else might or might not stick
/// over time. Gnome names are typically variants on the names of ancestors or
/// distant relatives, though some are purely new inventions. When dealing
/// with humans and others who are “stuffy” about names, a gnome learns to use
/// no more than three names: a personal name, a clan name, and a nickname,
/// choosing the one in each category that’s the most fun to say.
#[derive(Debug)]
pub struct Gnome {
    /// Similar to a "first name"
    personal_name: &'static str,
    /// A nickname that has stuck over time
    nickname: &'static str,
    /// The clan the gnome is a part of
    clan_name: &'static str,
}

impl fmt::Display for Gnome {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} \"{}\" {}",
            self.personal_name, self.nickname, self.clan_name
        )
    }
}

impl Distribution<Gnome> for Standard {
    /// Generate a new Elf name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gnome {
        Gnome {
            personal_name: [FEMALE, MALE].choose(rng).unwrap().choose(rng).unwrap(),
            nickname: NICKNAMES.choose(rng).unwrap(),
            clan_name: CLAN.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Gnome {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: Gnome = rand_utils::rng_from_entropy().gen();
        assert!(!name.personal_name.is_empty());
        assert!(!name.nickname.is_empty());
        assert!(!name.clan_name.is_empty());
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!(
                "{} \"{}\" {}",
                name.personal_name, name.nickname, name.clan_name
            )
        );
    }
}
const CLAN: &[&str] = &[
    "Beren",
    "Daergel",
    "Folkor",
    "Fotz",
    "Fundi",
    "Garrick",
    "Gran'Shoop",
    "Juntberry",
    "Nackle",
    "Murnig",
    "Ningel",
    "Onderquill",
    "Raulnor",
    "Scheppen",
    "Timbers",
    "Turen",
    "Wigglehoof",
];

const FEMALE: &[&str] = &[
    "Bimpnottin",
    "Breena",
    "Caramip",
    "Carlin",
    "Cray",
    "Dabbledob",
    "Ditch",
    "Donella",
    "Duvamil",
    "Ella",
    "Ellyjobell",
    "Ellywick",
    "Erris",
    "Facktor\u{e9}",
    "Jabby",
    "Joybell",
    "Lilli",
    "Loopmottin",
    "Lorilla",
    "Mardnab",
    "Nissa",
    "Nyx",
    "Oda",
    "Orla",
    "Panana",
    "Pinchwit",
    "Quippy",
    "Roywyn",
    "Shamil",
    "Tana",
    "Tervaround",
    "Ulla",
    "Waywocket",
    "Zanna",
];

const MALE: &[&str] = &[
    "Alston",
    "Alvyn",
    "Anverth",
    "Boddynock",
    "Brocc",
    "Burgell",
    "Delebean",
    "Dimble",
    "Eldon",
    "Erky",
    "Fibblestib",
    "Fonkin",
    "Frug",
    "Gerbo",
    "Gimble",
    "Glim",
    "Gnerkli",
    "Griballix",
    "Hoobur",
    "Jebeddo",
    "Jerronimous",
    "Kellen",
    "Korboz",
    "Namfoodle",
    "Orryn",
    "Pallabar",
    "Pog",
    "Roondar",
    "Seebo",
    "Sindri",
    "Uppendown",
    "Warryn",
    "Wrenn",
    "Zook",
];

const NICKNAMES: &[&str] = &[
    "Aleslosh",
    "Ashhearth",
    "Badger",
    "Cloak",
    "Doublelock",
    "Filchbatter",
    "Fnipper",
    "Jerr",
    "Ku",
    "Nim",
    "Oneshoe",
    "Pock",
    "Sparklegem",
    "Stumbleduck",
    "Wizzy",
];
