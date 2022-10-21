use alignments::{Alignment, Attitude, Morality};

use crate::{Deity, Domain, Pantheon};

pub const FORGOTTEN_REALMS: &[Deity] = &[
    Deity {
        name: "Adaki",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Tempest],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Cloud"],
        titles: &["goddess of air"],
    },
    Deity {
        name: "Amaunator",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Life, Domain::Light],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Golden sun"],
        titles: &[
            "god of the sun",
            "the Keeper of the Eternal Sun",
            "the Light of Law",
            "the Yellow God",
        ],
    },
    Deity {
        name: "Asmodeus",
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Knowledge, Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Three inverted triangles arranged in a long triangle"],
        titles: &[
            "god of induldence",
            "the Lord of the Ninth",
            "The Cloven",
            "Old Hoof and Horn",
        ],
    },
    Deity {
        name: "Auril",
        alignment: Alignment::new(Attitude::Neutral, Morality::Evil),
        domains: &[Domain::Nature, Domain::Tempest],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Six-pointed snowflake"],
        titles: &[
            "goddess of winter",
            "the Frostmaiden",
            "Lady Frostkiss",
            "Icedawn",
        ],
    },
    Deity {
        name: "Azuth",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Arcana, Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Left hand pointing upward, outlined in fire"],
        titles: &[
            "god of wizardry",
            "The High One",
            "the Lord of Spellcraft",
            "the First Magister",
        ],
    },
    Deity {
        name: "Bane",
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Upright black right hand, thumb and fingers together"],
        titles: &["god of tyranny", "the Black Hand", "the Lord of Darkness"],
    },
    Deity {
        name: "Beshaba",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Black antlers"],
        titles: &[
            "goddess of misfortune",
            "the Maid of Misfortune",
            "Lady Doom",
            "Black Bess",
        ],
    },
    Deity {
        name: "Bhaal",
        alignment: Alignment::new(Attitude::Neutral, Morality::Evil),
        domains: &[Domain::Death],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Skull surrounded by a ring of blood droplets"],
        titles: &["god of murder", "the Lord of Murder"],
    },
    Deity {
        name: "Chauntea",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Life],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Sheaf of grain or a blooming rose over grain"],
        titles: &[
            "goddess of agriculture",
            "the Great Mother",
            "the Grain Goddess",
            "the Earthmother",
        ],
    },
    Deity {
        name: "Cyric",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["White jawless skull on black or purple sunburst"],
        titles: &["god of lies", "the Prince of Lies", "the Dark Sun"],
    },
    Deity {
        name: "Deneir",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Arcana, Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Lit candle above an open eye"],
        titles: &[
            "god of writing",
            "the Lord of All Glyphs and Images",
            "the First Scribe",
            "the Scibe of Oghma",
        ],
    },
    Deity {
        name: "Eldath",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Life, Domain::Nature],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Waterfall plunging into still pool"],
        titles: &[
            "goddess of peace",
            "the Quiet One",
            "the Guardian of Groves",
            "the Mother of Waters",
        ],
    },
    Deity {
        name: "Gond",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Toothed cog with four spokes"],
        titles: &[
            "god of craft",
            "the Wonderbringer",
            "the Inspiration Divine",
            "the Holy Maker of All Things",
        ],
    },
    Deity {
        name: "Grumbar",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Mountain"],
        titles: &["god of earth"],
    },
    Deity {
        name: "Gwaeron Windstrom",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Knowledge, Domain::Nature],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Paw print with a five-pointed star in its center"],
        titles: &[
            "god of tracking",
            "the Mouth of Mielikki",
            "the Master Tracker",
            "the Tracker Never Led Astray",
        ],
    },
    Deity {
        name: "Helm",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Life, Domain::Light],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Staring eye on upright left gauntlet"],
        titles: &[
            "god of watchfulness",
            "the Watcher",
            "He of the Unsleeping Eyes",
            "the Vigilant One",
        ],
    },
    Deity {
        name: "Hoar",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["A coin with a two-faced head"],
        titles: &[
            "god of revenge and retribution",
            "the Doombringer",
            "Poet of Justice",
        ],
    },
    Deity {
        name: "Ilmater",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::Life],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Hands bound at the wrist with red cord"],
        titles: &[
            "god of endurance",
            "the Crying God",
            "the Rack-Broken Lord",
            "He Who Endures",
        ],
    },
    Deity {
        name: "Istishia",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Tempest],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Wave"],
        titles: &["god of water"],
    },
    Deity {
        name: "Jergal",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Knowledge, Domain::Death],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["A skull biting a scroll"],
        titles: &[
            "scribe of the dead",
            "the Final Scribe",
            "the Pitiless One",
            "the Bleak Seneschal",
        ],
    },
    Deity {
        name: "Kelemvor",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Death],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Upright skeletal arm holding balanced scales"],
        titles: &[
            "god of the dead",
            "the Lord of the Dead",
            "the Judge of the Damned",
        ],
    },
    Deity {
        name: "Kossuth",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Light],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Flame"],
        titles: &["god of fire"],
    },
    Deity {
        name: "Lathander",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Life, Domain::Light],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Road traveling into a sunrise"],
        titles: &[
            "god of dawn and renewal",
            "god of birth and renewal",
            "the Morninglord",
            "Inspiration's Dawn",
            "the Rose-and-Gold God",
        ],
    },
    Deity {
        name: "Leira",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Neutral),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Point-down triangle containing a swirl of mist"],
        titles: &[
            "goddess of illusion",
            "the Lady of the Mists",
            "Mistshadow",
            "the Lady of Deception",
        ],
    },
    Deity {
        name: "Lliira",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Good),
        domains: &[Domain::Life],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Triangle of three six-pointed stars"],
        titles: &[
            "goddess of joy",
            "Our Lady of Joy",
            "Joybringer",
            "the Mistress of Revels",
        ],
    },
    Deity {
        name: "Loviatar",
        alignment: Alignment::new(Attitude::Lawful, Morality::Evil),
        domains: &[Domain::Death],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Nine-tailed barbed scourge"],
        titles: &[
            "goddess of pain",
            "the Maiden of Pain",
            "the Scourge Mistress",
            "the Willing Whip",
        ],
    },
    Deity {
        name: "Malar",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Nature],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Clawed paw"],
        titles: &["god of the hunt", "the Beastlord", "the Black-Blooded One"],
    },
    Deity {
        name: "Mask",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Neutral),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Black mask"],
        titles: &[
            "god of thieves",
            "the Lord of Shadows",
            "the Master of All Thieves",
        ],
    },
    Deity {
        name: "Mielikki",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Nature],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Unicorn's head"],
        titles: &[
            "goddess of forests",
            "Our Lady of the Forest",
            "the Forest Queen",
        ],
    },
    Deity {
        name: "Milil",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Light],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Five-stringed harp made of leaves"],
        titles: &[
            "god of poetry and song",
            "the Lord of Song",
            "the One True Hand of All-Wise Oghma",
        ],
    },
    Deity {
        name: "Myrkul",
        alignment: Alignment::new(Attitude::Neutral, Morality::Evil),
        domains: &[Domain::Death],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["White human skull"],
        titles: &[
            "god of death",
            "the Lord of Bones",
            "Old Lord Skull",
            "the Reaper",
        ],
    },
    Deity {
        name: "Mystra",
        alignment: Alignment::new(Attitude::Neutral, Morality::Good),
        domains: &[Domain::Arcana, Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &[
            "Circle of seven stars",
            "nine stars encircling a flowing red mist",
            "a single star",
        ],
        titles: &[
            "goddess of magic",
            "the Lady of Mysteries",
            "Our Lady of Spells",
            "the Mother of All Magic",
        ],
    },
    Deity {
        name: "Oghma",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Blank scroll"],
        titles: &["god of knowledge", "the Binder", "the Lord of Knowledge"],
    },
    Deity {
        name: "The Red Knight",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Red knight lanceboard piece with stars for eyes"],
        titles: &[
            "goddess of strategy",
            "the Lady of Strategy",
            "the Crimson General",
            "the Grandmaster of the Lanceboard",
        ],
    },
    Deity {
        name: "Savras",
        alignment: Alignment::new(Attitude::Lawful, Morality::Neutral),
        domains: &[Domain::Arcana, Domain::Knowledge],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Crystal ball containing many kinds of eyes"],
        titles: &[
            "god of divination and fate",
            "the All-Seeing",
            "the Third Eye",
            "Divination's Lord",
        ],
    },
    Deity {
        name: "Sel\u{fb}ne",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Good),
        domains: &[Domain::Knowledge, Domain::Life],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Pair of eyes surrounded by seven stars"],
        titles: &[
            "goddess of the moon",
            "Our Lady of Silver",
            "the Moonmaiden",
            "the Night White Lady",
        ],
    },
    Deity {
        name: "Shar",
        alignment: Alignment::new(Attitude::Neutral, Morality::Evil),
        domains: &[Domain::Death, Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Black disk encircled with a purple border"],
        titles: &[
            "goddess of darkness and loss",
            "the Mistress of the Night",
            "the Dark Lady",
            "Our Lady of Loss",
        ],
    },
    Deity {
        name: "Silvanus",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Nature],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Oak leaf"],
        titles: &[
            "god of wild nature",
            "Oak Father",
            "the Old Oak",
            "Old Father Tree",
        ],
    },
    Deity {
        name: "Sune",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Good),
        domains: &[Domain::Life, Domain::Light],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Face of a beautiful red-haired woman"],
        titles: &[
            "goddess of love and beauty",
            "Lady Firehair",
            "the Lady of Love",
            "the Princess of Passion",
        ],
    },
    Deity {
        name: "Talona",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Death],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Three teardrops on a triangle"],
        titles: &[
            "goddess of poison and disease",
            "Lady of Poison",
            "Mistress of Disease",
            "the Plague-crone",
        ],
    },
    Deity {
        name: "Talos",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Tempest],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Three lightning bolts radiating from a central point"],
        titles: &["god of storms", "Stormlord", "the Destroyer"],
    },
    Deity {
        name: "Tempus",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Upright flaming sword"],
        titles: &["god of war", "the Foehammer", "the Lord of Battles"],
    },
    Deity {
        name: "Torm",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["White right gauntlet"],
        titles: &[
            "god of courage and self-sacrifice",
            "the Loyal Fury",
            "the True",
            "the Hand of Righteousness",
        ],
    },
    Deity {
        name: "Tymora",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Good),
        domains: &[Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Face-up coin"],
        titles: &["goddess of good fortune", "Lady Luck", "Our Smiling Lady"],
    },
    Deity {
        name: "Tyr",
        alignment: Alignment::new(Attitude::Lawful, Morality::Good),
        domains: &[Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Balanced scales resting on a warhammer"],
        titles: &[
            "god of justice",
            "Grimjaws",
            "the Maimed God",
            "the Evenhanded",
        ],
    },
    Deity {
        name: "Umberlee",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Evil),
        domains: &[Domain::Tempest],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Wave curling left and right"],
        titles: &[
            "goddess of the sea",
            "the Bitch Queen",
            "the Queen of the Depths",
            "the Wavemother",
        ],
    },
    Deity {
        name: "Valkur",
        alignment: Alignment::new(Attitude::Chaotic, Morality::Good),
        domains: &[Domain::Tempest, Domain::War],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["A cloud and three lightning bolts"],
        titles: &["Northlander god of sailors"],
    },
    Deity {
        name: "Waukeen",
        alignment: Alignment::new(Attitude::Neutral, Morality::Neutral),
        domains: &[Domain::Knowledge, Domain::Trickery],
        pantheon: Pantheon::ForgottenRealms,
        symbols: &["Upright coin with Waukeen's profile facing left"],
        titles: &[
            "goddess of trade",
            "Our Lady of Gold",
            "the Coinmaiden",
            "the Merchant's Friend",
        ],
    },
];
