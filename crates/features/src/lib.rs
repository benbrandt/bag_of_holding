//! # Features
//!
//! Features and traits from race, class, background, etc
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

use std::{borrow::Cow, fmt};

use serde::Serialize;
use sources::{Book, Sources};

/// A feature or trait a character has.
#[derive(Clone, Copy, Debug)]
pub struct Feature {
    /// Name of the feature or trait.
    title: &'static str,
    /// Citation for where more information about this feature is available.
    source: Book,
}

impl Feature {
    /// Generate a new feature with a title and source
    #[must_use]
    pub const fn new(title: &'static str, source: Book) -> Self {
        Self { title, source }
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.citation())
    }
}

impl Sources for Feature {
    fn sources(&self) -> Cow<'_, [Book]> {
        Cow::Owned(vec![self.source])
    }
}

/// Features or Traits this adds to the character
pub trait Features {
    /// List of features for this entity
    fn features(&self) -> &[Feature] {
        &[]
    }
}
