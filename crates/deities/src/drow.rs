use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const DROW: &[Deity] = &[
    Deity {
        name: "Eilistraee",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Good),
        domains: &[Domain::Life, Domain::Light, Domain::Nature],
        pantheon: Pantheon::Drow,
        symbols: &["Sword-wielding dancing drow female silhouetted against the full moon"],
        titles: &[
            "goddess of song, beauty, swordwork, hunting, and moonlight",
            "goddess of freedom, moonlight, and song",
        ],
    },
    Deity {
        name: "Ghaunadaur",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::War],
        pantheon: Pantheon::Drow,
        symbols: &["Purple eye with black sclera"],
        titles: &["deity of oozes, slimes, and outcasts"],
    },
    Deity {
        name: "Keptolo",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Nature, Domain::Trickery],
        pantheon: Pantheon::Drow,
        symbols: &["Mushroom"],
        titles: &["deity of beauty, hedonism, and fertility"],
    },
    Deity {
        name: "Kiaransalee",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Arcana, Domain::Death],
        pantheon: Pantheon::Drow,
        symbols: &["Female drow hand wearing many silver rings"],
        titles: &["goddess of necromancy", "goddess of the undead"],
    },
    Deity {
        name: "Malyk",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Tempest, Domain::Trickery],
        pantheon: Pantheon::Drow,
        symbols: &["A flame in a tear or a multihued vortex"],
        titles: &["deity of chaos, rebellion, and wild magic"],
    },
    Deity {
        name: "Lolth",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Trickery, Domain::War],
        pantheon: Pantheon::Drow,
        symbols: &["Spider"],
        titles: &[
            "goddess of spiders",
            "the Demon Queen of Spiders",
            "primary god of drow",
        ],
    },
    Deity {
        name: "Selvetarm",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::War],
        pantheon: Pantheon::Drow,
        symbols: &["Spider over crossed sword and mace"],
        titles: &["god of warriors", "god of warriors and slaughter"],
    },
    Deity {
        name: "Vhaeraun",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Trickery, Domain::War],
        pantheon: Pantheon::Drow,
        symbols: &["Black mask with blue glass lenses inset over eyes"],
        titles: &["god of thieves", "god of arrogance and thieves"],
    },
    Deity {
        name: "Zinzerena",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Neutral),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::Drow,
        symbols: &["Shortsword draped with cloth"],
        titles: &["deity of assassination, illusion, and lies"],
    },
];
