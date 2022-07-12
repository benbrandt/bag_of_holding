use std::fmt;

use rand::{
    distributions::Standard,
    prelude::{Distribution, SliceRandom},
    Rng,
};

use crate::NameGenerator;

/// Yuan-ti names have meanings that have been passed down through the
/// generations, although spellings and inflections have changed over time.
///
/// Some yuan-ti add more sibilants to their birth names to create an
/// exaggerated hissing sound, based on one’s personal preference and whether
/// an individual’s anatomy can more easily pronounce the name in this altered
/// form. An adopted name of this sort is recognized as a variant of the birth
/// name, rather than a unique name unto itself. A yuan-ti might refer to itself
/// by its birth name, by its adopted name, or (especially among purebloods) by
/// a name it borrows from the local populace.
#[derive(Debug)]
pub struct YuanTi {
    /// Name of the character
    name: &'static str,
}

impl fmt::Display for YuanTi {
    /// Formatted full name (for character sheet)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Distribution<YuanTi> for Standard {
    /// Generate a new Yuan-ti name.
    #[tracing::instrument(skip(rng))]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> YuanTi {
        YuanTi {
            name: *NAMES.choose(rng).unwrap(),
        }
    }
}

impl NameGenerator for YuanTi {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn name() {
        let name: YuanTi = rand_utils::rng_from_entropy().gen();
        assert!(!name.name.is_empty());
        // Formats full name
        assert_eq!(name.to_string(), format!("{}", name.name));
    }
}

const NAMES: &[&str] = &[
    "Asutali",
    "Dhosun",
    "Eztli",
    "Hessetal",
    "Hitotee",
    "Issahu",
    "Itstli",
    "Jarant",
    "Manuya",
    "Meztli",
    "Nesalli",
    "Otleh",
    "Shalkashlah",
    "Sisava",
    "Sitlali",
    "Soakosh",
    "Ssimalli",
    "Suisatal",
    "Talash",
    "Teoshi",
    "Yaotal",
    "Zihu",
];
