use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const GIANT: &[Deity] = &[
    Deity {
        name: "Grolantor",
        alignment: Alignment::CHAOTIC_EVIL,
        domains: &[Domain::War],
        pantheon: Pantheon::Giant,
        symbols: &["Wooden club"],
        titles: &["hill giant god of war"],
    },
    Deity {
        name: "Skoraeus Stonebones",
        alignment: Alignment::NEUTRAL,
        domains: &[Domain::Knowledge],
        pantheon: Pantheon::Giant,
        symbols: &["Stalactite"],
        titles: &["god of stone giants and art"],
    },
    Deity {
        name: "Surtur",
        alignment: Alignment::LAWFUL_EVIL,
        domains: &[Domain::Knowledge, Domain::War],
        pantheon: Pantheon::Giant,
        symbols: &["Flaming sword"],
        titles: &["god of fire giants and craft"],
    },
    Deity {
        name: "Thrym",
        alignment: Alignment::CHAOTIC_EVIL,
        domains: &[Domain::War],
        pantheon: Pantheon::Giant,
        symbols: &["White double-bladed axe"],
        titles: &["god of frost giants and strength"],
    },
];
