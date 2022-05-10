#![warn(clippy::pedantic, rust_2018_idioms)]

use abilities::{Ability, AbilityScores};
use itertools::{repeat_n, Itertools};
use rand::SeedableRng;
use rand_pcg::Pcg64;
use statrs::statistics::Statistics;
use strum::IntoEnumIterator;

#[test]
fn generate_ability_scores() {
    let mut rng = Pcg64::from_entropy();

    // Generate all possible results of rolling 4d6 and keeping top 3
    let possibilities = repeat_n(1..=6, 4)
        .multi_cartesian_product()
        .map(|l| l.into_iter().sorted().rev().take(3).sum::<i32>())
        .map(f64::from)
        .collect_vec();
    let mean = possibilities.iter().mean();
    let std_dev = possibilities.iter().std_dev();

    let scores = AbilityScores::gen(&mut rng);

    let results = Ability::iter().map(|a| scores.score(&a)).collect_vec();
    // All scores are in valid range
    assert!(results.iter().all(|r| (3..=18).contains(r)));
    // Average is within expected range
    assert!((results.into_iter().map(f64::from).mean() - mean).abs() < std_dev);
}
