//! # Sources
//!
//! Allows an entity to reference which source books it was generated from.
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

use serde::{Deserialize, Serialize};
use strum::Display;

/// Supported Source Books
#[derive(Debug, Deserialize, Display, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum Book {
    /// Player's Handbook
    Phb,
}

/// Trait for any entity in need of citation.
///
/// Makes it easer for users to find more information in the source books.
pub trait Sources {
    /// Return a list of source books for the entity.
    fn sources(&self) -> &[Book];
}
