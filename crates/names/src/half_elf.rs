use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::{elf::Elf, human::Human, NameGenerator};

/// Half-elves use either human or elven naming conventions. As if to emphasize
/// that they don’t really fit in to either society, half-elves raised among
/// humans are often given elven names, and those raised among elves often take
/// human names.
#[derive(Debug)]
pub struct HalfElf {
    /// Given name
    first_name: &'static str,
    /// Last name
    surname: Option<&'static str>,
}

impl fmt::Display for HalfElf {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.first_name)?;
        if let Some(surname) = self.surname {
            write!(f, " {}", surname)?;
        }
        Ok(())
    }
}

impl Distribution<HalfElf> for Standard {
    /// Generate a new half elf name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HalfElf {
        let human = rng.gen::<Human>();
        let elf = rng.gen::<Elf>();

        HalfElf {
            first_name: *[human.first_name, elf.adult_name].choose(rng).unwrap(),
            surname: *[human.surname, Some(elf.family_name)].choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for HalfElf {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: HalfElf = rand_utils::rng_from_entropy().gen();
        assert!(!name.first_name.is_empty());
        if let Some(surname) = name.surname {
            assert!(!surname.is_empty());
        }
        // Formats full name
        assert_eq!(
            name.to_string(),
            format!("{} {}", name.first_name, name.surname.unwrap_or_default()).trim()
        );
    }
}
