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

use races::{Race, RaceGenerator, RaceOption};
use rand::Rng;
use sources::Sources;
use strum::IntoEnumIterator;

#[test]
fn generate_random_race() {
    let mut rng = rand_utils::rng_from_entropy();
    let race = rng.gen::<Race>();

    let name = race.gen_name(&mut rng);
    assert!(!name.is_empty());
}

#[test]
fn can_generate_all_races() {
    let mut rng = rand_utils::rng_from_entropy();
    for race in RaceOption::iter() {
        let race = race.gen(&mut rng);

        let name = race.gen_name(&mut rng);
        assert!(!name.is_empty());
        let sources = race.sources();
        assert!(!sources.is_empty());

        assert!(!race.ability_increases().is_empty());

        let citation = race.citation();
        assert!(citation.contains(&race.to_string()));

        for source in sources {
            assert!(citation.contains(&source.to_string()));
        }

        let age = race.gen_age(&mut rng);
        assert!(race.age_range().contains(&age));
    }
}
