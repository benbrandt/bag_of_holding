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

use deities::{Deity, Pantheon};
use strum::IntoEnumIterator;

#[test]
fn all_deities_match_their_pantheon() {
    for pantheon in Pantheon::iter() {
        for deity in pantheon.deities() {
            assert_eq!(deity.pantheon, pantheon);
        }
    }
}

#[test]
fn gen_pantheon() {
    let mut rng = rand_utils::rng_from_entropy();
    let _ = Pantheon::gen(&mut rng, &[], &[], &[]);
}

#[test]
fn gen_deity_if_required() {
    let mut rng = rand_utils::rng_from_entropy();
    let deity = Deity::gen(&mut rng, &[], &[], &[], true);
    assert!(deity.is_some());
}

#[test]
fn dont_gen_deity_if_not_required() {
    let mut rng = rand_utils::rng_from_entropy();
    let mut deity = None;
    for _ in 0..1000 {
        deity = Deity::gen(&mut rng, &[], &[], &[], false);
        if deity.is_none() {
            break;
        }
    }
    assert!(deity.is_none());
}
