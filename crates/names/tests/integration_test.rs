#![warn(
    clippy::pedantic,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
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
