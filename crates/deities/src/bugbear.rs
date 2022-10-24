use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const BUGBEAR: &[Deity] = &[Deity {
    name: "Hruggek",
    alignment: Alignment::CHAOTIC_EVIL,
    domains: &[Domain::War],
    pantheon: Pantheon::Bugbear,
    symbols: &["Morningstar"],
    titles: &["god of violence"],
}];
