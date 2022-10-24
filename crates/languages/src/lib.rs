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

use std::collections::HashSet;

use derive_more::Deref;
use rand::Rng;
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
    fn weight(self) -> i32 {
        match self {
            Self::Exotic => 0,
            Self::Standard => 2,
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
    fn language_type(self) -> LanguageType {
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

    /// Weighting for language based on type and influences
    fn weight(self, likely_languages: &[Language]) -> i32 {
        self.language_type().weight()
            * i32::try_from(likely_languages.iter().filter(|&ll| ll == &self).count()).unwrap()
    }
}

/// Set of languages known by a character. All characters know Common
#[derive(Debug, Deref, Serialize)]
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
        let language = *self
            .remaining_languages()
            .choose_exp_weighted(rng, |l| l.weight(likely_languages))
            .unwrap();
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
        let choices = self.remaining_languages();
        let languages = choices
            .choose_multiple_exp_weighted(rng, amount, |l| l.weight(likely_languages))
            .unwrap();
        self.0.extend(languages);
    }
}

impl Default for Languages {
    fn default() -> Self {
        Self(HashSet::from([Language::Common]))
    }
}
