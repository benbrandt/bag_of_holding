use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, IteratorRandom, SliceRandom},
    Rng,
};
use strum::{EnumIter, IntoEnumIterator};

use crate::NameGenerator;

/// Having so much more variety than other cultures, humans as a whole have no
/// typical names. Some human parents give their children names from other
/// languages, such as Dwarvish or Elvish (pronounced more or less correctly),
/// but most parents give names that are linked to their region’s culture or to
/// the naming traditions of their ancestors.
#[derive(Debug)]
pub struct Human {
    /// Given name
    pub(crate) first_name: &'static str,
    /// Family name
    pub(crate) surname: Option<&'static str>,
}

impl fmt::Display for Human {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.first_name)?;
        if let Some(surname) = self.surname {
            write!(f, " {surname}")?;
        }
        Ok(())
    }
}

impl Distribution<Human> for Standard {
    /// Generate a new human name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Human {
        // 1/10 chance of having name come from multiple ethnicities
        let amount = [(1, 9), (2, 1)].choose_weighted(rng, |i| i.1).unwrap().0;
        let names = Ethnicity::iter()
            .choose_multiple(rng, amount)
            .into_iter()
            .map(|e| e.name(rng))
            .collect::<Vec<_>>();

        // Choose between the generated options
        Human {
            first_name: names.iter().map(|n| n.first_name).choose(rng).unwrap(),
            surname: names.iter().map(|n| n.surname).choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for Human {}

/// Ethnicity options for humans, which determines which name lists we choose from
#[derive(Debug, EnumIter)]
enum Ethnicity {
    Arkaiun,
    Barovian,
    Bedine,
    Calishite,
    Chondathan,
    Damaran,
    Ffolk,
    Gur,
    Halruaan,
    Illuskan,
    Imaskari,
    Mulan,
    Nar,
    Rashemi,
    Shaaran,
    Shou,
    Tethyrian,
    Tuigan,
    Turami,
    Ulutiun,
}

impl Ethnicity {
    /// Returns lists of names for the given ethnicity
    fn names(self) -> EthnicityNames {
        match self {
            Self::Arkaiun => ARKAIUN,
            Self::Barovian => BAROVIAN,
            Self::Bedine => BEDINE,
            Self::Calishite => CALISHITE,
            Self::Chondathan | Self::Tethyrian => CHONDATHAN,
            Self::Damaran => DAMARAN,
            Self::Ffolk => FFOLK,
            Self::Gur => GUR,
            Self::Halruaan => HALRUAAN,
            Self::Illuskan => ILLUSKAN,
            Self::Imaskari => IMASKARI,
            Self::Mulan => MULAN,
            Self::Nar => NAR,
            Self::Rashemi => RASHEMI,
            Self::Shaaran => SHAARAN,
            Self::Shou => SHOU,
            Self::Tuigan => TUIGAN,
            Self::Turami => TURAMI,
            Self::Ulutiun => ULUTIUN,
        }
    }

    /// Generates a human name for the given ethnicity
    fn name<R: Rng + ?Sized>(self, rng: &mut R) -> Human {
        let names = self.names();
        Human {
            first_name: [names.female, names.male]
                .choose(rng)
                .unwrap()
                .choose(rng)
                .unwrap(),
            surname: names.surname.choose(rng).copied(),
        }
    }
}

impl Distribution<Ethnicity> for Standard {
    /// Generate a new dwarven name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ethnicity {
        Ethnicity::iter().choose(rng).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ethnicities() {
        let mut rng = rand_utils::rng_from_entropy();
        for ethnicity in Ethnicity::iter() {
            let name = ethnicity.name(&mut rng);
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

    #[test]
    fn name() {
        let name: Human = rand_utils::rng_from_entropy().gen();
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

/// Name options for a given ethnicity
struct EthnicityNames {
    female: &'static [&'static str],
    male: &'static [&'static str],
    surname: &'static [&'static str],
}

const ARKAIUN: EthnicityNames = EthnicityNames {
    female: &["Glouris", "Maeve", "Sevaera", "Xaemarra", "Zraela"],
    male: &["Houn", "Rhivaun", "Umbril", "Xaemar", "Zeltaebar"],
    surname: &["Lharaendo", "Mristar", "Wyndael"],
};

const BAROVIAN: EthnicityNames = EthnicityNames {
    female: &[
        "Alana",
        "Alenka",
        "Amalthia",
        "Anastrasya",
        "Anna",
        "Arabelle",
        "Artista",
        "Aziana",
        "Bianca",
        "Clavdia",
        "Danya",
        "Davanka",
        "Dezdrelda",
        "Diavola",
        "Dimira",
        "Dorfniya",
        "Dorina",
        "Drasha",
        "Drilvia",
        "Drusilla",
        "Elisabeta",
        "Elisabeth",
        "Elsa",
        "Eva",
        "Ezmerelda",
        "Fatima",
        "Fiona",
        "Gertruda",
        "Grilsha",
        "Helga",
        "Helwa",
        "Ireena",
        "Isabella",
        "Isolde",
        "Ivana",
        "Jarzinka",
        "Kala",
        "Katerina",
        "Kereza",
        "Korina",
        "Kretyana",
        "Lavinia",
        "Lovina",
        "Ludmilla",
        "Lydia",
        "Madalena",
        "Magda",
        "Marina",
        "Marta",
        "Mary",
        "Marzena",
        "Mathilda",
        "Minodora",
        "Mirabel",
        "Miruna",
        "Muriel",
        "Myrtle",
        "Nimira",
        "Nyanka",
        "Olivenka",
        "Patrina",
        "Rosavalda",
        "Ruxandra",
        "Sasha",
        "Sorina",
        "Sorvia",
        "Stefania",
        "Stella",
        "Tasha",
        "Tatyana",
        "Tereska",
        "Valentina",
        "Varushka",
        "Vasha",
        "Vasilka",
        "Victoria",
        "Volenta",
        "Wensencia",
        "Willemina",
        "Yelena",
        "Yolanda",
        "Zondra",
        "Zuleika",
    ],
    male: &[
        "Adrian",
        "Alek",
        "Alexei",
        "Andral",
        "Andrej",
        "Anton",
        "Ariel",
        "Arik",
        "Arrigal",
        "Artank",
        "Artimus",
        "Balthazar",
        "Bascal",
        "Bildrath",
        "Bluto",
        "Bogan",
        "Boris",
        "Bray",
        "Brom",
        "Ciril",
        "Claudiu",
        "Clovin",
        "Cyrus",
        "Gadof",
        "Dag",
        "Dalvan",
        "Damia",
        "Dargos",
        "Darzin",
        "Davian",
        "Dhavit",
        "Dmitri",
        "Doru",
        "Dostron",
        "Dragomir",
        "Eisglaze",
        "Elvir",
        "Emeric",
        "Emil",
        "Erik",
        "Ernst",
        "Escher",
        "Falkon",
        "Franz",
        "Frederick",
        "Freek",
        "Fritz",
        "Gargosh",
        "Godfrey",
        "Gorek",
        "Gralmore",
        "Grigor",
        "Grygori",
        "Gunther",
        "Gustav",
        "Haliq",
        "Hans",
        "Harkus",
        "Henrik",
        "Ilya",
        "Intree",
        "Ismark",
        "Ivan",
        "Izek",
        "Jakarion",
        "Jarnwald",
        "Jirko",
        "Karl",
        "Kasimir",
        "Katsky",
        "Kellen",
        "Khazan",
        "Kiril",
        "Klutz",
        "Kobal",
        "Kolya",
        "Kolyan",
        "Korga",
        "Kroval",
        "Krystofor",
        "Lars",
        "Lazlo",
        "Leo",
        "Leonid",
        "Lief",
        "Livius",
        "Lucian",
        "Luvash",
        "Marek",
        "Martin",
        "Milivoj",
        "Miroslav",
        "Mishka",
        "Nikolai",
        "Nikolaj",
        "Nimir",
        "Oleg",
        "Otto",
        "Parpol",
        "Radovan",
        "Radu",
        "Ratka",
        "Rictavio",
        "Savid",
        "Sedrik",
        "Seraz",
        "Sergei",
        "Skennis",
        "Stahbal",
        "Stanimir",
        "Stefan",
        "Szlodar",
        "Tatsaul",
        "Thornboldt",
        "Troisky",
        "Tural",
        "Udo",
        "Urwin",
        "Valentin",
        "Vargas",
        "Vasily",
        "Victor",
        "Viggo",
        "Vilnius",
        "Vladimir",
        "Vladislav",
        "Walter",
        "Yeska",
        "Yesper",
        "Yevgeni",
        "Zsolt",
        "Zygfrek",
    ],
    surname: &[
        "Alastroi",
        "Antonova",
        "Arasek",
        "Atonovich",
        "Barthos",
        "Belasco",
        "Belview",
        "Blinksky",
        "Cantemir",
        "d'Avenir",
        "Dargovich",
        "Dargova",
        "DeSlop",
        "Diavolov",
        "Diminski",
        "Dilisnya",
        "Dolvof",
        "Donavich",
        "Dorakova",
        "Drazkoi",
        "Dr\u{fc}f",
        "du Plumette",
        "Durst",
        "Endorovich",
        "Eris",
        "Fallona von Twitterberg",
        "Garvinski",
        "Gregorovich",
        "Grejenko",
        "Grislek",
        "Groza",
        "Grygorovich",
        "Grygorova",
        "Herrenghast",
        "Hornsgaard",
        "Indi-Bhak",
        "Indirovich",
        "Ivanovich",
        "Ivanova",
        "Ivliskova",
        "Ivliskovich",
        "Janek",
        "Karelova",
        "Karushkin",
        "Kjurls",
        "Kolyana",
        "Kolyanovich",
        "Konstantinovich",
        "Konstantinova",
        "Krezkov",
        "Krezcova",
        "Krogarov",
        "Krushkin",
        "Krykski",
        "Lansten",
        "Larnak",
        "Lazarescu",
        "Lipsiege",
        "Lorensk",
        "Lukovich",
        "Lukresh",
        "Lipsiege",
        "Markivia",
        "Martikov",
        "Marticova",
        "Mironovich",
        "Mironovna",
        "Moldovar",
        "Nikolovich",
        "Nikolova",
        "Nimblenobs",
        "Nimirovich",
        "Nimirova",
        "Ofenheiss",
        "Olensky",
        "Oronovich",
        "Oronova",
        "Petrovich",
        "Petrovna",
        "Polensky",
        "Popofsky",
        "Radovich",
        "Radova",
        "Rilsky",
        "Rikalova",
        "Romulich",
        "Ruvak",
        "Sik-Valoo",
        "Spinwitovich",
        "Stefanovich",
        "Stefanova",
        "Stoyanovich",
        "Strazni",
        "Swilovich",
        "Swilova",
        "Szlodarovich",
        "Taltos",
        "Targolov",
        "Targolova",
        "Tomescu",
        "Toranescu",
        "Tripalotsky",
        "Tyminski",
        "Ulbrek",
        "Ulrich",
        "Vadu",
        "Vallakovich",
        "van der Voort",
        "Velikov",
        "Velikovna",
        "Vilisevic",
        "Vinshaw",
        "Voltanescu",
        "von Holtz",
        "von Weerg",
        "Vonderbucks",
        "Wachter",
        "Yolensky",
        "Yunk",
        "Zalenski",
        "Zalken",
    ],
};

const BEDINE: EthnicityNames = EthnicityNames {
    female: &["Aisha", "Farah", "Nura", "Rashida", "Zalebyeh"],
    male: &["Aali", "Rashid", "Tahnon", "Tanzim", "Whalide"],
    surname: &[
        "Alaii",
        "Bordjia",
        "Clelarra",
        "Desai",
        "Dakawa",
        "Dursalai",
        "Goldor",
        "Iriphawa",
        "Kellordrai",
        "Lalajar",
        "Qahtan",
        "Yethai",
        "Zazalaar",
    ],
};

const CALISHITE: EthnicityNames = EthnicityNames {
    female: &[
        "Atala", "Ceidil", "Hama", "Jasmal", "Meilil", "Seipora", "Yasheira", "Zasheida",
    ],
    male: &[
        "Aseir", "Bardeid", "Haseid", "Khemed", "Mehmen", "Sudeiman", "Zasheir",
    ],
    surname: &[
        "Basha", "Dumein", "Jassan", "Khalid", "Mostana", "Pashar", "Rein",
    ],
};

const CHONDATHAN: EthnicityNames = EthnicityNames {
    female: &[
        "Aleina", "Andwe", "Arveene", "Ava", "Belinda", "Belle", "Belynne", "Bertrice", "Bloeth",
        "Bronwyn", "Chalkie", "Daelia", "Diana", "Ebela", "Elsa", "Erliza", "Esvele", "Freda",
        "Gardorra", "Gildha", "Haeleeya", "Halia", "Hesten", "Jelayne", "Jhessail", "Kerri",
        "Kharissa", "Kim", "Linene", "Lottie", "Luna", "Lureene", "Maza", "Minghee", "Miri",
        "Mirna", "Mischka", "Moguhl", "Morwen", "Nestra", "Nilsa", "Ocheri", "Rowan", "Shandri",
        "Silvana", "Tanas", "Teresa", "Tessele", "Thalamra", "Thistle", "Tiarshe", "Tistyana",
        "Trilena", "Ylienna",
    ],
    male: &[
        "Aldith",
        "Alger",
        "Ander",
        "Athgar",
        "Brawn",
        "Burton",
        "Carkuss",
        "Daerismun",
        "Daeros",
        "Dagult",
        "Daren",
        "Darvin",
        "Dauner",
        "Derid",
        "Dillard",
        "Dorn",
        "Elmar",
        "Evendur",
        "Favric",
        "Gorstag",
        "Grauman",
        "Grim",
        "Grovet",
        "Harbin",
        "Harburk",
        "Helm",
        "Iarno",
        "Imdarr",
        "Jarl",
        "Javen",
        "Jenkin",
        "Jeremy",
        "Kaidrod",
        "Kal",
        "Lanar",
        "Malark",
        "Maldwyn",
        "Menard",
        "Micah",
        "Morn",
        "Nars",
        "Narth",
        "Nasher",
        "Palien",
        "Pip",
        "Randal",
        "Rasqel",
        "Selin",
        "Sigil",
        "Sildar",
        "Stedd",
        "Taumarik",
        "Thalan",
        "Thamal",
        "Thavus",
        "Thel",
        "Toblen",
        "Torlin",
        "Ulder",
        "Yander",
        "York",
    ],
    surname: &[
        "Aerath",
        "Alagondar",
        "Albrek",
        "Amblecrown",
        "Amcathra",
        "Ammakyl",
        "Anteos",
        "Anuvien",
        "Baldasker",
        "Barthen",
        "Battleby",
        "Boot",
        "Brightlance",
        "Buckman",
        "Burr",
        "Cadrasz",
        "Caradoon",
        "Chatte",
        "Clearlake",
        "Creed",
        "Cururen",
        "Daggerford",
        "Dendrar",
        "Duhn",
        "Dundragon",
        "Eagleshields",
        "Embuirhan",
        "Emmert",
        "Evenwood",
        "Fenwick",
        "Fieldsalder",
        "Frakk",
        "Friedson",
        "Graywind",
        "Greycastle",
        "Hallwinter",
        "Hanadroum",
        "Harpell",
        "Hartwick",
        "Hemzar",
        "Ilzimmer",
        "Inchtarwurn",
        "Jenz",
        "Jhansczil",
        "Kreeg",
        "Kromlor",
        "MacFinn",
        "Mammlar",
        "Mare",
        "Margaster",
        "McGable",
        "Neverember",
        "Oglyntyr",
        "Ostever",
        "Palyr",
        "Portyr",
        "Rault",
        "Raurym",
        "Ravenguard",
        "Relvaunder",
        "Roaringhorn",
        "Ruudheart",
        "Ruthiol",
        "Sharke",
        "Sharnshield",
        "Silvershield",
        "Snoot",
        "Solmen",
        "Splintfig",
        "Stelmane",
        "Stonehill",
        "Suldivver",
        "Tallstag",
        "Tarm",
        "Tarmikos",
        "Teeshe",
        "Thent",
        "Thornton",
        "Tresendar",
        "Twotooth",
        "Vanthampur",
        "Vloot",
        "Wester",
        "Zelorrgosz",
    ],
};

const DAMARAN: EthnicityNames = EthnicityNames {
    female: &[
        "Alethra", "Kara", "Katernin", "Mara", "Natali", "Olma", "Tana", "Zora",
    ],
    male: &[
        "Bor", "Fodel", "Glar", "Grigor", "Igan", "Ivor", "Kosef", "Mival", "Orel", "Pavel",
        "Sergor",
    ],
    surname: &[
        "Bersk", "Chernin", "Dotsk", "Kulenov", "Marsk", "Nemetsk", "Shemov", "Starag",
    ],
};

const FFOLK: EthnicityNames = EthnicityNames {
    female: &["Alicia", "Gennifer", "Meridith", "Elaine", "Olivia"],
    male: &["Artur", "Bern", "Colin", "Manfred", "Tristan"],
    surname: &["Archer", "Gareth", "Leed", "Kendrick", "Morgan", "Waters"],
};

const GUR: EthnicityNames = EthnicityNames {
    female: &["Varra", "Ulmarra", "Imza", "Navarra", "Yuldra"],
    male: &["Boriv", "Gardar", "Madevik", "Vlad"],
    surname: &["Chergoba", "Drazlad", "Tazyara", "Vargoba", "Stayankina"],
};

const HALRUAAN: EthnicityNames = EthnicityNames {
    female: &["Aithe", "Chalan", "Oloma", "Phaele", "Sarade"],
    male: &[
        "Aldym",
        "Chand",
        "Meleghost",
        "Presmer",
        "Sandrue",
        "Uregaunt",
    ],
    surname: &["Avhoste", "Darante", "Maurmeril", "Stamaraster"],
};

const ILLUSKAN: EthnicityNames = EthnicityNames {
    female: &[
        "Amafrey", "Arla", "Barri", "Betha", "Cefrey", "Dabahl", "Dagmaer", "Davena", "Druette",
        "Fryer", "Goldie", "Heian", "Kethra", "Mara", "Olga", "Silifrey", "Teega", "Tevya",
        "Throa", "Wemp", "Westra",
    ],
    male: &[
        "Ander",
        "Bardok",
        "Benham",
        "Beniago",
        "Blath",
        "Bran",
        "Cashaan",
        "Fallinoor",
        "Frath",
        "Geth",
        "Hartouchen",
        "Jendrick",
        "Lander",
        "Luth",
        "Malcer",
        "Milo",
        "Noriel",
        "Rakeem",
        "Stor",
        "Taman",
        "Travis",
        "Tortuk",
        "Urth",
        "Velos",
        "Whiskey",
        "Zelenn",
        "Zurb",
    ],
    surname: &[
        "Baram",
        "Brightwood",
        "Helder",
        "Hoffman",
        "Hornraven",
        "Kurth",
        "Lackman",
        "Nuxoll",
        "Razortongue",
        "Rethnor",
        "Stormwind",
        "Suljack",
        "Taerl",
        "Windrivver",
    ],
};

const IMASKARI: EthnicityNames = EthnicityNames {
    female: &["Apret", "Bask", "Fanul", "Mokat", "Nismet", "Ril"],
    male: &["Chavra", "Duma", "Hukir", "Jama", "Pradir", "Sikhil"],
    surname: &["Datharathi", "Melpurvatta", "Nalambar", "Tiliputakas"],
};

const MULAN: EthnicityNames = EthnicityNames {
    female: &[
        "Arizima", "Chathi", "Nephis", "Nulara", "Murithi", "Sefris", "Thola", "Umara", "Zolis",
    ],
    male: &[
        "Aoth",
        "Bareris",
        "Ehput-Ki",
        "Hamun",
        "Kethoth",
        "Mumed",
        "Ramas",
        "Reidoth",
        "So-Kehur",
        "Thazar-De",
        "Urhur",
    ],
    surname: &[
        "Ankhalab",
        "Anskuld",
        "Fezim",
        "Hahpet",
        "Kost",
        "Nathandem",
        "Sepret",
        "Uuthrakt",
    ],
};

const NAR: EthnicityNames = EthnicityNames {
    female: &["Anva", "Dasha", "Dima", "Olga", "Westra", "Zlatara"],
    male: &[
        "Avan", "Ostaram", "Petro", "Stor", "Taman", "Thalaman", "Urth",
    ],
    surname: &[
        "Dashkev",
        "Hargoth",
        "Laboda",
        "Lackman",
        "Stonar",
        "Stormwind",
        "Sulyma",
    ],
};

const RASHEMI: EthnicityNames = EthnicityNames {
    female: &[
        "Fyevarra", "Hulmarra", "Immith", "Imzel", "Navarra", "Shevarra", "Tammith", "Yuldra",
    ],
    male: &[
        "Borivik",
        "Faurgar",
        "Gallio",
        "Jandar",
        "Kanithar",
        "Madislak",
        "Ralmevik",
        "Shaumar",
        "Vladislak",
    ],
    surname: &[
        "Chergoba",
        "Dyernina",
        "Elibro",
        "Iltazyara",
        "Murnyethara",
        "Stayanoga",
        "Ulmokina",
    ],
};

const SHAARAN: EthnicityNames = EthnicityNames {
    female: &["Anet", "Bes", "Idim", "Lenet", "Moqem", "Neghet", "Sihvet"],
    male: &[
        "Awar", "Cohis", "Damota", "Gewar", "Hapah", "Laskaw", "Senesaw", "Tokhis",
    ],
    surname: &["Cor Marak", "Laumee Harr", "Moq Qu Harr", "Woraw Tarak"],
};

const SHOU: EthnicityNames = EthnicityNames {
    female: &["Bai", "Chao", "Jia", "Lei", "Mei", "Qiao", "Shui", "Tai"],
    male: &[
        "An", "Chen", "Chi", "Fai", "Jiang", "Jun", "Lian", "Long", "Meng", "On", "Shan", "Shui",
        "Wen",
    ],
    surname: &[
        "Chien", "Huang", "Kao", "Kung", "Lao", "Ling", "Mei", "Pin", "Shin", "Sum", "Tan", "Wan",
    ],
};

const TUIGAN: EthnicityNames = EthnicityNames {
    female: &["Bolormaa", "Bortai", "Erdene", "Naran"],
    male: &["Atlan", "Bayar", "Chingis", "Chinua", "Mongke", "Temur"],
    surname: &[],
};

const TURAMI: EthnicityNames = EthnicityNames {
    female: &[
        "Balama", "Dona", "Faila", "Jalana", "Luisa", "Marta", "Quara", "Selise", "Vonda",
    ],
    male: &[
        "Anton", "Diero", "Marcon", "Pieron", "Rimardo", "Romero", "Salazar", "Umbero",
    ],
    surname: &[
        "Agosto",
        "Astorio",
        "Calabra",
        "Domine",
        "Falone",
        "Marivaldi",
        "Pisacar",
        "Ramondo",
    ],
};

const ULUTIUN: EthnicityNames = EthnicityNames {
    female: &["Akna", "Chena", "Kaya", "Sedna", "Ublereak"],
    male: &["Amak", "Chu", "Imnek", "Kanut", "Siku"],
    surname: &[],
};
