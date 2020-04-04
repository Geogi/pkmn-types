use clap::{arg_enum, value_t, App, Arg};
use pkmn_types::sample::SAMPLE_POKEMON;
use pkmn_types::Resistance::{Regular, Resistant, Weak};
use pkmn_types::{Pokemon, Resistance, SamplePokemon, Type};

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
    let mut resist_stab = vec![];
    let mut not_weak_stab = vec![];
    let mut unsafe_stab = vec![];
    let mut safe_count = 0;
    for sample_pokemon in SAMPLE_POKEMON {
        let SamplePokemon(_, opp) = sample_pokemon;
        let mut stabbers = vec![];
        let mut stabbers_str = vec![];
        let mut stab_safe = false;
        for (n, p) in vec![("P1", p1), ("P2", p2), ("P3", p3)] {
            if let Some(t) = find_stab(&p, opp) {
                stabbers.push(p);
                if let Some(t2) = find_stab(opp, &p) {
                    stabbers_str.push(format!("{} with {:?} (unsafe because {:?})", n, t, t2));
                } else {
                    stab_safe = true;
                    stabbers_str.push(format!("{} with {:?}", n, t));
                }
            }
        }
        if stabbers.is_empty() {
            println!("{}: cannot stab.", sample_pokemon.format_type());
            if vec![p1, p2, p3]
                .iter()
                .all(|p| resist_at_least(p, opp, Resistant))
            {
                resist_stab.push(sample_pokemon);
            } else {
                not_weak_stab.push(sample_pokemon);
            }
        } else {
            println!(
                "{}: {}.",
                sample_pokemon.format_type(),
                stabbers_str.join(", ")
            );
            if stab_safe {
                safe_count += 1;
            } else {
                unsafe_stab.push(sample_pokemon);
            }
        }
    }
    println!();
    println!(
        "{} type variations resist stabbing (reduced damage or less):",
        resist_stab.len()
    );
    for pokemon in &resist_stab {
        println!("{}", pokemon.format_type());
    }
    println!();
    println!(
        "{} type variations are not weak to stabbing (standard damage):",
        not_weak_stab.len()
    );
    for pokemon in &not_weak_stab {
        println!("{}", pokemon.format_type());
    }
    println!();
    println!(
        "{} type variations are unsafe to stab (superior damage but same for the opponent):",
        unsafe_stab.len()
    );
    for pokemon in &unsafe_stab {
        println!("{}", pokemon.format_type());
    }
    println!();
    println!("summary:");
    println!("{} types resist stabbing.", resist_stab.len());
    println!("{} types not weak to stabbing.", not_weak_stab.len());
    println!("{} types unsafe to stab.", unsafe_stab.len());
    println!("{} types safe to stab.", safe_count);
    println!();
}

fn resist_at_least(attacker: &Pokemon, defender: &Pokemon, level: Resistance) -> bool {
    defender.resist(attacker.primary) >= level
        && attacker
            .secondary
            .map(|t| defender.resist(t) >= level)
            .unwrap_or(true)
}

fn find_stab(attacker: &Pokemon, defender: &Pokemon) -> Option<Type> {
    if defender.resist(attacker.primary) <= Weak {
        return Some(attacker.primary);
    } else {
        if let Some(secondary) = attacker.secondary {
            if defender.resist(secondary) <= Weak {
                return Some(secondary);
            }
        }
    }
    None
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
            MaybeType::Fight => Some(Type::Fighting),
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
            CliType::Fight => Type::Fighting,
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

impl PokemonExt for SamplePokemon {
    fn format_type(&self) -> String {
        let mut out = format!("{:?}", self.1.primary);
        if let Some(secondary) = self.1.secondary {
            out.push_str(&format!(" {:?}", secondary));
        }
        out.push_str(&format!(" ({})", self.0));
        out
    }
}
