//! # Character Descriptions
//!
//! Things to know about a character, such as personality, backstory, appearance, etc.
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

use std::borrow::Cow;

/// Trait for any entity that provides some description of the character's
/// appearance.
pub trait Appearance {
    /// Elements of the characters appearance provided by this entity
    fn appearance(&self) -> Cow<'_, [&'_ str]> {
        Cow::Borrowed(&[])
    }
}

/// Trait for any entity that provides some character backstory.
pub trait Backstory {
    /// Elements of the characters backstory provided by this entity
    fn backstory(&self) -> Cow<'_, [&'_ str]> {
        Cow::Borrowed(&[])
    }
}
