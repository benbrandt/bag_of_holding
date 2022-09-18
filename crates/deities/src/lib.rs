//! # Deities and Pantheons
//!
//! Deities, and their related Pantheons, that a character could choose from
//! when choosing favored deities, if any.
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

use rand::{seq::SliceRandom, Rng};
use rand_utils::exp_weight;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

/// In a pantheon, every deity has influence over different aspects of mortal
/// life and civilization, called a deity’s domain. All the domains over which
/// a deity has influence are called the deity’s portfolio. For example, the
/// portfolio of the Greek god Apollo includes the domains of Knowledge, Life,
/// and Light. As a cleric, you choose one aspect of your deity’s portfolio to
/// emphasize, and you are granted powers related to that domain.
///
/// Your choice might correspond to a particular sect dedicated to your deity.
/// Apollo, for example, could be worshiped in one region as Phoebus
/// (“radiant”) Apollo, emphasizing his influence over the Light domain, and in
/// a different place as Apollo Acesius (“healing”), emphasizing his
/// association with the Life domain. Alternatively, your choice of domain
/// could simply be a matter of personal preference, the aspect of the deity
/// that appeals to you most.
#[derive(Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, PartialEq, Serialize)]
pub enum Domain {
    /// Magic is an energy that suffuses the multiverse and that fuels both
    /// destruction and creation. Gods of the Arcana domain know the secrets
    /// and potential of magic intimately. For some of these gods, magical
    /// knowledge is a great responsibility that comes with a special
    /// understanding of the nature of reality. Other gods of Arcana see magic
    /// as pure power, to be used as its wielder sees fit.
    ///
    /// The gods of this domain are often associated with knowledge, as
    /// learning and arcane power tend to go hand-in-hand. In the Realms,
    /// deities of this domain include Azuth and Mystra, as well as Corellon
    /// Larethian of the elven pantheon. In other worlds, this domain includes
    /// Hecate, Math Mathonwy, and Isis; the triple moon gods of Solinari,
    /// Lunitari, and Nuitari of Krynn; and Boccob, Vecna, and Wee Jas of
    /// Greyhawk.
    Arcana,
    /// The Death domain is concerned with the forces that cause death, as well
    /// as the negative energy that gives rise to undead creatures. Deities
    /// such as Chemosh, Myrkul, and Wee Jas are patrons of necromancers, death
    /// knights, liches, mummy lords, and vampires. Gods of the Death domain
    /// also embody murder (Anubis, Bhaal, and Pyremius), pain (Iuz or
    /// Loviatar), disease or poison (Incabulos, Talona, or Morgion), and the
    /// underworld (Hades and Hel).
    Death,
    /// The gods of the forge are patrons of artisans who work with metal, from
    /// a humble blacksmith who keeps a village in horseshoes and plow blades
    /// to the mighty elf artisan whose diamond-tipped arrows of mithral have
    /// felled demon lords. The gods of the forge teach that, with patience and
    /// hard work, even the most intractable metal can be transformed from a
    /// lump of ore to a beautifully wrought object. Clerics of these deities
    /// search for objects lost to the forces of darkness, liberate mines
    /// overrun by orcs, and uncover rare and wondrous materials necessary to c
    /// reate potent magic items. Followers of these gods take great pride in
    /// their work, and they are willing to craft and use heavy armor and
    /// powerful weapons to protect them. Deities of this domain include Gond,
    /// Reorx, Onatar, Moradin, Hephaestus, and Goibhniu.
    Forge,
    /// Gods of the grave watch over the line between life and death. To these
    /// deities, death and the afterlife are a foundational part of the
    /// multiverse. To desecrate the peace of the dead is an abomination.
    /// Deities of the grave include Kelemvor, Wee Jas, the ancestral spirits
    /// of the Undying Court, Hades, Anubis, and Osiris. Followers of these
    /// deities seek to put wandering spirits to rest, destroy the undead, and
    /// ease the suffering of the dying. Their magic also allows them to stave
    /// off death for a time, particularly for a person who still has some
    /// great work to accomplish in the world. This is a delay of death, not a
    /// denial of it, for death will eventually get its due.
    Grave,
    /// The gods of knowledge — including Oghma, Boccob, Gilean, Aureon, and
    /// Thoth — value learning and understanding above all. Some teach that
    /// knowledge is to be gathered and shared in libraries and universities,
    /// or promote the practical knowledge of craft and invention. Some deities
    /// hoard knowledge and keep its secrets to themselves. And some promise
    /// their followers that they will gain tremendous power if they unlock the
    /// secrets of the multiverse. Followers of these gods study esoteric lore,
    /// collect old tomes, delve into the secret places of the earth, and learn
    /// all they can. Some gods of knowledge promote the practical knowledge of
    /// craft and invention, including smith deities like Gond, Reorx, Onatar,
    /// Moradin, Hephaestus, and Goibhniu.
    Knowledge,
    /// The Life domain focuses on the vibrant positive energy — one of the
    /// fundamental forces of the universe — that sustains all life. The gods
    /// of life promote vitality and health through healing the sick and
    /// wounded, caring for those in need, and driving away the forces of death
    /// and undeath. Almost any non-evil deity can claim influence over this
    /// domain, particularly agricultural deities (such as Chauntea, Arawai,
    /// and Demeter), sun gods (such as Lathander, Pelor, and Re-Horakhty),
    /// gods of healing or endurance (such as Ilmater, Mishakal, Apollo, and
    /// Diancecht), and gods of home and community (such as Hestia, Hathor, and
    /// Boldrei).
    Life,
    /// Gods of light — including Helm, Lathander, Pholtus, Branchala, the
    /// Silver Flame, Belenus, Apollo, and Re-Horakhty — promote the ideals of
    /// rebirth and renewal, truth, vigilance, and beauty, often using the
    /// symbol of the sun. Some of these gods are portrayed as the sun itself
    /// or as a charioteer who guides the sun across the sky. Others are
    /// tireless sentinels whose eyes pierce every shadow and see through every
    /// deception. Some are deities of beauty and artistry, who teach that art
    /// is a vehicle for the soul’s improvement. Clerics of a god of light are
    /// enlightened souls infused with radiance and the power of their gods’
    /// discerning vision, charged with chasing away lies and burning away
    /// darkness.
    Light,
    /// Gods of nature are as varied as the natural world itself, from
    /// inscrutable gods of the deep forests (such as Silvanus, Obad-Hai,
    /// Chislev, Balinor, and Pan) to friendly deities associated with
    /// particular springs and groves (such as Eldath). Druids revere nature as
    /// a whole and might serve one of these deities, practicing mysterious
    /// rites and reciting all-but-forgotten prayers in their own secret
    /// tongue. But many of these gods have clerics as well, champions who take
    /// a more active role in advancing the interests of a particular nature
    /// god. These clerics might hunt the evil monstrosities that despoil the
    /// woodlands, bless the harvest of the faithful, or wither the crops of
    /// those who anger their gods.
    Nature,
    /// Gods whose portfolios include the Tempest domain — including Talos,
    /// Umberlee, Kord, Zeboim, the Devourer, Zeus, and Thor — govern storms,
    /// sea, and sky. They include gods of lightning and thunder, gods of
    /// earthquakes, some fire gods, and certain gods of violence, physical
    /// strength, and courage. In some pantheons, a god of this domain rules
    /// over other deities and is known for swift justice delivered by
    /// thunderbolts. In the pantheons of seafaring people, gods of this domain
    /// are ocean deities and the patrons of sailors. Tempest gods send their
    /// clerics to inspire fear in the common folk, either to keep those folk
    /// on the path of righteousness or to encourage them to offer sacrifices
    /// of propitiation to ward off divine wrath.
    Tempest,
    /// Gods of trickery — such as Tymora, Beshaba, Olidammara, the Traveler,
    /// Garl Glittergold, and Loki — are mischief-makers and instigators who
    /// stand as a constant challenge to the accepted order among both gods and
    /// mortals. They’re patrons of thieves, scoundrels, gamblers, rebels, and
    /// liberators. Their clerics are a disruptive force in the world,
    /// puncturing pride, mocking tyrants, stealing from the rich, freeing
    /// captives, and flouting hollow traditions. They prefer subterfuge,
    /// pranks, deception, and theft rather than direct confrontation.
    Trickery,
    /// War has many manifestations. It can make heroes of ordinary people. It
    /// can be desperate and horrific, with acts of cruelty and cowardice
    /// eclipsing instances of excellence and courage. In either case, the gods
    /// of war watch over warriors and reward them for their great deeds. The
    /// clerics of such gods excel in battle, inspiring others to fight the
    /// good fight or offering acts of violence as prayers. Gods of war include
    /// champions of honor and chivalry (such as Torm, Heironeous, and
    /// Kiri-Jolith) as well as gods of destruction and pillage (such as
    /// Erythnul, the Fury, Gruumsh, and Ares) and gods of conquest and
    /// domination (such as Bane, Hextor, and Maglubiyet). Other war gods (such
    /// as Tempus, Nike, and Nuada) take a more neutral stance, promoting war
    /// in all its manifestations and supporting warriors in any circumstance.
    War,
}

impl Domain {
    /// Create a weight based on other factors that may influence a Domain
    /// choice, such as a character's choice of deity
    fn weight(self, influences: &[Self]) -> f64 {
        exp_weight(influences.iter().filter(|&i| i == &self).count())
    }

    /// Generate a domain choice, weighted by influences from other choices on
    /// the character sheet
    ///
    /// # Panics
    ///
    /// Will panic if weighting logic is wrong
    #[tracing::instrument(skip(rng))]
    pub fn gen<R: Rng + ?Sized>(rng: &mut R, influences: &[Self]) -> Self {
        let domain = *Self::iter()
            .collect::<Vec<_>>()
            .choose_weighted(rng, |d| d.weight(influences))
            .unwrap();

        metrics::increment_counter!("domains", &[("domain", domain.to_string()),]);

        domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight() {
        assert!((Domain::Life.weight(&[Domain::Death]) - exp_weight(0)).abs() < f64::EPSILON);
        assert!(
            (Domain::Life.weight(&[Domain::Life, Domain::Death]) - exp_weight(1)).abs()
                < f64::EPSILON
        );
        assert!(
            (Domain::Life.weight(&[Domain::Life, Domain::Life, Domain::Death]) - exp_weight(2))
                .abs()
                < f64::EPSILON
        );
    }
}
