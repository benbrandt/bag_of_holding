use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const BUGBEAR: &[Deity] = &[Deity {
    name: "Hruggek",
    alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
    domains: &[Domain::War],
    pantheon: Pantheon::Bugbear,
    symbols: &["Morningstar"],
    titles: &["god of violence"],
}];
