use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};
use serde::Serialize;

use crate::{Gender, Name};

/// A dwarf’s name belongs to the clan, not to the individual. A dwarf who
/// misuses or brings shame to a clan name is stripped of the name and
/// forbidden by law to use any dwarven name in its place.
#[derive(Debug, Serialize)]
pub struct Dwarf {
    /// Granted by a clan elder, in accordance with tradition.
    pub first_name: &'static str,
    /// Clan the dwarf is a part of.
    pub clan_name: &'static str,
    /// Usual gender associated with the name.
    pub gender: Gender,
}

impl fmt::Display for Dwarf {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.clan_name)
    }
}

impl Distribution<Dwarf> for Standard {
    /// Generate a new dwarven name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dwarf {
        let gender: Gender = rng.gen();

        let first_name = *match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        }
        .choose(rng)
        .unwrap();

        let clan_name = *CLAN.choose(rng).unwrap();

        metrics::increment_counter!(
            "names_dwarf",
            &[("first_name", first_name), ("clan_name", clan_name),]
        );

        Dwarf {
            first_name,
            clan_name,
            gender,
        }
    }
}

impl Name for Dwarf {}

const CLAN: &[&str] = &[
    "Arnskull",
    "Balderk",
    "Battlehammer",
    "Blackbanner",
    "Blackhammer",
    "Brawnanvil",
    "Bucklebar",
    "Coalsmith",
    "Copperwraught",
    "Dankil",
    "Darkfell",
    "Deepaxe",
    "Deepdelve",
    "Deepforge",
    "Deephammer",
    "Dhargun",
    "Eaglecleft",
    "Eversharp",
    "Fireforge",
    "Flamestoker",
    "Foehammer",
    "Frostbeard",
    "Gallowglar",
    "Gorunn",
    "Grayshard",
    "Grimtongue",
    "Hammerthorn",
    "Hammerwhacker",
    "Hillsafar",
    "Holderhek",
    "Horn",
    "Ironbeard",
    "Ironfist",
    "Ironrune",
    "Ironshield",
    "Jundeth",
    "Kettlecopp",
    "Kwarter",
    "Loderr",
    "Lutgehr",
    "Narlagh",
    "Orothiar",
    "Quarrymaster",
    "Quirstiron",
    "Rockfist",
    "Rockseeker",
    "Rookoath",
    "Rumnaheim",
    "Rustfire",
    "Shattershield",
    "Skulldark",
    "Sstar",
    "Stoneshaft",
    "Stoneshield",
    "Stoneshoulder",
    "Strakeln",
    "Strongheart",
    "Talctuft",
    "Thunderwind",
    "Torunn",
    "Torwyn",
    "Trueforger",
    "Ungart",
    "Waranvil",
    "Warcrown",
    "Watchever",
    "Waybeard",
    "Worldthrone",
    "Wyrmslayer",
    "Yund",
];

const FEMALE: &[&str] = &[
    "Aela",
    "Amara",
    "Amber",
    "Artin",
    "Audhild",
    "Balifra",
    "Bardryn",
    "Dagdra",
    "Dagnabbet",
    "Dagnal",
    "Dazlyn",
    "Diesa",
    "Eldeth",
    "Falkrunn",
    "Finellen",
    "Gargosa",
    "Grista",
    "Grizzelda",
    "Gunnloda",
    "Gurdis",
    "Gustava",
    "Gwendolyn",
    "Helgret",
    "Helja",
    "Hlin",
    "Kathra",
    "Kira",
    "Kollette",
    "Kristryd",
    "Ilde",
    "Liftrasa",
    "Mardred",
    "Rala",
    "Riswynn",
    "Rizwin",
    "Ruby",
    "Sannl",
    "Sharna",
    "Tithmel",
    "Torbera",
    "Tordek",
    "Torgga",
    "Vistra",
    "Vozala",
];

const MALE: &[&str] = &[
    "Adrik",
    "Alberich",
    "Baern",
    "Barendd",
    "Bromm",
    "Brottor",
    "Bruenor",
    "Caerhan",
    "Connerad",
    "Dain",
    "Darrak",
    "Delg",
    "Drorn",
    "Ebenezer",
    "Eberk",
    "Einkil",
    "Emerus",
    "Fargrim",
    "Flint",
    "Gandalug",
    "Gardain",
    "Garumn",
    "Gorat",
    "Gori",
    "Gundren",
    "Harbek",
    "Harbromm",
    "Harnoth",
    "Ilgostrogue",
    "Jormun",
    "Kildrak",
    "Macklin",
    "Morgran",
    "Morinn",
    "Morkai",
    "Norbus",
    "Nundro",
    "Ollyn",
    "Orsik",
    "Oskar",
    "Rangrim",
    "Rurik",
    "Storn",
    "Taklinn",
    "Tannus",
    "Tenelar",
    "Thardin",
    "Thoman",
    "Thoradin",
    "Thorin",
    "Tordek",
    "Traubon",
    "Travok",
    "Ulaar",
    "Ulfgar",
    "Umbrag",
    "Veit",
    "Vondal",
    "Zardak",
];
