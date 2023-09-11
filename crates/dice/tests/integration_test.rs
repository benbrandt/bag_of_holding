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

use dice::{Die, Roll};
use rand::{seq::IteratorRandom, Rng};
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
            .map(|_| die.roll(&mut rng))
            .collect::<Vec<_>>();

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
            let rolls = die.roll_multiple(&mut rng, i).collect::<Vec<_>>();
            assert_eq!(rolls.len(), i);
            assert!(rolls.iter().all(|d| (1..=die.into()).contains(d)));
        }
    }
}

#[test]
fn roll_command() {
    let mut rng = rand_utils::rng_from_entropy();
    let amount = rng.gen::<u8>() as usize;
    let die = Die::iter().choose(&mut rng).unwrap();
    let rolls = Roll::new(amount, die).gen(&mut rng);

    assert_eq!(rolls.count(), amount);
}
