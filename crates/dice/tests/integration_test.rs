use dice::Die;
use rand::SeedableRng;
use rand_pcg::Pcg64;

#[test]
fn roll_d4() {
    let mut rng = Pcg64::from_entropy();
    let roll = Die::D4.roll(&mut rng);
    assert!((1..=4).contains(&roll));
}

#[test]
fn roll_multiple_d4() {
    let mut rng = Pcg64::from_entropy();
    // Roll a normal range of dice
    for i in 1..=12 {
        let rolls = Die::D4.roll_multiple(&mut rng, i);
        assert_eq!(rolls.len(), i as usize);
        assert!(rolls.iter().all(|d| (1..=4).contains(d)));
    }
}
