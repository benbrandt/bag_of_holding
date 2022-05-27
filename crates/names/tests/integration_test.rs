#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use names::{Dwarf, Name};

#[test]
fn dwarven_name() {
    let mut rng = rand_utils::rng_from_entropy();
    let name = Dwarf::gen(&mut rng);
    assert!(!name.first_name.is_empty());
    assert!(!name.clan_name.is_empty());
}
