use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const LIZARDFOLK: &[Deity] = &[Deity {
    name: "Semuanya",
    alignment: Alignment::NEUTRAL,
    domains: &[Domain::Life],
    pantheon: Pantheon::Lizardfolk,
    symbols: &["Egg"],
    titles: &["deity of survival"],
}];
