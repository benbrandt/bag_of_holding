use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const DRAGONLANCE: &[Deity] = &[
    Deity {
        name: "Paladine",
        titles: &["god of rulers and guardians"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::War],
        symbols: &["Silver triangle"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Branchala",
        titles: &["god of music"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Light],
        symbols: &["Bard's harp"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Habbakuk",
        titles: &["god of animal life and the sea"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Nature, Domain::Tempest],
        symbols: &["Blue bird"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Kiri-Jolith",
        titles: &["god of honor and war"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::War],
        symbols: &["Bison's horns"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Majere",
        titles: &["god of meditation and order"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Knowledge],
        symbols: &["Copper spider"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Mishakal",
        titles: &["goddess of healing"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Knowledge, Domain::Life],
        symbols: &["Blue infinity sign"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Solinari",
        titles: &["god of good magic"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[],
        symbols: &["White circle or sphere"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Gilean",
        titles: &["god of knowledge"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        symbols: &["Open book"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Chislev",
        titles: &["goddess of nature"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Nature],
        symbols: &["Feather"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Reorx",
        titles: &["god of craft"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        symbols: &["Forging hammer"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Shinare",
        titles: &["goddess of wealth and trade"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge, Domain::Trickery],
        symbols: &["Griffon's wing"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Sirrion",
        titles: &["god of fire and change"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Nature],
        symbols: &["Multi-colored fire"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Zivilyn",
        titles: &["god of wisdom"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        symbols: &["Great green or gold tree"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Lunitari",
        titles: &["goddess of neutral magic"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[],
        symbols: &["Red circle or sphere"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Takhisis",
        titles: &["goddess of night and hatred"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Death],
        symbols: &["Black crescent"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Chemosh",
        titles: &["god of the undead"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Death],
        symbols: &["Yellow skull"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Hiddukel",
        titles: &["god of lies and greed"],
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Trickery],
        symbols: &["Broken merchant's scales"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Morgion",
        titles: &["god of disease and secrecy"],
        alignment: Alignment::new(Attitude::Neutral, Morality::Evil),
        domains: &[Domain::Death],
        symbols: &["Hood with two red eyes"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Sargonnas",
        titles: &["god of vengeance and fire"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::War],
        symbols: &["Stylized red condor"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Zeboim",
        titles: &["goddess of the sea and storms"],
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Tempest],
        symbols: &["Turtle shell"],
        pantheon: Pantheon::Dragonlance,
    },
    Deity {
        name: "Nuitari",
        titles: &["god of evil magic"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[],
        symbols: &["Black circle or sphere"],
        pantheon: Pantheon::Dragonlance,
    },
];
