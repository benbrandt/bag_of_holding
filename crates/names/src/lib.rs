//! # Names
//!
//! Generate names for any race in the D&D multiverse.
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

use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::{
    bugbear::Bugbear, dragonborn::Dragonborn, duergar::Duergar, dwarf::Dwarf, elf::Elf,
    githyanki::Githyanki, githzerai::Githzerai, gnome::Gnome, goblin::Goblin, goliath::Goliath,
    half_elf::HalfElf, half_orc::HalfOrc, halfling::Halfling, hobgoblin::Hobgoblin, human::Human,
    kenku::Kenku, kobold::Kobold, lizardfolk::Lizardfolk, orc::Orc, tabaxi::Tabaxi,
    tiefling::Tiefling, triton::Triton, yuan_ti::YuanTi,
};

mod bugbear;
mod dragonborn;
mod duergar;
mod dwarf;
mod elf;
mod githyanki;
mod githzerai;
mod gnome;
mod goblin;
mod goliath;
mod half_elf;
mod half_orc;
mod halfling;
mod hobgoblin;
mod human;
mod kenku;
mod kobold;
mod lizardfolk;
mod orc;
mod tabaxi;
mod tiefling;
mod triton;
mod yuan_ti;

/// Implements the ability to generate a name for a given race.
/// Can contain whatever information is necessary for a given name
/// (such as clan names, child names, etc)
///
/// Display impl should format the name in a format suitable for a character
/// sheet.
pub trait NameGenerator: fmt::Display + Sized
where
    Standard: Distribution<Self>,
{
}

/// Available race options to choose names from
#[derive(Debug, Deserialize, Display, EnumIter, Serialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Name {
    /// Bugbears only have a single name
    Bugbear,
    /// Dragonborn have personal names given at birth, but they put their clan
    /// names first as a mark of honor. A childhood name or nickname is often used
    /// among clutchmates as a descriptive term or a term of endearment. The name
    /// might recall an event or center on a habit.
    Dragonborn,
    /// Derivation of normal dwarven names, with different clans
    Duergar,
    /// A dwarf’s name belongs to the clan, not to the individual. A dwarf who
    /// misuses or brings shame to a clan name is stripped of the name and
    /// forbidden by law to use any dwarven name in its place.
    Dwarf,
    /// Elves are considered children until they declare themselves adults, some
    /// time after the hundredth birthday, and before this period they are called
    /// by child names.
    ///
    /// On declaring adulthood, an elf selects an adult name, although those who
    /// knew him or her as a youngster might continue to use the child name. Each
    /// elf’s adult name is a unique creation, though it might reflect the names of
    /// respected individuals or other family members. Little distinction exists
    /// between male names and female names; the groupings here reflect only
    /// general tendencies. In addition, every elf bears a family name, typically a
    /// combination of other Elvish words. Some elves traveling among humans
    /// translate their family names into Common, but others retain the Elvish
    /// version.
    Elf,
    /// Githyanki only have a single name
    Githyanki,
    /// Githzerai only have a single name
    Githzerai,
    /// Gnomes love names, and most have half a dozen or so. A gnome’s mother,
    /// father, clan elder, aunts, and uncles each give the gnome a name, and
    /// various nicknames from just about everyone else might or might not stick
    /// over time. Gnome names are typically variants on the names of ancestors or
    /// distant relatives, though some are purely new inventions. When dealing
    /// with humans and others who are “stuffy” about names, a gnome learns to use
    /// no more than three names: a personal name, a clan name, and a nickname,
    /// choosing the one in each category that’s the most fun to say.
    Gnome,
    /// Goblins only have a single name
    Goblin,
    /// Every goliath has three names: a birth name assigned by the newborn’s
    /// mother and father, a nickname assigned by the tribal chief, and a family or
    /// clan name. A birth name is up to three syllables long. Clan names are five
    /// syllables or more and end in a vowel.
    ///
    /// Birth names are rarely linked to gender. Goliaths see females and males as
    /// equal in all things, and they find societies with roles divided by gender
    /// to be puzzling or worthy of mockery. To a goliath, the person who is best
    /// at a job should be the one tasked with doing it.
    ///
    /// A goliath’s nickname is a description that can change on the whim of a
    /// chieftain or tribal elder. It refers to a notable deed, either a success or
    /// failure, committed by the goliath. Goliaths assign and use nicknames with
    /// their friends of other races, and change them to refer to an individual’s
    /// notable deeds.
    ///
    /// Goliaths present all three names when identifying themselves, in the order
    /// of birth name, nickname, and clan name. In casual conversation, they use
    /// their nickname.
    Goliath,
    /// Half-elves use either human or elven naming conventions. As if to emphasize
    /// that they don’t really fit in to either society, half-elves raised among
    /// humans are often given elven names, and those raised among elves often take
    /// human names.
    HalfElf,
    /// Half-orcs usually have names appropriate to the culture in which they were
    /// raised. A half-orc who wants to fit in among humans might trade an orc name
    /// for a human name. Some half-orcs with human names decide to adopt a
    /// guttural orc name because they think it makes them more intimidating.
    HalfOrc,
    /// A halfling has a given name, a family name, and possibly a nickname. Family
    /// names are often nicknames that stuck so tenaciously they have been passed
    /// down through the generations.
    Halfling,
    /// Hobgoblins only have a single name
    Hobgoblin,
    /// Having so much more variety than other cultures, humans as a whole have no
    /// typical names. Some human parents give their children names from other
    /// languages, such as Dwarvish or Elvish (pronounced more or less correctly),
    /// but most parents give names that are linked to their region’s culture or to
    /// the naming traditions of their ancestors.
    Human,
    /// Given that kenku can duplicate any sound, their names are drawn from a
    /// staggering variety of noises and phrases. Kenku names tend to break down
    /// into three categories that make no distinction between male and female
    /// names.
    ///
    /// Kenku thugs, warriors, and toughs adopt noises made by weapons, such as the
    /// clang of a mace against armor or the sound made by a breaking bone. Non-
    /// kenku refer to the kenku by describing this noise. Examples of this type of
    /// name include Smasher, Clanger, Slicer, and Basher.
    ///
    /// Kenku thieves, con artists, and burglars adopt animal noises, typically
    /// those common in urban settings. In this manner, kenku can call out to each
    /// other while those who overhear them mistake them for common animals.
    /// Non-kenku use names that refer to the sound made or the animal a kenku
    /// mimics, such as Rat Scratch, Whistler, Mouser, and Growler.
    ///
    /// Some kenku turn their back on crime to pursue legitimate trades. These
    /// kenku adopt noises made as part of their craft. A sailor duplicates the
    /// sound of a fluttering sail, while a smith mimics the clanging of a hammer
    /// on metal. Non-kenku describe these folk by their trade sounds, such as Sail
    /// Snap, Hammerer, and Cutter.
    Kenku,
    /// Kobold names are derived from the Draconic tongue and usually relate to a
    /// characteristic of the owner, such as scale color, distinctive body parts,
    /// or typical behavior. For example, “Red Foot,” “White Claw,” and “Scurry”
    /// are Common translations of often-used names. A kobold might change its name
    /// when it becomes an adult, or add additional word-syllables after important
    /// events such as completing its first hunt, laying its first egg, or
    /// surviving its first battle.
    Kobold,
    /// Lizardfolk take their names from the Draconic language. They use simple
    /// descriptives granted by the tribe based on an individual’s notable deeds or
    /// actions. For example, Garurt translates as “axe,” a name given to a
    /// lizardfolk warrior who defeated an orc and claimed his foe’s weapon. A
    /// lizardfolk who likes to hide in a stand of reeds before ambushing an animal
    /// might be called Achuak, which means “green” to describe how she blends into
    /// the foliage.
    ///
    /// Lizardfolk make no distinction between male and female in their naming
    /// conventions.
    Lizardfolk,
    /// Orc names don’t always have meaning in the Orc language, and most
    /// noteworthy orcs are given epithets by their tribe mates.
    Orc,
    /// Each tabaxi has a single name, determined by clan and based on a complex
    /// formula that involves astrology, prophecy, clan history, and other esoteric
    /// factors. Tabaxi names can apply to both males and females, and most use
    /// nicknames derived from or inspired by their full names.
    ///
    /// Clan names are usually based on a geographical feature located in or near
    /// the clan’s territory.
    Tabaxi,
    /// Tiefling names fall into three broad categories. Tieflings born into
    /// another culture typically have names reflective of that culture. Some have
    /// names derived from the Infernal language, passed down through generations,
    /// that reflect their fiendish heritage. And some younger tieflings, striving
    /// to find a place in the world, adopt a name that signifies a virtue or other
    /// concept and then try to embody that concept. For some, the chosen name is a
    /// noble quest. For others, it’s a grim destiny.
    Tiefling,
    /// Most triton names have two or three syllables. Male names typically end
    /// with a vowel and the letter s, and female names traditionally end with an n.
    /// Tritons use their home protectorate as a surname, with the name formed by
    /// adding a vowel followed by a “th” to the end of the protectorate’s name.
    Triton,
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
    YuanTi,
}

impl Name {
    /// Generate a new name for the given race
    ///
    /// ```
    /// use names::Name;
    /// use rand::Rng;
    ///
    /// let name = Name::Dwarf.gen(&mut rand::thread_rng());
    /// ```
    #[tracing::instrument(skip(rng))]
    pub fn gen<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        metrics::counter!("names", &[("generator", self.to_string())]).increment(1);

        match self {
            Self::Bugbear => rng.gen::<Bugbear>().to_string(),
            Self::Dragonborn => rng.gen::<Dragonborn>().to_string(),
            Self::Duergar => rng.gen::<Duergar>().to_string(),
            Self::Dwarf => rng.gen::<Dwarf>().to_string(),
            Self::Elf => rng.gen::<Elf>().to_string(),
            Self::Githyanki => rng.gen::<Githyanki>().to_string(),
            Self::Githzerai => rng.gen::<Githzerai>().to_string(),
            Self::Gnome => rng.gen::<Gnome>().to_string(),
            Self::Goblin => rng.gen::<Goblin>().to_string(),
            Self::Goliath => rng.gen::<Goliath>().to_string(),
            Self::HalfElf => rng.gen::<HalfElf>().to_string(),
            Self::HalfOrc => rng.gen::<HalfOrc>().to_string(),
            Self::Halfling => rng.gen::<Halfling>().to_string(),
            Self::Hobgoblin => rng.gen::<Hobgoblin>().to_string(),
            Self::Human => rng.gen::<Human>().to_string(),
            Self::Kenku => rng.gen::<Kenku>().to_string(),
            Self::Kobold => rng.gen::<Kobold>().to_string(),
            Self::Lizardfolk => rng.gen::<Lizardfolk>().to_string(),
            Self::Orc => rng.gen::<Orc>().to_string(),
            Self::Tabaxi => rng.gen::<Tabaxi>().to_string(),
            Self::Tiefling => rng.gen::<Tiefling>().to_string(),
            Self::Triton => rng.gen::<Triton>().to_string(),
            Self::YuanTi => rng.gen::<YuanTi>().to_string(),
        }
    }
}
