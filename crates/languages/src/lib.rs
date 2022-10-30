//! # Languages
//!
//! Languages a character or creatures can speak and understand
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

use std::{borrow::Cow, collections::HashSet};

use deities::{Deities, Pantheon};
use derive_more::Deref;
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use rand_utils::SliceExpRandom;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

/// How likely it is for a character to know a language
#[derive(Clone, Copy, Debug)]
enum LanguageType {
    /// Less common, and therefore less likely to know
    Exotic,
    /// Fairly common languages throughout the multiverse
    Standard,
}

impl LanguageType {
    /// Weighting for language likelihood
    const fn weight(self) -> i32 {
        match self {
            Self::Exotic => 0,
            Self::Standard => 3,
        }
    }
}

/// Languages as character can learn an speak
#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, Hash, PartialEq, Serialize)]
pub enum Language {
    /// Typical Speakers: Humans
    /// Script: Common
    Common,
    /// Typical Speakers: Dwarves
    /// Script: Dwarvish
    Dwarvish,
    /// Typical Speakers: Elves
    /// Script: Elvish
    Elvish,
    /// Typical Speakers: Ogres, Giants
    /// Script: Dwarvish
    Giant,
    /// Typical Speakers: Gnomes
    /// Script: Dwarvish
    Gnomish,
    /// Typical Speakers: Goblinoids
    /// Script: Dwarvish
    Goblin,
    /// Typical Speakers: Halflings
    /// Script: Common
    Halfling,
    /// Typical Speakers: Orcs
    /// Script: Dwarvish
    Orc,
    /// Typical Speakers: Demons
    /// Script: Infernal
    Abyssal,
    /// Typical Speakers: Celestials
    /// Script: Celestial
    Celestial,
    /// Typical Speakers: Dragons, Dragonborn
    /// Script: Draconic
    Draconic,
    /// Typical Speakers: Mind flayers, Beholders
    /// Script: -
    #[serde(rename = "Deep Speech")]
    DeepSpeech,
    /// Typical Speakers: Devils
    /// Script: Infernal
    Infernal,
    /// Typical Speakers: Elementals
    /// Script: Dwarvish
    Primordial,
    /// Typical Speakers: Fey creatures
    /// Script: Elvish
    Sylvan,
    /// Typical Speakers: Underworld traders
    /// Script: Elvish
    Undercommon,
    /// Typical Speakers: Rogues
    /// Script: -
    #[serde(rename = "Thieves' Cant")]
    ThievesCant,
    /// Typical Speakers: Druids
    /// Script: -
    #[serde(rename = "Tongue of Druids")]
    TongueOfDruids,
}

impl Language {
    /// Language type for each language (used for weighting)
    const fn language_type(self) -> LanguageType {
        match self {
            Self::Common
            | Self::Dwarvish
            | Self::Elvish
            | Self::Giant
            | Self::Gnomish
            | Self::Goblin
            | Self::Halfling
            | Self::Orc => LanguageType::Standard,
            Self::Abyssal
            | Self::Celestial
            | Self::Draconic
            | Self::DeepSpeech
            | Self::Infernal
            | Self::Primordial
            | Self::Sylvan
            | Self::Undercommon
            | Self::ThievesCant
            | Self::TongueOfDruids => LanguageType::Exotic,
        }
    }

    /// Weighting for language based on type
    fn weight(self) -> i32 {
        self.language_type().weight()
    }
}

impl Deities for Language {
    fn pantheons(&self) -> Cow<'_, [Pantheon]> {
        Cow::Borrowed(match self {
            Self::Dwarvish => &[Pantheon::Dwarven],
            Self::Elvish | Self::Sylvan => &[Pantheon::Elven],
            Self::Giant => &[Pantheon::Giant],
            Self::Gnomish => &[Pantheon::Gnomish],
            Self::Goblin => &[Pantheon::Bugbear, Pantheon::Goblin],
            Self::Halfling => &[Pantheon::Halfling],
            Self::Orc => &[Pantheon::Orc],
            Self::Draconic => &[Pantheon::Dragon, Pantheon::Kobold, Pantheon::Lizardfolk],
            Self::Undercommon => &[Pantheon::Drow, Pantheon::Duergar],
            Self::Abyssal
            | Self::Common
            | Self::Celestial
            | Self::DeepSpeech
            | Self::Infernal
            | Self::Primordial
            | Self::ThievesCant
            | Self::TongueOfDruids => &[],
        })
    }
}

/// Set of languages known by a character. All characters know Common
#[derive(Clone, Debug, Deref, Serialize)]
#[serde(transparent)]
pub struct Languages(HashSet<Language>);

impl Languages {
    /// New set of languages. Starts with Common
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Remaining language options
    fn remaining_languages(&self) -> Vec<Language> {
        Language::iter()
            .collect::<HashSet<Language>>()
            .difference(&self.0)
            .copied()
            .collect()
    }

    /// Choose a new language for the character, based on influences
    ///
    /// # Panics
    ///
    /// If not enough languages to choose from
    pub fn choose<R: Rng + ?Sized>(&mut self, rng: &mut R, likely_languages: &[Language]) {
        let remaining = self.remaining_languages();
        if remaining.is_empty() {
            return;
        }
        let remaining_likely = likely_languages
            .iter()
            .filter(|&l| remaining.contains(l))
            .collect::<Vec<_>>();
        // 10% chance we ignore the likely languages
        let language = if remaining_likely.is_empty() || (1..=10).choose(rng).unwrap() == 10 {
            *remaining.choose_exp_weighted(rng, |l| l.weight()).unwrap()
        } else {
            **remaining_likely.choose(rng).unwrap()
        };
        self.0.insert(language);
    }

    /// Choose new languages for the character, based on influences
    ///
    /// # Panics
    ///
    /// If not enough languages to choose from
    pub fn choose_multiple<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        amount: usize,
        likely_languages: &[Language],
    ) {
        for _ in 0..amount {
            self.choose(rng, likely_languages);
        }
    }
}

impl Default for Languages {
    fn default() -> Self {
        Self(HashSet::from([Language::Common]))
    }
}

impl Deities for Languages {
    fn pantheons(&self) -> Cow<'_, [Pantheon]> {
        Cow::Owned(
            self.iter()
                .flat_map(|l| l.pantheons().to_vec())
                .collect::<Vec<_>>(),
        )
    }
}

/// Define how this entity influences the character's languages
pub trait LanguageOptions {
    /// How many additional languages does this provide the character
    fn additional_languages(&self) -> usize {
        0
    }

    /// Which languages the character is likely to choose from
    fn likely_languages(&self) -> Cow<'_, [Language]> {
        Cow::Borrowed(&[])
    }
}
