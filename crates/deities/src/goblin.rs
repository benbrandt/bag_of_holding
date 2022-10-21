use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const GOBLIN: &[Deity] = &[Deity {
    name: "Maglubiyet",
    alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
    domains: &[Domain::War],
    symbols: &["Bloody axe"],
    titles: &["god of war"],
    pantheon: Pantheon::Goblin,
}];
