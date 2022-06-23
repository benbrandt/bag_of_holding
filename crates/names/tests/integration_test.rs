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
use strum::IntoEnumIterator;

#[test]
fn can_generate_all_names() {
    let mut rng = rand_utils::rng_from_entropy();
    for name in Name::iter() {
        assert!(!name.gen(&mut rng).is_empty());
    }
}
