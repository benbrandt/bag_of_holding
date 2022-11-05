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

use std::collections::HashSet;

use abilities::Ability;
use characters::{Character, CharacterBuildError};
use descriptions::{Appearance, Backstory};
use languages::Language;
use races::RaceGenerator;
use rand::Rng;
use serde_json::json;
use sizes::HeightAndWeight;
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
    let mut rng = rand_utils::rng_from_entropy();
    let mut character = Character::new().gen_ability_scores(&mut rng);

    let prev_ability_scores = Ability::iter()
        .map(|a| character.ability_scores.as_ref().unwrap().score(a))
        .sum::<u8>();

    character = character.gen_race(&mut rng).unwrap();

    assert!(character.race.is_some());

    let new_ability_scores = Ability::iter()
        .map(|a| character.ability_scores.as_ref().unwrap().score(a))
        .sum::<u8>();

    assert_eq!(
        new_ability_scores,
        prev_ability_scores
            + character
                .race
                .unwrap()
                .ability_increases()
                .iter()
                .sum::<u8>()
    );
}

#[test]
fn generate_age() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new()
        .gen_ability_scores(&mut rng)
        .gen_race(&mut rng)
        .unwrap()
        .gen_age(&mut rng)
        .unwrap();

    assert!(character.age.unwrap() > 0);
}

#[test]
fn generate_height_and_weight() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new()
        .gen_ability_scores(&mut rng)
        .gen_race(&mut rng)
        .unwrap()
        .gen_height_and_weight(&mut rng)
        .unwrap();

    assert!(character.height_and_weight.unwrap().height > 0);
    assert!(character.height_and_weight.unwrap().weight > 0);
}

#[test]
fn generate_deity() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new()
        .gen_ability_scores(&mut rng)
        .gen_race(&mut rng)
        .unwrap()
        .gen_deity(&mut rng);
    assert!(character.is_ok());
}

#[test]
fn generate_alignment() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new().gen_alignment(&mut rng);

    assert!(character.alignment.is_some());
}

#[test]
fn generate_languages() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new()
        .gen_ability_scores(&mut rng)
        .gen_race(&mut rng)
        .unwrap()
        .gen_languages(&mut rng)
        .unwrap();

    assert!(character.languages.len() > 1);
    assert!(character.languages.contains(&Language::Common));
}

#[test]
fn generate_full_character() {
    let character: Character = rand_utils::rng_from_entropy().gen();

    assert!(!character.name.is_empty());
    assert!(character.ability_scores.is_some());
    assert!(character.race.is_some());
    assert!(character.age.is_some());
    assert!(character.height_and_weight.is_some());
    assert!(character.languages.len() > 1);
    assert!(character.alignment.is_some());

    let race_appearance = character.race.as_ref().unwrap().appearance();
    let race_appearance = race_appearance.iter().collect::<HashSet<_>>();
    let character_appearance = character.appearance();
    let character_appearance = character_appearance.iter().collect::<HashSet<_>>();
    assert!(race_appearance.is_subset(&character_appearance));
    let race_backstory = character.race.as_ref().unwrap().backstory();
    let race_backstory = race_backstory.iter().collect::<HashSet<_>>();
    let character_backstory = character.backstory();
    let character_backstory = character_backstory.iter().collect::<HashSet<_>>();
    assert!(race_backstory.is_subset(&character_backstory));
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
    assert_eq!(
        character.race.as_ref().unwrap().citation(),
        serialized["race"]
    );
    assert_eq!(character.age.unwrap(), serialized["age"]);

    let HeightAndWeight { height, weight } = character.height_and_weight.unwrap();
    assert_eq!(height, serialized["height"]);
    assert_eq!(weight, serialized["weight"]);
    assert_eq!(
        json!(character.race.as_ref().unwrap().size()),
        serialized["size"]
    );
    assert_eq!(
        character.alignment.unwrap().to_string(),
        serialized["alignment"]
    );

    assert_eq!(json!(character.languages), serialized["languages"]);
    assert_eq!(json!(character.deity), serialized["deity"]);
}

#[test]
fn ability_scores_chosen_before_race() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new();

    assert_eq!(
        character.gen_race(&mut rng).unwrap_err(),
        CharacterBuildError::MissingAbilityScores
    );
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

#[test]
fn race_is_chosen_before_age() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new();

    assert_eq!(
        character.gen_age(&mut rng).unwrap_err(),
        CharacterBuildError::MissingRace
    );
}

#[test]
fn race_is_chosen_before_height_and_weight() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new();

    assert_eq!(
        character.gen_height_and_weight(&mut rng).unwrap_err(),
        CharacterBuildError::MissingRace
    );
}

#[test]
fn race_is_chosen_before_languages() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new();

    assert_eq!(
        character.gen_languages(&mut rng).unwrap_err(),
        CharacterBuildError::MissingRace
    );
}

#[test]
fn race_is_chosen_before_deity() {
    let mut rng = rand_utils::rng_from_entropy();
    let character = Character::new();

    assert_eq!(
        character.gen_deity(&mut rng).unwrap_err(),
        CharacterBuildError::MissingRace
    );
}
