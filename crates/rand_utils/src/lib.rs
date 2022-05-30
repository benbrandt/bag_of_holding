//! # `rand_utils`
//!
//! Shared random utilities and logic
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

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

/// Creates a new instance of the RNG seeded via getrandom.
/// Consolidates choice of random number generator algorithm to a single place.
///
/// Currently uses the Pcg64 algorithm for its statistical properties in the
/// use cases present in the neighboring crates. Ideal to get better
/// statistical randomness and crypto needs are not required.
///
/// ```
/// use rand::Rng;
///
/// let mut rng = rand_utils::rng_from_entropy();
/// let x: u32 = rng.gen();
/// ```
#[tracing::instrument]
#[must_use]
pub fn rng_from_entropy() -> impl Rng {
    Pcg64::from_entropy()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_generate_random_number() {
        let mut rng = rng_from_entropy();
        let x: u8 = rng.gen();
        assert!((0..255).contains(&x));
    }
}
