use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const DRAGON: &[Deity] = &[
    Deity {
        name: "Bahamut",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Life, Domain::War],
        pantheon: Pantheon::Dragon,
        symbols: &["Dragon's head in profile"],
        titles: &["god of good"],
    },
    Deity {
        name: "Tiamat",
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::Dragon,
        symbols: &["Dragon head with five claw marks"],
        titles: &["god of evil"],
    },
];
