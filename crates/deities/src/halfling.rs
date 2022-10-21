use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const HALFLING: &[Deity] = &[
    Deity {
        name: "Arvoreen",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::War],
        pantheon: Pantheon::Halfling,
        symbols: &["Crossed short swords"],
        titles: &[
            "god of vigilance and war",
            "defender-god",
            "watchful protector",
        ],
    },
    Deity {
        name: "Brandobaris",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::Halfling,
        symbols: &["Halfling footprint"],
        titles: &[
            "god of thievery, stealth, and adventure",
            "god of adventure and thievery",
        ],
    },
    Deity {
        name: "Charmalaine",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::Halfling,
        symbols: &["Burning boot print"],
        titles: &["deity of keen senses and luck"],
    },
    Deity {
        name: "Cyrrollalee",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Life],
        pantheon: Pantheon::Halfling,
        symbols: &["An open door"],
        titles: &[
            "goddess of hearth, hospitality, and home",
            "goddess of trust and handicrafts",
            "goddess of hearth and home",
        ],
    },
    Deity {
        name: "Sheela Peryroyl",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Nature, Domain::Tempest],
        pantheon: Pantheon::Halfling,
        symbols: &["Flower"],
        titles: &[
            "goddess of agriculture and weather",
            "goddess of nature",
            "goddess of love, song, and dance",
            "the lady of fields, streams, and the wilds found in shire and glen",
            "the Green Sister of Yondalla",
            "goddess of agriculture, nature, and weather",
        ],
    },
    Deity {
        name: "Urogalan",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Death, Domain::Grave, Domain::Knowledge],
        pantheon: Pantheon::Halfling,
        symbols: &["Silhouette of a dog's head"],
        titles: &["god of earth and death"],
    },
    Deity {
        name: "Yondalla",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Life],
        pantheon: Pantheon::Halfling,
        symbols: &["Cornucopia on a shield", "shield", "cornucopia"],
        titles: &[
            "goddess of bounty, fertility, and protection",
            "protector of hearth, home, and family",
            "the Blessed One",
            "Primary goddess of halflings",
        ],
    },
];
