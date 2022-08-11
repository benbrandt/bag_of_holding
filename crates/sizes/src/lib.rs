//! # Sizes
//!
//! Everything to do with the size of player characters.
//! Whether it be size for combat mechanics, or generating height and weight.
//!
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
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Size of a given character or monster
#[derive(Copy, Clone, Debug, Deserialize, Display, Serialize)]
pub enum Size {
    /// Space: 2 1/2 by 2 1/2 ft. Example: Imp, sprite
    Tiny,
    /// Space: 5 by 5 ft. Example: Giant rat, goblin
    Small,
    /// Space: 5 by 5 ft. Example: Orc, werewolf
    Medium,
    /// Space: 10 by 10 ft. Example: Hippogriff, ogre
    Large,
    /// Space: 15 by 15 ft. Example: Fire giant, treant
    Huge,
    /// Space: 20 by 20 ft. or larger. Example: Kraken, purple worm
    Gargantuan,
}

/// Generated height and weight values for a creature.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct HeightAndWeight {
    /// Generated height of creature, in inches
    pub height: u8,
    /// Generated weight of creature, in pounds
    pub weight: u16,
}

/// Generate a height and weight values for various different creature types
///
/// ```
/// use rand::Rng;
/// use sizes::{HeightAndWeight, HeightAndWeightTable};
///
/// let HeightAndWeight { height, weight } = HeightAndWeightTable::Dragonborn.gen(&mut rand::thread_rng());
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Display, EnumIter, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum HeightAndWeightTable {
    /// Generate from a range of valid Dragonborn sizes.
    Dragonborn,
}

impl HeightAndWeightTable {
    /// Convert feet + inches to just inches
    #[tracing::instrument]
    fn in_inches(feet: u8, inches: u8) -> u8 {
        feet * 12 + inches
    }

    /// Base height to use for height calculations
    #[tracing::instrument]
    fn base_height(self) -> u8 {
        match self {
            Self::Dragonborn => Self::in_inches(5, 6),
        }
    }

    /// Variable modifier to use in height calculations
    #[tracing::instrument]
    fn height_modifier(self) -> Roll {
        match self {
            Self::Dragonborn => Roll::new(2, Die::D8),
        }
    }

    /// Base weight to use for weight calculations
    #[tracing::instrument]
    fn base_weight(self) -> u16 {
        match self {
            Self::Dragonborn => 175,
        }
    }

    /// Variable modifier to use in weight calculations
    #[tracing::instrument]
    fn weight_modifier(self) -> WeightMod {
        match self {
            Self::Dragonborn => WeightMod::Roll(Roll::new(2, Die::D6)),
        }
    }

    /// Generate valid height (in inches) and weight (in pounds) for a given creature.
    ///
    /// ```
    /// use rand::Rng;
    /// use sizes::{HeightAndWeight, HeightAndWeightTable};
    ///
    /// let HeightAndWeight { height, weight } = HeightAndWeightTable::Dragonborn.gen(&mut rand::thread_rng());
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn gen<R: Rng + ?Sized>(self, rng: &mut R) -> HeightAndWeight {
        let height = self.base_height() + self.height_modifier().gen(rng).sum::<u8>();

        // Weight modifier is multiplied by height
        let weight_mod: u16 = match self.weight_modifier() {
            WeightMod::_Fixed(f) => f,
            WeightMod::Roll(r) => u16::from(r.gen(rng).sum::<u8>()),
        } * u16::from(height);

        HeightAndWeight {
            height,
            weight: self.base_weight() + weight_mod,
        }
    }
}

/// Weight modifier
#[derive(Copy, Clone)]
enum WeightMod {
    /// Fixed additional weight
    _Fixed(u16),
    /// Role to find additional weight
    Roll(Roll),
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;

    use super::*;

    impl WeightMod {
        /// Minimum valid weight modifier
        fn min(self) -> usize {
            match self {
                Self::_Fixed(f) => usize::from(f),
                Self::Roll(r) => r.min(),
            }
        }

        /// Maximum valid weight modifier
        pub fn max(self) -> usize {
            match self {
                Self::_Fixed(f) => usize::from(f),
                Self::Roll(r) => r.max(),
            }
        }
    }

    #[test]
    fn valid_generator() {
        for table in HeightAndWeightTable::iter() {
            let base_h = usize::from(table.base_height());
            let base_w = usize::from(table.base_weight());
            let h_mod = table.height_modifier();
            let w_mod = table.weight_modifier();

            let HeightAndWeight { height, weight } = table.gen(&mut rand_utils::rng_from_entropy());

            let min_h = base_h + h_mod.min();
            let max_h = base_h + h_mod.max();

            let min_w = base_w + (min_h * w_mod.min());
            let max_w = base_w + (max_h * w_mod.max());

            assert!((min_h..=max_h).contains(&height.into()));
            assert!((min_w..=max_w).contains(&weight.into()));
        }
    }
}
