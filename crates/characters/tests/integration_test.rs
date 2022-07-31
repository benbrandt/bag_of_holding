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
use characters::{Character, CharacterBuildError};
use rand::Rng;
use serde_json::json;
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

    assert!(!character.name.is_empty());
    assert!(character.ability_scores.is_some());
    assert!(character.race.is_some());
}

#[test]
fn serialize_to_character_sheet() {
    let character: Character = rand_utils::rng_from_entropy().gen();
    let serialized = json!(&character);

    assert_eq!(character.name, serialized["name"]);
    assert_eq!(
        json!(&character.ability_scores),
        serialized["ability_scores"]
    );
    assert_eq!(character.race.unwrap().citation(), serialized["race"]);
}

#[test]
fn race_is_chosen_before_name() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new();

    assert_eq!(
        character.gen_name(&mut rng).unwrap_err(),
        CharacterBuildError::MissingRace
    );
}
