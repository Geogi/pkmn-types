use crate::{Pokemon, SamplePokemon, Type};

pub static SAMPLE_POKEMON: &[SamplePokemon] = &[
    SamplePokemon(
        "Joltik",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Electric),
        },
    ),
    SamplePokemon(
        "Cutiefly",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Heracross",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Larvesta",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Butterfree",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Shedinja",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Paras",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Nincada",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Weedle",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Poison),
        },
    ),
    SamplePokemon(
        "Dottler",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Shuckle",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Rock),
        },
    ),
    SamplePokemon(
        "Forretress",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Surskit",
        Pokemon {
            primary: Type::Bug,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Caterpie",
        Pokemon {
            primary: Type::Bug,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Deino",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Impidimp",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Scraggy",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Houndour",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Murkrow",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Sableye",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Zarude",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Sneasel",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Rattata",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Normal),
        },
    ),
    SamplePokemon(
        "Inkay",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Pawniard",
        Pokemon {
            primary: Type::Dark,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Meowth",
        Pokemon {
            primary: Type::Dark,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Zekrom",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Electric),
        },
    ),
    SamplePokemon(
        "Hakamo-o",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Reshiram",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Dragonite",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Dreepy",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Gible",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Kyurem",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Latias",
        Pokemon {
            primary: Type::Dragon,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Dratini",
        Pokemon {
            primary: Type::Dragon,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Morpeko",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Dracozolt",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Dedenne",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Rotom",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Zapdos",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Rotom",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Rotom",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Rotom",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Helioptile",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Normal),
        },
    ),
    SamplePokemon(
        "Toxel",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Poison),
        },
    ),
    SamplePokemon(
        "Raichu",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Magnemite",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Rotom",
        Pokemon {
            primary: Type::Electric,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Pikachu",
        Pokemon {
            primary: Type::Electric,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Togetic",
        Pokemon {
            primary: Type::Fairy,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Zacian",
        Pokemon {
            primary: Type::Fairy,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Clefairy",
        Pokemon {
            primary: Type::Fairy,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Pangoro",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Hawlucha",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Marshadow",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Crabominable",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Meditite",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Lucario",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Urshifu",
        Pokemon {
            primary: Type::Fighting,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Mankey",
        Pokemon {
            primary: Type::Fighting,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Sizzlipede",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Bug),
        },
    ),
    SamplePokemon(
        "Incineroar",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Turtonator",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Combusken",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Charizard",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Marowak",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Numel",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Litleo",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Normal),
        },
    ),
    SamplePokemon(
        "Darmanitan",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Magcargo",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Rock),
        },
    ),
    SamplePokemon(
        "Heatran",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Volcanion",
        Pokemon {
            primary: Type::Fire,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Charmander",
        Pokemon {
            primary: Type::Fire,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Noibat",
        Pokemon {
            primary: Type::Flying,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Corviknight",
        Pokemon {
            primary: Type::Flying,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Cramorant",
        Pokemon {
            primary: Type::Flying,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Tornadus",
        Pokemon {
            primary: Type::Flying,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Spiritomb",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Giratina",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Mimikyu",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Litwick",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Drifloon",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Phantump",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Sandygast",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Gastly",
        Pokemon {
            primary: Type::Ghost,
            secondary: Some(Type::Poison),
        },
    ),
    SamplePokemon(
        "Misdreavus",
        Pokemon {
            primary: Type::Ghost,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Nuzleaf",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Exeggutor",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Cottonee",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Breloom",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Hoppip",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Decidueye",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Torterra",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Snover",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Bulbasaur",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Poison),
        },
    ),
    SamplePokemon(
        "Exeggcute",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Ferroseed",
        Pokemon {
            primary: Type::Grass,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Tangela",
        Pokemon {
            primary: Type::Grass,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Sandile",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Vibrava",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Stunfisk",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Electric),
        },
    ),
    SamplePokemon(
        "Gligar",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Yamask",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Baltoy",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Rhyhorn",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Rock),
        },
    ),
    SamplePokemon(
        "Diglett",
        Pokemon {
            primary: Type::Ground,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Sandshrew",
        Pokemon {
            primary: Type::Ground,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Snom",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Bug),
        },
    ),
    SamplePokemon(
        "Ninetales",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Darmanitan",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Articuno",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Froslass",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Swinub",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Mr. Mime",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Sandshrew",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Spheal",
        Pokemon {
            primary: Type::Ice,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Vulpix",
        Pokemon {
            primary: Type::Ice,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Drampa",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Jigglypuff",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Meloetta",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Pidgey",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Deerling",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Diggersby",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Girafarig",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Bibarel",
        Pokemon {
            primary: Type::Normal,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Rattata",
        Pokemon {
            primary: Type::Normal,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Skorupi",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Bug),
        },
    ),
    SamplePokemon(
        "Grimer",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Dragalge",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Weezing",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Croagunk",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Salandit",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Zubat",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Nidoqueen",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Skrelp",
        Pokemon {
            primary: Type::Poison,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Ekans",
        Pokemon {
            primary: Type::Poison,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Hoopa",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Rapidash",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Gallade",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Victini",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Natu",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Hoopa",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Celebi",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Indeedee",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Normal),
        },
    ),
    SamplePokemon(
        "Solgaleo",
        Pokemon {
            primary: Type::Psychic,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Abra",
        Pokemon {
            primary: Type::Psychic,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Anorith",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Bug),
        },
    ),
    SamplePokemon(
        "Tyranitar",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Tyrunt",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Geodude",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Electric),
        },
    ),
    SamplePokemon(
        "Carbink",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Terrakion",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Carkol",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Fire),
        },
    ),
    SamplePokemon(
        "Aerodactyl",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Lileep",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Geodude",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Amaura",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Nihilego",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Poison),
        },
    ),
    SamplePokemon(
        "Lunatone",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Shieldon",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Omanyte",
        Pokemon {
            primary: Type::Rock,
            secondary: Some(Type::Water),
        },
    ),
    SamplePokemon(
        "Sudowoodo",
        Pokemon {
            primary: Type::Rock,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Dialga",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Mawile",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Cobalion",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Skarmory",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Honedge",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Steelix",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Beldum",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Aron",
        Pokemon {
            primary: Type::Steel,
            secondary: Some(Type::Rock),
        },
    ),
    SamplePokemon(
        "Meowth",
        Pokemon {
            primary: Type::Steel,
            secondary: None,
        },
    ),
    SamplePokemon(
        "Dewpider",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Bug),
        },
    ),
    SamplePokemon(
        "Carvanha",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Dark),
        },
    ),
    SamplePokemon(
        "Kingdra",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Dragon),
        },
    ),
    SamplePokemon(
        "Chinchou",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Electric),
        },
    ),
    SamplePokemon(
        "Marill",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Fairy),
        },
    ),
    SamplePokemon(
        "Poliwrath",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Fighting),
        },
    ),
    SamplePokemon(
        "Gyarados",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Flying),
        },
    ),
    SamplePokemon(
        "Frillish",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Ghost),
        },
    ),
    SamplePokemon(
        "Lotad",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Grass),
        },
    ),
    SamplePokemon(
        "Wooper",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Ground),
        },
    ),
    SamplePokemon(
        "Dewgong",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Ice),
        },
    ),
    SamplePokemon(
        "Tentacool",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Poison),
        },
    ),
    SamplePokemon(
        "Slowpoke",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Psychic),
        },
    ),
    SamplePokemon(
        "Corsola",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Rock),
        },
    ),
    SamplePokemon(
        "Empoleon",
        Pokemon {
            primary: Type::Water,
            secondary: Some(Type::Steel),
        },
    ),
    SamplePokemon(
        "Squirtle",
        Pokemon {
            primary: Type::Water,
            secondary: None,
        },
    ),
];
