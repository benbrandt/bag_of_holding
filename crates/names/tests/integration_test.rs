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

use names::Name;

#[test]
fn name() {
    let name = Name::Dwarf.gen(&mut rand_utils::rng_from_entropy());
    assert!(!name.to_string().is_empty());
}
