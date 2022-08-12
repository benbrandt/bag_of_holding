//! # Damage
//!
//! Different damage times, and character resistance and immunity to them.
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

use serde::{Deserialize, Serialize};
use strum::Display;

/// Different attacks, damaging spells, and other harmful effects deal
/// different types of damage. Damage types have no rules of their own, but
/// other rules, such as damage resistance, rely on the types.
#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, PartialEq, Serialize)]
pub enum DamageType {
    /// The corrosive spray of an adult black dragon’s breath and the dissolving enzymes secreted by a black pudding deal acid damage.
    Acid,
    /// Blunt force attacks — hammers, falling, constriction, and the like — deal bludgeoning damage.
    Bludgeoning,
    /// The infernal chill radiating from an ice devil’s spear and the frigid blast of a young white dragon’s breath deal cold damage.
    Cold,
    /// Ancient red dragons breathe fire, and many spells conjure flames to deal fire damage.
    Fire,
    /// Force is pure magical energy focused into a damaging form. Most effects that deal force damage are spells, including magic missile and spiritual weapon.
    Force,
    /// A lightning bolt spell and a blue dragon wyrmling’s breath deal lightning damage.
    Lightning,
    /// Necrotic damage, dealt by certain undead and a spell such as chill touch, withers matter and even the soul.
    Necrotic,
    /// Puncturing and impaling attacks, including spears and monsters’ bites, deal piercing damage.
    Piercing,
    /// Venomous stings and the toxic gas of an adult green dragon’s breath deal poison damage.
    Poison,
    /// Mental abilities such as a psionic blast deal psychic damage.
    Psychic,
    /// Radiant damage, dealt by a cleric’s flame strike spell or an angel’s smiting weapon, sears the flesh like fire and overloads the spirit with power.
    Radiant,
    ///  Swords, axes, and monsters’ claws deal slashing damage.
    Slashing,
    /// A concussive burst of sound, such as the effect of the thunderwave spell, deals thunder damage.
    Thunder,
}

/// Trait to encapuslate resistances
pub trait Resistances {
    /// Return list of resistances for this entity
    fn resistances(&self) -> Cow<'_, [DamageType]> {
        Cow::Borrowed(&[])
    }
}
