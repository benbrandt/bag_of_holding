use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::{human::Human, orc::Orc, NameGenerator};

/// Half-orcs usually have names appropriate to the culture in which they were
/// raised. A half-orc who wants to fit in among humans might trade an orc name
/// for a human name. Some half-orcs with human names decide to adopt a
/// guttural orc name because they think it makes them more intimidating.
#[derive(Debug)]
pub struct HalfOrc {
    /// Given name
    first_name: &'static str,
    /// Last name
    surname: Option<&'static str>,
}

impl fmt::Display for HalfOrc {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.first_name)?;
        if let Some(surname) = self.surname {
            write!(f, " {}", surname)?;
        }
        Ok(())
    }
}

impl Distribution<HalfOrc> for Standard {
    /// Generate a new half-orc name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HalfOrc {
        let human = rng.gen::<Human>();
        let orc = rng.gen::<Orc>();

        HalfOrc {
            first_name: [human.first_name, orc.name].choose(rng).unwrap(),
            surname: *[human.surname, Some(orc.epithet)].choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for HalfOrc {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: HalfOrc = rand_utils::rng_from_entropy().gen();
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
