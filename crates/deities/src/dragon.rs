use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const DRAGON: &[Deity] = &[
    Deity {
        name: "Bahamut",
        titles: &["god of good"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Life, Domain::War],
        symbols: &["Dragon's head in profile"],
        pantheon: Pantheon::Dragon,
    },
    Deity {
        name: "Tiamat",
        titles: &["god of evil"],
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Trickery],
        symbols: &["Dragon head with five claw marks"],
        pantheon: Pantheon::Dragon,
    },
];
