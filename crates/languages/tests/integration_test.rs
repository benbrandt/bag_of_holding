use languages::{Language, Languages};
use strum::IntoEnumIterator;

#[test]
fn should_always_have_common() {
    let languages = Languages::new();
    assert!(languages.contains(&Language::Common));
}

#[test]
fn should_have_two_after_choosing_one() {
    let mut rng = rand_utils::rng_from_entropy();
    let mut languages = Languages::new();
    languages.choose(&mut rng, &[]);
    assert_eq!(languages.len(), 2);
}

#[test]
fn should_have_three_after_choosing_two() {
    let mut rng = rand_utils::rng_from_entropy();
    let mut languages = Languages::new();
    languages.choose_multiple(&mut rng, 2, &[]);
    assert_eq!(languages.len(), 3);
}

#[test]
fn should_stop_at_max() {
    let mut rng = rand_utils::rng_from_entropy();
    let lang_count = Language::iter().count();
    let mut languages = Languages::new();
    languages.choose_multiple(&mut rng, lang_count + 1, &[]);
    assert_eq!(languages.len(), lang_count);
}
