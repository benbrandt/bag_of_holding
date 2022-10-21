use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const GIANT: &[Deity] = &[
    Deity {
        name: "Grolantor",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::War],
        pantheon: Pantheon::Giant,
        symbols: &["Wooden club"],
        titles: &["hill giant god of war"],
    },
    Deity {
        name: "Skoraeus Stonebones",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        pantheon: Pantheon::Giant,
        symbols: &["Stalactite"],
        titles: &["god of stone giants and art"],
    },
    Deity {
        name: "Surtur",
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Knowledge, Domain::War],
        pantheon: Pantheon::Giant,
        symbols: &["Flaming sword"],
        titles: &["god of fire giants and craft"],
    },
    Deity {
        name: "Thrym",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::War],
        pantheon: Pantheon::Giant,
        symbols: &["White double-bladed axe"],
        titles: &["god of frost giants and strength"],
    },
];
