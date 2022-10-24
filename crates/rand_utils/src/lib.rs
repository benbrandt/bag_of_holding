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

use std::{
    f64::consts::E,
    ops::{AddAssign, Sub},
};

use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform},
        WeightedError,
    },
    seq::{SliceChooseIter, SliceRandom},
    Rng, SeedableRng,
};
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

/// Create an exponential based weight from a value.
/// Works well for `choose_exp_weighted*` options.
fn exp_weight<V>(val: V) -> f64
where
    i32: TryFrom<V>,
{
    E.powi(i32::try_from(val).unwrap_or_default())
}

/// Choose values from a slice based on exponential weights.
/// Will adjust weights by offset of the minimum score so that the minimum
/// weight is always 1.

/// Extension trait on slices, providing exponential based sampling.
///
/// This trait is implemented on all `[T]` slice types, providing several
/// methods for choosing elements. You must `use` this trait:
///
/// ```
/// use rand_utils::SliceExpRandom;
///
/// let mut rng = rand_utils::rng_from_entropy();
/// let choices = [('a', 2), ('b', 1), ('c', 1)];
/// // 58% chance to print 'a', 21% chance to print 'b', 21% chance to print 'c'
/// println!("{:?}", choices.choose_exp_weighted(&mut rng, |item| item.1).unwrap().0);
/// ```
pub trait SliceExpRandom: SliceRandom {
    /// Similar to [`rand::seq::SliceRandom::choose_weighted`], but applying
    /// `E.pow(weight)` to each weight.
    ///
    /// Also scales the weight so that the smallest weight because `E.pow(0)`,
    /// offsetting the other weights by the same amount before applying the
    /// exponent.
    /// ```
    /// use rand_utils::SliceExpRandom;
    ///
    /// let mut rng = rand_utils::rng_from_entropy();
    /// let choices = [('a', 2), ('b', 1), ('c', 0)];
    /// // 73% chance to print 'a', 27% chance to print 'b', 10% chance to print 'c'
    /// println!("{:?}", choices.choose_exp_weighted(&mut rng, |item| item.1).unwrap().0);
    /// ```
    /// # Errors
    ///
    /// Errors if weights supplied are not valid
    fn choose_exp_weighted<R, F, B, X>(
        &self,
        rng: &mut R,
        weight: F,
    ) -> Result<&Self::Item, WeightedError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Item) -> B,
        B: SampleBorrow<X> + Copy + Default + Ord + Sub<Output = B>,
        X: SampleUniform + for<'a> AddAssign<&'a X> + PartialOrd<X> + Clone + Default,
        i32: TryFrom<B>;

    /// Similar to [`rand::seq::SliceRandom::choose_multiple_weighted`], but
    /// applying `E.pow(weight)` to each weight. The elements are returned in an
    /// arbitrary, unspecified order.
    ///
    /// If all of the weights are equal, even if they are all zero, each element has
    /// an equal likelihood of being selected.
    ///
    /// # Example
    ///
    /// ```
    /// use rand_utils::SliceExpRandom;

    /// let choices = [('a', 2), ('b', 1), ('c', 1)];
    /// let mut rng = rand_utils::rng_from_entropy();
    /// println!("{:?}", choices.choose_multiple_exp_weighted(&mut rng, 2, |item| item.1).unwrap().collect::<Vec<_>>());
    /// ```

    /// # Errors
    ///
    /// Errors if weights supplied are not valid
    fn choose_multiple_exp_weighted<R, F, X>(
        &self,
        rng: &mut R,
        amount: usize,
        weight: F,
    ) -> Result<SliceChooseIter<'_, Self, Self::Item>, WeightedError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Item) -> X,
        X: Into<f64> + Copy + Default + Ord + Sub<Output = X>,
        i32: TryFrom<X>;
}

impl<T> SliceExpRandom for [T] {
    #[allow(clippy::redundant_closure)]
    fn choose_exp_weighted<R, F, B, X>(
        &self,
        rng: &mut R,
        weight: F,
    ) -> Result<&Self::Item, WeightedError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Item) -> B,
        B: SampleBorrow<X> + Copy + Default + Ord + Sub<Output = B>,
        X: SampleUniform + for<'a> AddAssign<&'a X> + PartialOrd<X> + Clone + Default,
        i32: TryFrom<B>,
    {
        let min = self.iter().map(|i| weight(i)).min().unwrap_or_default();
        self.choose_weighted(rng, |i| exp_weight(weight(i) - min))
    }

    #[allow(clippy::redundant_closure)]
    fn choose_multiple_exp_weighted<R, F, X>(
        &self,
        rng: &mut R,
        amount: usize,
        weight: F,
    ) -> Result<SliceChooseIter<'_, Self, Self::Item>, WeightedError>
    where
        R: Rng + ?Sized,
        F: Fn(&Self::Item) -> X,
        X: Into<f64> + Copy + Default + Ord + Sub<Output = X>,
        i32: TryFrom<X>,
    {
        let min = self.iter().map(|i| weight(i)).min().unwrap_or_default();
        self.choose_multiple_weighted(rng, amount, |i| exp_weight(weight(i) - min))
    }
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
