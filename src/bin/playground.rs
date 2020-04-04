use pkmn_types::Pokemon;
use pkmn_types::Type::*;

fn main() {
    let p1 = Pokemon {
        primary: Ground,
        secondary: Some(Fire),
    };
    println!("{:?}", p1.resist(Electric));
}
