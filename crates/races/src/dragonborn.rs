use std::fmt;

use names::Name;
use rand::{
    distributions::Standard,
    prelude::{Distribution, IteratorRandom},
};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::RaceGenerator;

/// Born of dragons, as their name proclaims, the dragonborn walk proudly
/// through a world that greets them with fearful incomprehension. Shaped
/// by draconic gods or the dragons themselves, dragonborn originally
/// hatched from dragon eggs as a unique race, combining the best
/// attributes of dragons and humanoids. Some dragonborn are faithful
/// servants to true dragons, others form the ranks of soldiers in great
/// wars, and still others find themselves adrift, with no clear calling
/// in life.
#[derive(Debug)]
pub struct Dragonborn {
    /// Your breath weapon and damage resistance are determined by this dragon
    /// type.
    draconic_ancestry: DraconicAncestry,
}

/// Dragon types available for ancestry options
#[derive(Debug, Display, EnumIter)]
enum DraconicAncestry {
    Black,
    Blue,
    Brass,
    Bronze,
    Copper,
    Gold,
    Green,
    Red,
    Silver,
    White,
}

impl RaceGenerator for Dragonborn {
    /// Name generator to use for this race
    #[tracing::instrument]
    fn name_generator(&self) -> Name {
        Name::Dragonborn
    }
}

impl fmt::Display for Dragonborn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Dragonborn", self.draconic_ancestry)
    }
}

impl Distribution<Dragonborn> for Standard {
    /// Generate a random dragonborn
    #[tracing::instrument(skip(rng))]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Dragonborn {
        let draconic_ancestry = DraconicAncestry::iter().choose(rng).unwrap();

        metrics::increment_counter!(
            "races",
            &[(
                "dragonborn_draconic_ancestry",
                draconic_ancestry.to_string()
            )]
        );

        Dragonborn { draconic_ancestry }
    }
}
