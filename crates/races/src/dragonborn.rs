use std::{borrow::Cow, fmt, ops::RangeInclusive};

use deities::{Deities, Pantheon};
use descriptions::{Appearance, Backstory};
use names::Name;
use rand::{
    distributions::Standard,
    prelude::{Distribution, IteratorRandom},
    Rng,
};
use sizes::{HeightAndWeightTable, Size};
use sources::{Book, Sources};
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
#[derive(Copy, Clone, Debug)]
pub struct Dragonborn {
    /// Your breath weapon and damage resistance are determined by this dragon
    /// type.
    draconic_ancestry: DraconicAncestry,
}

/// Dragon types available for ancestry options
#[derive(Copy, Clone, Debug, Display, EnumIter)]
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

impl Appearance for Dragonborn {}

impl Backstory for Dragonborn {}

impl Deities for Dragonborn {
    fn pantheons(&self) -> Cow<'_, [Pantheon]> {
        Cow::Borrowed(&[Pantheon::Dragon])
    }
}

impl RaceGenerator for Dragonborn {
    /// Name generator to use for this race
    fn name_generator(&self) -> Name {
        Name::Dragonborn
    }

    /// Ability score increase available for this race
    fn ability_increases(&self) -> &[u8] {
        &[2, 1]
    }

    /// Generate an age for a character of this race
    fn age_range(&self) -> RangeInclusive<u16> {
        3..=80
    }

    /// Height and weight table to use for this race
    fn height_and_weight_table(&self) -> HeightAndWeightTable {
        HeightAndWeightTable::Dragonborn
    }

    /// Size of this race
    fn size(&self) -> Size {
        Size::Medium
    }
}

impl Sources for Dragonborn {
    fn sources(&self) -> Cow<'_, [Book]> {
        Cow::Borrowed(&[Book::Phb])
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
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dragonborn {
        let draconic_ancestry = DraconicAncestry::iter().choose(rng).unwrap();

        metrics::counter!(
            "races",
            &[(
                "dragonborn_draconic_ancestry",
                draconic_ancestry.to_string()
            )]
        ).increment(1);

        Dragonborn { draconic_ancestry }
    }
}
