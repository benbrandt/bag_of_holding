//! # Speeds
//!
//! A way to capture speed information for any entity that affects character speed.
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

use std::fmt;

use serde::{Deserialize, Serialize};

/// Speed type, along with how much it increases the character's speed for that
/// type, in feet.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Speed {
    /// How far a creature can climb in a turn
    Climbing(u8),
    /// How far a creature can fly in a turn
    Flying(u8),
    /// How far a creature can swim in a turn
    Swimming(u8),
    /// How far a creature can walk in a turn
    Walking(u8),
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Climbing(s) => write!(f, "Climbing Speed: {s}ft"),
            Self::Flying(s) => write!(f, "Flying Speed: {s}ft"),
            Self::Swimming(s) => write!(f, "Swimming Speed: {s}ft"),
            Self::Walking(s) => write!(f, "Walking Speed: {s}ft"),
        }
    }
}

/// Trait for any entity that affect character speeds
pub trait Speeds: fmt::Display {
    /// Return a list of source books for the entity.
    fn speeds(&self) -> &[Speed];
}
