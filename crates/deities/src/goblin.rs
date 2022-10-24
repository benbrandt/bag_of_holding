use alignments::Alignment;

use crate::{Deity, Domain, Pantheon};

pub const GOBLIN: &[Deity] = &[Deity {
    name: "Maglubiyet",
    alignment: Alignment::LAWFUL_EVIL,
    domains: &[Domain::War],
    symbols: &["Bloody axe"],
    titles: &["god of war"],
    pantheon: Pantheon::Goblin,
}];
