use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const DUERGAR: &[Deity] = &[
    Deity {
        name: "Deep Duerra",
        alignment: Alignment::LAWFUL_EVIL,
        domains: &[Domain::Arcana, Domain::Knowledge, Domain::War],
        pantheon: Pantheon::Duergar,
        symbols: &["Mind flayer skill"],
        titles: &["goddess of conquest and psionics"],
    },
    Deity {
        name: "Laduguer",
        alignment: Alignment::LAWFUL_EVIL,
        domains: &[Domain::Arcana, Domain::Death, Domain::Forge],
        pantheon: Pantheon::Duergar,
        symbols: &["Broken arrow"],
        titles: &["god of magic and slavery", "god of labor and slavery"],
    },
];
