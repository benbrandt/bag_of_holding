use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const KOBOLD: &[Deity] = &[Deity {
    name: "Kurtulmak",
    alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
    domains: &[Domain::War],
    pantheon: Pantheon::Kobold,
    symbols: &["Gnome skull"],
    titles: &["god of war and mining"],
}];
