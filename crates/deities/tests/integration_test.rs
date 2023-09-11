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

use deities::{Deity, Domain, Pantheon};
use rand::Rng;
use strum::IntoEnumIterator;

#[test]
fn all_deities_match_their_pantheon() {
    for pantheon in Pantheon::iter() {
        for deity in &*pantheon.deities(None) {
            assert_eq!(deity.pantheon, pantheon);
        }
    }
}

#[test]
fn gen_pantheon() {
    let mut rng = rand_utils::rng_from_entropy();
    let _ = Pantheon::gen(&mut rng, None, &[], &[], &[]);
}

#[test]
fn gen_deity_if_required() {
    let mut rng = rand_utils::rng_from_entropy();
    let deity = Deity::gen(&mut rng, None, &[], &[], &[], true);
    assert!(deity.is_some());
}

#[test]
fn dont_gen_deity_if_not_required() {
    let mut rng = rand_utils::rng_from_entropy();
    let mut deity = None;
    for _ in 0..1000 {
        deity = Deity::gen(&mut rng, None, &[], &[], &[], false);
        if deity.is_none() {
            break;
        }
    }
    assert!(deity.is_none());
}

#[test]
fn domain_matches_if_selected() {
    let mut rng = rand_utils::rng_from_entropy();
    for _ in 0..10 {
        let domain = rng.gen::<Domain>();
        let deity = Deity::gen(&mut rng, Some(domain), &[], &[], &[], true).unwrap();

        assert!(deity.domains.contains(&domain));
    }
}
