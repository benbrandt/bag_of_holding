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

use abilities::Ability;
use character::Character;
use rand::Rng;
use strum::IntoEnumIterator;

#[test]
fn generate_character() {
    let character: Character = rand_utils::rng_from_entropy().gen();

    for ability in Ability::iter() {
        assert!(character.ability_scores.score(ability) > 0);
    }
}
