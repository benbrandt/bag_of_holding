#![warn(
    clippy::pedantic,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms
)]

use dice::Die;
use itertools::Itertools;
use statrs::{
    distribution::Uniform,
    statistics::{Distribution, Statistics},
};
use strum::IntoEnumIterator;

#[test]
fn roll() {
    let mut rng = rand_utils::rng_from_entropy();

    for die in Die::iter() {
        let dist = Uniform::new(1.0, die.into()).unwrap();

        let rolls = (0..u32::from(die) * 10)
            .into_iter()
            .map(|_| die.roll(&mut rng))
            .collect_vec();

        assert!(rolls.iter().all(|roll| (1..=die.into()).contains(roll)));
        let mean = rolls.iter().map(|&r| f64::from(r)).mean();
        assert!((mean - dist.mean().unwrap()).abs() < dist.std_dev().unwrap());
    }
}

#[test]
fn roll_multiple() {
    let mut rng = rand_utils::rng_from_entropy();
    for die in Die::iter() {
        // Roll a normal range of dice
        for i in 1..=12 {
            let rolls = die.roll_multiple(&mut rng, i).collect_vec();
            assert_eq!(rolls.len(), i as usize);
            assert!(rolls.iter().all(|d| (1..=die.into()).contains(d)));
        }
    }
}
