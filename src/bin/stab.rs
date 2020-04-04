use clap::{arg_enum, value_t, App, Arg};
use pkmn_types::{Pokemon, Resistance, Type, ALL_POKEMON_NAIVE};

fn main() {
    let _m = App::new("stab")
        .args(&[
            Arg::with_name("P1_T1")
                .case_insensitive(true)
                .possible_values(&CliType::variants()),
            Arg::with_name("P1_T2")
                .case_insensitive(true)
                .possible_values(&MaybeType::variants()),
            Arg::with_name("P2_T1")
                .case_insensitive(true)
                .possible_values(&CliType::variants()),
            Arg::with_name("P2_T2")
                .case_insensitive(true)
                .possible_values(&MaybeType::variants()),
            Arg::with_name("P3_T1")
                .case_insensitive(true)
                .possible_values(&CliType::variants()),
            Arg::with_name("P3_T2")
                .case_insensitive(true)
                .possible_values(&MaybeType::variants()),
        ])
        .get_matches();
    let p1 = Pokemon {
        primary: value_t!(_m.value_of("P1_T1"), CliType)
            .unwrap_or_else(|_| unreachable!())
            .into(),
        secondary: value_t!(_m.value_of("P1_T2"), MaybeType)
            .unwrap_or_else(|_| unreachable!())
            .into(),
    };
    let p2 = Pokemon {
        primary: value_t!(_m.value_of("P2_T1"), CliType)
            .unwrap_or_else(|_| unreachable!())
            .into(),
        secondary: value_t!(_m.value_of("P2_T2"), MaybeType)
            .unwrap_or_else(|_| unreachable!())
            .into(),
    };
    let p3 = Pokemon {
        primary: value_t!(_m.value_of("P3_T1"), CliType)
            .unwrap_or_else(|_| unreachable!())
            .into(),
        secondary: value_t!(_m.value_of("P3_T2"), MaybeType)
            .unwrap_or_else(|_| unreachable!())
            .into(),
    };
    let mut uncovered = vec![];
    for pokemon in &*ALL_POKEMON_NAIVE {
        if pokemon.resist(p1.primary) < Resistance::Standard {
            continue;
        }
        if let Some(secondary) = p1.secondary {
            if pokemon.resist(secondary) < Resistance::Standard {
                continue;
            }
        }
        if pokemon.resist(p2.primary) < Resistance::Standard {
            continue;
        }
        if let Some(secondary) = p2.secondary {
            if pokemon.resist(secondary) < Resistance::Standard {
                continue;
            }
        }
        if pokemon.resist(p3.primary) < Resistance::Standard {
            continue;
        }
        if let Some(secondary) = p3.secondary {
            if pokemon.resist(secondary) < Resistance::Standard {
                continue;
            }
        }
        uncovered.push(pokemon);
    }
    for pokemon in uncovered {
        println!("{}", pokemon.format_type());
    }
}

arg_enum! {
#[derive(Eq, PartialEq)]
enum MaybeType {
    None,
    Normal,
    Fight,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}
}

impl Into<Option<Type>> for MaybeType {
    fn into(self) -> Option<Type> {
        match self {
            MaybeType::None => None,
            MaybeType::Normal => Some(Type::Normal),
            MaybeType::Fight => Some(Type::Fight),
            MaybeType::Flying => Some(Type::Flying),
            MaybeType::Poison => Some(Type::Poison),
            MaybeType::Ground => Some(Type::Ground),
            MaybeType::Rock => Some(Type::Rock),
            MaybeType::Bug => Some(Type::Bug),
            MaybeType::Ghost => Some(Type::Ghost),
            MaybeType::Steel => Some(Type::Steel),
            MaybeType::Fire => Some(Type::Fire),
            MaybeType::Water => Some(Type::Water),
            MaybeType::Grass => Some(Type::Grass),
            MaybeType::Electric => Some(Type::Electric),
            MaybeType::Psychic => Some(Type::Psychic),
            MaybeType::Ice => Some(Type::Ice),
            MaybeType::Dragon => Some(Type::Dragon),
            MaybeType::Dark => Some(Type::Dark),
            MaybeType::Fairy => Some(Type::Fairy),
        }
    }
}

arg_enum! {
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CliType {
    Normal,
    Fight,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}
}

impl Into<Type> for CliType {
    fn into(self) -> Type {
        match self {
            CliType::Normal => Type::Normal,
            CliType::Fight => Type::Fight,
            CliType::Flying => Type::Flying,
            CliType::Poison => Type::Poison,
            CliType::Ground => Type::Ground,
            CliType::Rock => Type::Rock,
            CliType::Bug => Type::Bug,
            CliType::Ghost => Type::Ghost,
            CliType::Steel => Type::Steel,
            CliType::Fire => Type::Fire,
            CliType::Water => Type::Water,
            CliType::Grass => Type::Grass,
            CliType::Electric => Type::Electric,
            CliType::Psychic => Type::Psychic,
            CliType::Ice => Type::Ice,
            CliType::Dragon => Type::Dragon,
            CliType::Dark => Type::Dark,
            CliType::Fairy => Type::Fairy,
        }
    }
}

trait PokemonExt {
    fn format_type(&self) -> String;
}

impl PokemonExt for Pokemon {
    fn format_type(&self) -> String {
        let mut out = format!("{:?}", self.primary);
        if let Some(secondary) = self.secondary {
            out.push_str(&format!(" {:?}", secondary));
        }
        out
    }
}
