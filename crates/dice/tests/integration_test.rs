use dice::Die;
use itertools::Itertools;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use statrs::{
    distribution::ChiSquared,
    statistics::{Distribution, Statistics},
};
use strum::IntoEnumIterator;

#[test]
fn roll() {
    let mut rng = Pcg64::from_entropy();

    for die in Die::iter() {
        let die_num: u32 = die.into();
        let die_avg = (die_num - 1) as f64 / 2.0;
        let std_dev = ChiSquared::new(die_avg).unwrap().std_dev().unwrap();

        let rolls = (0..100)
            .into_iter()
            .map(|_| die.roll(&mut rng))
            .collect_vec();

        assert!(rolls.iter().all(|roll| (1..=die_num).contains(roll)));
        let mean = rolls.iter().map(|&r| r as f64).mean();
        assert!((mean - die_avg).abs() < std_dev);
    }
}

#[test]
fn roll_multiple() {
    let mut rng = Pcg64::from_entropy();
    for die in Die::iter() {
        // Roll a normal range of dice
        for i in 1..=12 {
            let rolls = die.roll_multiple(&mut rng, i);
            assert_eq!(rolls.len(), i as usize);
            assert!(rolls.iter().all(|d| (1..=die.into()).contains(d)));
        }
    }
}
