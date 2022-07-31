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
use sources::Sources;
use strum::IntoEnumIterator;

#[test]
fn generate_ability_scores() {
    let character = Character::new().gen_ability_scores(&mut rand_utils::rng_from_entropy());

    for ability in Ability::iter() {
        assert!(character.ability_scores.as_ref().unwrap().score(ability) > 0);
    }
}

#[test]
fn generate_race() {
    let character = Character::new().gen_race(&mut rand_utils::rng_from_entropy());

    assert!(character.race.is_some());
}

#[test]
fn generate_full_character() {
    let character: Character = rand_utils::rng_from_entropy().gen();

    assert!(character.ability_scores.is_some());
    assert!(character.race.is_some());
}

#[test]
fn serialize_to_character_sheet() {
    let character: Character = rand_utils::rng_from_entropy().gen();
    let serialized = serde_json::to_value(&character).unwrap();
    assert_eq!(
        serde_json::to_value(&character.ability_scores).unwrap(),
        serialized["ability_scores"]
    );
    assert_eq!(character.race.unwrap().citation(), serialized["race"]);
}
