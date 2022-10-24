use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const ORC: &[Deity] = &[
    Deity {
        name: "Bahgtru",
        alignment: Alignment::LAWFUL_EVIL,
        domains: &[Domain::War],
        pantheon: Pantheon::Orc,
        symbols: &["Broken thigh bone"],
        titles: &["god of pure, brute strength", "son of Gruumsh"],
    },
    Deity {
        name: "Gruumsh",
        alignment: Alignment::CHAOTIC_EVIL,
        domains: &[Domain::Tempest, Domain::War],
        pantheon: Pantheon::Orc,
        symbols: &["Unblinking eye"],
        titles: &[
            "god of storms and war",
            "god of conquest, strength, and survival",
            "One-Eye",
        ],
    },
    Deity {
        name: "Ilneval",
        alignment: Alignment::LAWFUL_EVIL,
        domains: &[Domain::War],
        pantheon: Pantheon::Orc,
        symbols: &["Upright blood-spattered sword"],
        titles: &["god of strategy and hordes", "War Master"],
    },
    Deity {
        name: "Luthic",
        alignment: Alignment::LAWFUL_EVIL,
        domains: &[Domain::Life, Domain::Nature],
        pantheon: Pantheon::Orc,
        symbols: &["Orcish rune meaning \"cave entrance\""],
        titles: &[
            "mother-goddess of fertility and healding",
            "goddess of fecundity, caverns, and witchery",
        ],
    },
    Deity {
        name: "Shargaas",
        alignment: Alignment::NEUTRAL_EVIL,
        domains: &[Domain::Trickery],
        pantheon: Pantheon::Orc,
        symbols: &["Red crescent moon with a skull between the moon's horns"],
        titles: &["god of darkness, night, and stealth", "the Night Lord"],
    },
    Deity {
        name: "Yurtrus",
        alignment: Alignment::NEUTRAL_EVIL,
        domains: &[Domain::Death],
        pantheon: Pantheon::Orc,
        symbols: &["White hand, palm outward"],
        titles: &[
            "god of death and disease",
            "god of plagues and death",
            "the White-Handed",
            "Lord of Maggots",
        ],
    },
];
