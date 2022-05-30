#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use abilities::{Ability, AbilityScores};
use itertools::{repeat_n, Itertools};
use rand::Rng;
use statrs::statistics::Statistics;
use strum::IntoEnumIterator;

#[test]
fn generate_ability_scores() {
    // Generate all possible results of rolling 4d6 and keeping top 3
    let possibilities = repeat_n(1..=6, 4)
        .multi_cartesian_product()
        .map(|l| l.into_iter().sorted().rev().take(3).sum::<i32>())
        .map(f64::from)
        .collect_vec();
    let mean = possibilities.iter().mean();
    let std_dev = possibilities.iter().std_dev();

    let scores: AbilityScores = rand_utils::rng_from_entropy().gen();

    let results = Ability::iter().map(|a| scores.score(a)).collect_vec();
    // All scores are in valid range
    assert!(results.iter().all(|r| (3..=18).contains(r)));
    // Average is within expected range
    assert!((results.into_iter().map(f64::from).mean() - mean).abs() < std_dev);
}

#[test]
fn modifiers() {
    let scores: AbilityScores = rand_utils::rng_from_entropy().gen();

    // All modifiers are in valid range
    let results = Ability::iter().map(|a| scores.modifier(a)).collect_vec();
    assert!(results.iter().all(|r| (-5..=5).contains(r)));
}
