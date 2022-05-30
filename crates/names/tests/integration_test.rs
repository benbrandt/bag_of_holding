#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use names::Dwarf;
use rand::Rng;

#[test]
fn dwarven_name() {
    let name: Dwarf = rand_utils::rng_from_entropy().gen();
    assert!(!name.first_name.is_empty());
    assert!(!name.clan_name.is_empty());
    // Formats full name
    assert_eq!(
        name.to_string(),
        format!("{} {}", name.first_name, name.clan_name)
    );
}
