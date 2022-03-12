use dice::Die;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use strum::IntoEnumIterator;

#[test]
fn roll() {
    let mut rng = Pcg64::from_entropy();
    for die in Die::iter() {
        let roll = die.roll(&mut rng);
        assert!((1..=die.into()).contains(&roll));
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
