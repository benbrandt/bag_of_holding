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

use abilities::{Ability, AbilityScores};
use itertools::{repeat_n, Itertools};
use rand::Rng;
use serde_json::json;
use statrs::statistics::Statistics;
use strum::IntoEnumIterator;

#[test]
fn generate_ability_scores() {
    // Generate all possible results of rolling 4d6 and keeping top 3
    let possibilities = repeat_n(1..=6, 4)
        .multi_cartesian_product()
        .map(|l| l.into_iter().sorted().rev().take(3).sum::<i32>())
        .map(f64::from)
        .collect::<Vec<_>>();
    let mean = possibilities.iter().mean();
    let std_dev = possibilities.iter().std_dev();

    let scores: AbilityScores = rand_utils::rng_from_entropy().gen();

    let results = Ability::iter().map(|a| scores.score(a)).collect::<Vec<_>>();
    // All scores are in valid range
    assert!(results.iter().all(|r| (3..=18).contains(r)));
    // Average is within expected range
    assert!((results.into_iter().map(f64::from).mean() - mean).abs() < std_dev);
}

#[test]
fn modifiers() {
    let scores: AbilityScores = rand_utils::rng_from_entropy().gen();

    // All modifiers are in valid range
    let results = Ability::iter()
        .map(|a| scores.modifier(a))
        .collect::<Vec<_>>();
    assert!(results.iter().all(|r| (-5..=5).contains(r)));
}

#[test]
fn serialization() {
    let scores: AbilityScores = rand_utils::rng_from_entropy().gen();
    let serialized = json!(scores);

    for ability in Ability::iter() {
        assert_eq!(
            json!({
                "base": scores.score(ability),
                "racial_increase": 0,
                "score": scores.score(ability),
                "modifier": scores.modifier(ability),
            }),
            serialized[ability.to_string()]
        );
    }
}

#[test]
fn racial_increases() {
    let mut rng = rand::thread_rng();
    let mut scores: AbilityScores = rng.gen();
    let increases = &[1, 2];

    // Cache previous scores
    let prev_scores = scores.clone();

    // Update scores
    scores.gen_racial_increases(&mut rng, increases);

    // Check that two were updated with the correct diff
    let diffs = Ability::iter()
        .map(|a| scores.score(a) - prev_scores.score(a))
        .filter(|s| s > &0)
        .sorted()
        .collect::<Vec<_>>();

    assert_eq!(diffs, increases);

    // Check that none are above 20
    assert!(Ability::iter().map(|a| scores.score(a)).all(|s| s <= 20));
}
