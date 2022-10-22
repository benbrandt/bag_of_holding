//! # Alignments
//!
//! All creatures have alignments of some kind. So figure out yours!
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

use std::{borrow::Cow, fmt, str};

use rand::{seq::SliceRandom, Rng};
use rand_utils::exp_weight;
use serde::Serialize;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq, Serialize)]
/// attitudes toward society and order
pub enum Attitude {
    /// little regard for what others expect, creatures follow their whims
    Chaotic,
    /// Do what is expected by society, law, tradition, personal codes, loyalty, or order
    Lawful,
    /// According to their needs, steer clear of moral questions and don’t take sides, doing what seems best at the time
    Neutral,
}

impl Attitude {
    /// Weight in comparison to itself. Each has 4 points to spend
    fn individual_weight(self, influence: Self) -> i32 {
        match self {
            Self::Chaotic => match influence {
                Self::Chaotic => 3,
                Self::Neutral => 1,
                Self::Lawful => 0,
            },
            Self::Lawful => match influence {
                Self::Chaotic => 0,
                Self::Neutral => 1,
                Self::Lawful => 3,
            },
            Self::Neutral => match influence {
                Self::Neutral => 2,
                Self::Lawful | Self::Chaotic => 1,
            },
        }
    }

    fn weight(self, influences: &[Self]) -> f64 {
        exp_weight(
            influences
                .iter()
                .map(|&i| self.individual_weight(i))
                .sum::<i32>(),
        )
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, PartialEq, Serialize)]
/// Character's morality, view toward good and bad
pub enum Morality {
    /// methodically take what they want, do whatever they can get away with, or act with arbitrary violence, spurred by their greed, hatred, or bloodlust
    Evil,
    /// counted on to do the right thing, counted on to do the right thing, creatures act as their conscience directs
    Good,
    /// the alignment of those who prefer to steer clear of moral questions and don’t take sides, doing what seems best at the time, holding their personal freedom above all else
    Neutral,
}

impl Morality {
    /// Weight in comparison to itself. Each has 4 points to spend
    fn individual_weight(self, influence: Self) -> i32 {
        match self {
            Self::Evil => match influence {
                Self::Evil => 3,
                Self::Neutral => 1,
                Self::Good => 0,
            },
            Self::Good => match influence {
                Self::Evil => 0,
                Self::Neutral => 1,
                Self::Good => 3,
            },
            Self::Neutral => match influence {
                Self::Neutral => 2,
                Self::Good | Self::Evil => 1,
            },
        }
    }

    fn weight(self, influences: &[Self]) -> f64 {
        exp_weight(
            influences
                .iter()
                .map(|&i| self.individual_weight(i))
                .sum::<i32>(),
        )
    }
}

/// A typical creature in the game world has an alignment, which broadly
/// describes its moral and personal attitudes. Alignment is a combination of
/// two factors: one identifies morality (good, evil, or neutral), and the
/// other describes attitudes toward society and order (lawful, chaotic, or
/// neutral). Thus, nine distinct alignments define the possible combinations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(into = "String")]
pub struct Alignment {
    /// lawful, chaotic, or neutral
    attitude: Attitude,
    /// good, evil, or neutral
    morality: Morality,
}

impl Alignment {
    /// Create a new alignment
    #[must_use]
    pub const fn new(attitude: Attitude, morality: Morality) -> Self {
        Self { attitude, morality }
    }

    /// Generate alignment, weighted by influences from other choices on the character sheet
    ///
    /// # Panics
    ///
    /// Will panic if weighting logic is wrong
    #[tracing::instrument(skip(rng))]
    pub fn gen<R: Rng + ?Sized>(
        rng: &mut R,
        attitude_influences: &[Attitude],
        morality_influences: &[Morality],
    ) -> Self {
        let attitude = *Attitude::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |a| a.weight(attitude_influences))
            .unwrap();
        let morality = *Morality::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |a| a.weight(morality_influences))
            .unwrap();

        metrics::increment_counter!(
            "alignments",
            &[
                ("attitude", attitude.to_string()),
                ("morality", morality.to_string())
            ]
        );

        Self { attitude, morality }
    }

    /// Weight of a particular alignment based on influences.
    /// Useful for comparing things like deities.
    #[must_use]
    pub fn weight(
        &self,
        attitude_influences: &[Attitude],
        morality_influences: &[Morality],
    ) -> f64 {
        self.attitude.weight(attitude_influences) + self.morality.weight(morality_influences)
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self {
                attitude: Attitude::Neutral,
                morality: Morality::Neutral,
            } => write!(f, "Neutral"),
            Self { attitude, morality } => write!(f, "{attitude} {morality}"),
        }
    }
}

impl From<Alignment> for String {
    fn from(alignment: Alignment) -> Self {
        alignment.to_string()
    }
}

/// Trait to describe how this entity affects a character's alignment
pub trait AlignmentInfluences {
    /// List of attitude influences
    fn attitude(&self) -> Cow<'_, [Attitude]> {
        Cow::Borrowed(&[])
    }

    /// List of morality influences
    fn morality(&self) -> Cow<'_, [Morality]> {
        Cow::Borrowed(&[])
    }
}

impl AlignmentInfluences for Alignment {
    fn attitude(&self) -> Cow<'_, [Attitude]> {
        Cow::Owned(vec![self.attitude])
    }

    fn morality(&self) -> Cow<'_, [Morality]> {
        Cow::Owned(vec![self.morality])
    }
}
