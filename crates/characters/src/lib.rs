//! # Characters
//!
//! Crate to generate entire characters. Assembles together all of the other
//! crates together into a final character sheet.
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

use std::borrow::Cow;

use abilities::AbilityScores;
use alignments::{Alignment, AlignmentInfluences};
use deities::{Deities, Deity, Pantheon};
use descriptions::{Appearance, Backstory};
use races::{Race, RaceGenerator};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::Serialize;
use sizes::{HeightAndWeight, Size};
use sources::Sources;
use thiserror::Error;

/// Full character information.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(into = "CharacterSheet")]
pub struct Character {
    /// Ability scores of the character
    pub ability_scores: Option<AbilityScores>,
    /// The character's age
    pub age: Option<u16>,
    /// The character's alignment
    pub alignment: Option<Alignment>,
    /// The character's favored deity
    pub deity: Option<Deity>,
    /// The character's height and weight
    pub height_and_weight: Option<HeightAndWeight>,
    /// The character's name
    pub name: String,
    /// Race of the character
    pub race: Option<Race>,
}

impl Character {
    /// Creates a new default [`Character`].
    ///
    /// ```
    /// use characters::Character;
    ///
    /// let character = Character::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate and add base ability scores for your character.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new().gen_ability_scores(&mut rng);
    /// ```
    #[must_use]
    #[tracing::instrument(skip(rng))]
    pub fn gen_ability_scores<R: Rng + ?Sized>(mut self, rng: &mut R) -> Self {
        self.ability_scores = Some(rng.gen::<AbilityScores>());
        self
    }

    /// Helper function to return a reference to the ability scores, otherwise error
    fn try_ability_scores(&mut self) -> Result<&mut AbilityScores, CharacterBuildError> {
        self.ability_scores
            .as_mut()
            .ok_or(CharacterBuildError::MissingAbilityScores)
    }

    /// Generate a race for your character.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new().gen_ability_scores(&mut rng).gen_race(&mut rng);
    /// ```
    /// # Errors
    ///
    /// Will error if ability scores are not already chosen.
    #[tracing::instrument(skip(rng))]
    pub fn gen_race<R: Rng + ?Sized>(mut self, rng: &mut R) -> Result<Self, CharacterBuildError> {
        let race = rng.gen::<Race>();
        // Generate ability score increases
        self.try_ability_scores()?
            .gen_racial_increases(rng, race.ability_increases());
        self.race = Some(race);
        Ok(self)
    }

    /// Helper function to return a reference to the race, otherwise error
    fn try_race(&self) -> Result<&Race, CharacterBuildError> {
        self.race.as_ref().ok_or(CharacterBuildError::MissingRace)
    }

    /// Generate a name for your character.
    ///
    /// Requires a Race to be selected already.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new()
    ///     .gen_ability_scores(&mut rng)
    ///     .gen_race(&mut rng)?
    ///     .gen_name(&mut rng)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    ///
    /// Will error if race is not already chosen.
    #[tracing::instrument(skip(rng))]
    pub fn gen_name<R: Rng + ?Sized>(mut self, rng: &mut R) -> Result<Self, CharacterBuildError> {
        self.name = self.try_race()?.gen_name(rng);
        Ok(self)
    }

    /// Generate an age for your character.
    ///
    /// Requires a Race to be selected already.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new()
    ///     .gen_ability_scores(&mut rng)
    ///     .gen_race(&mut rng)?
    ///     .gen_age(&mut rng)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Will error if race is not already chosen.
    #[tracing::instrument(skip(rng))]
    pub fn gen_age<R: Rng + ?Sized>(mut self, rng: &mut R) -> Result<Self, CharacterBuildError> {
        self.age = Some(self.try_race()?.gen_age(rng));
        Ok(self)
    }

    /// Generate a height and weight for your character.
    ///
    /// Requires a Race to be selected already.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new()
    ///     .gen_ability_scores(&mut rng)
    ///     .gen_race(&mut rng)?
    ///     .gen_height_and_weight(&mut rng)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Will error if race is not already chosen.
    #[tracing::instrument(skip(rng))]
    pub fn gen_height_and_weight<R: Rng + ?Sized>(
        mut self,
        rng: &mut R,
    ) -> Result<Self, CharacterBuildError> {
        self.height_and_weight = Some(self.try_race()?.gen_height_and_weight(rng));
        Ok(self)
    }

    /// Helper for getting all pantheons the character might choose
    ///
    /// # Errors
    ///
    /// Will error if race isn't already chosen
    fn pantheons(&self) -> Result<Vec<Pantheon>, CharacterBuildError> {
        let mut pantheons = vec![Pantheon::ForgottenRealms];
        pantheons.extend(self.try_race()?.pantheons().iter());

        Ok(pantheons)
    }

    /// Helper for whether or not character requires a deity
    ///
    /// # Errors
    ///
    /// Will error if race isn't already chosen
    fn deity_required(&self) -> Result<bool, CharacterBuildError> {
        Ok(self.try_race()?.deity_required())
    }

    /// Generate an deity for your character.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new()
    ///     .gen_ability_scores(&mut rng)
    ///     .gen_race(&mut rng)?
    ///     .gen_deity(&mut rng)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn gen_deity<R: Rng + ?Sized>(mut self, rng: &mut R) -> Result<Self, CharacterBuildError> {
        self.deity = Deity::gen(
            rng,
            None,
            &self.pantheons()?,
            &self.attitude(),
            &self.morality(),
            self.deity_required()?,
        );
        Ok(self)
    }

    /// Generate an alignment for your character.
    ///
    /// ```
    /// use characters::Character;
    /// use rand::Rng;
    ///
    /// let mut rng = rand::thread_rng();
    /// let character = Character::new()
    ///     .gen_alignment(&mut rng);
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn gen_alignment<R: Rng + ?Sized>(mut self, rng: &mut R) -> Self {
        self.alignment = Some(Alignment::gen(rng, &self.attitude(), &self.morality()));
        self
    }

    /// Helper method to generate a full character in the right order with a result.
    #[tracing::instrument(skip(rng))]
    fn gen<R: Rng + ?Sized>(rng: &mut R) -> Result<Self, CharacterBuildError> {
        Ok(Character::new()
            .gen_ability_scores(rng)
            .gen_race(rng)?
            .gen_name(rng)?
            .gen_age(rng)?
            .gen_height_and_weight(rng)?
            .gen_deity(rng)?
            .gen_alignment(rng))
    }
}

impl AlignmentInfluences for Character {
    fn attitude(&self) -> Cow<'_, [alignments::Attitude]> {
        self.deity
            .as_ref()
            .map(alignments::AlignmentInfluences::attitude)
            .unwrap_or_default()
    }

    fn morality(&self) -> Cow<'_, [alignments::Morality]> {
        self.deity
            .as_ref()
            .map(alignments::AlignmentInfluences::morality)
            .unwrap_or_default()
    }
}

impl Appearance for Character {
    fn appearance(&self) -> Cow<'_, [&'_ str]> {
        self.race
            .as_ref()
            .map(Appearance::appearance)
            .unwrap_or_default()
    }
}

impl Backstory for Character {
    fn backstory(&self) -> Cow<'_, [&'_ str]> {
        self.race
            .as_ref()
            .map(Backstory::backstory)
            .unwrap_or_default()
    }
}

impl Distribution<Character> for Standard {
    /// Generate a fully random character.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Character {
        // Not a result because we should be calling these steps in the right order
        Character::gen(rng).unwrap()
    }
}

/// Errors caused by an invalid character build
#[derive(Debug, Eq, Error, PartialEq)]
pub enum CharacterBuildError {
    /// Error produced when ability scores are required to make a decision
    #[error("This character is missing ability scores. Please choose or generate ability scores before this step.")]
    MissingAbilityScores,
    /// Error produced when a race is required to make a decision
    #[error("This character is missing a race. Please choose or generate a race option before this step.")]
    MissingRace,
}

/// Serializable, public interface for a character
#[derive(Serialize)]
struct CharacterSheet {
    /// Ability scores of the character
    pub ability_scores: Option<AbilityScores>,
    /// The character's age
    pub age: Option<u16>,
    /// The character's alignment
    pub alignment: Option<Alignment>,
    /// The character's favored deity
    pub deity: Option<Deity>,
    /// The character's height and weight
    #[serde(flatten)]
    pub height_and_weight: Option<HeightAndWeight>,
    /// Name of the character
    pub name: String,
    /// Chosen race of the character
    pub race: Option<String>,
    /// The character's size
    pub size: Option<Size>,
}

impl From<Character> for CharacterSheet {
    fn from(character: Character) -> Self {
        Self {
            ability_scores: character.ability_scores,
            age: character.age,
            alignment: character.alignment,
            deity: character.deity,
            height_and_weight: character.height_and_weight,
            name: character.name,
            race: character.race.as_ref().map(Sources::citation),
            size: character.race.as_ref().map(RaceGenerator::size),
        }
    }
}
