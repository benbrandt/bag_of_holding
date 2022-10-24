use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const KOBOLD: &[Deity] = &[Deity {
    name: "Kurtulmak",
    alignment: Alignment::LAWFUL_EVIL,
    domains: &[Domain::War],
    pantheon: Pantheon::Kobold,
    symbols: &["Gnome skull"],
    titles: &["god of war and mining"],
}];
