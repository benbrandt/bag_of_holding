use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const LIZARDFOLK: &[Deity] = &[Deity {
    name: "Semuanya",
    alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
    domains: &[Domain::Life],
    pantheon: Pantheon::Lizardfolk,
    symbols: &["Egg"],
    titles: &["deity of survival"],
}];
