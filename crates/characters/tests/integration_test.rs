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
use characters::Character;
use rand::Rng;
use strum::IntoEnumIterator;

#[test]
fn generate_ability_scores() {
    let character = Character::new().ability_scores(&mut rand_utils::rng_from_entropy());

    for ability in Ability::iter() {
        assert!(character.ability_scores.as_ref().unwrap().score(ability) > 0);
    }
}

#[test]
fn generate_full_character() {
    let character: Character = rand_utils::rng_from_entropy().gen();

    assert!(character.ability_scores.is_some());
}
